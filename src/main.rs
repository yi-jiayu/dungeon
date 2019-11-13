use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadSurface, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, TextureQuery, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::ttf::Font;
use sdl2::video::WindowContext;
use std::path::Path;
use std::time::Instant;

// Path to tileset
const TILESET_PATH: &str = "tileset.png";

// Duration of a single animation frame in milliseconds
const FRAME_DURATION: u128 = 100;

// handle the annoying Rect i32
macro_rules! rect (
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32))
);

macro_rules! keydown (
    ($key:ident) => (Event::KeyDown {
                        keycode: Option::Some(Keycode::$key),
                        ..
                    })
);

macro_rules! keyup (
    ($key:ident) => (Event::KeyUp {
                        keycode: Option::Some(Keycode::$key),
                        ..
                    })
);

#[derive(PartialEq)]
enum Facing {
    Left,
    Right,
}

//noinspection RsWrongLifetimeParametersNumber
struct Character<'a> {
    x: f64,
    y: f64,
    velocity_x: f64,
    velocity_y: f64,
    facing: Facing,
    curr_frame: i32,
    width: u32,
    height: u32,
    sprite_sheet: &'a Texture<'a>,
    sprite_width: u32,
    sprite_height: u32,
    sprite_y: i32,
    idle_sprite_offset: i32,
    moving_sprite_offset: i32,
    num_frames: i32,
}

impl Character<'_> {
    fn pos_x(&self) -> i32 {
        self.x.round() as i32
    }

    fn pos_y(&self) -> i32 {
        self.y.round() as i32
    }

    fn is_moving(&self) -> bool {
        self.velocity_x != 0.0 || self.velocity_y != 0.0
    }

    fn integrate(&mut self, t: u128, dt: u128) {
        self.x += self.velocity_x * dt as f64;
        self.y += self.velocity_y * dt as f64;
        self.curr_frame = (t / FRAME_DURATION) as i32 % self.num_frames;
    }

    fn render_to(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let offset = if self.is_moving() {
            self.moving_sprite_offset
        } else {
            self.idle_sprite_offset
        };
        let src_rect = rect!(
            offset + 16 * self.curr_frame,
            self.sprite_y,
            self.sprite_width,
            self.sprite_height
        );
        let dst_rect = rect!(self.pos_x(), self.pos_y(), self.width, self.height);

        canvas.copy_ex(
            self.sprite_sheet,
            src_rect,
            dst_rect,
            0.,
            None,
            self.facing == Facing::Left,
            false,
        )
    }
}

struct TextRenderer<'a> {
    font: &'a Font<'a, 'a>,
    texture_creator: &'a TextureCreator<WindowContext>,
}

impl TextRenderer<'_> {
    fn render(&self, text: &str, canvas: &mut WindowCanvas) -> Result<(), String> {
        let font_surface = self
            .font
            .render(text)
            .blended(Color::RGBA(255, 0, 0, 255))
            .map_err(|e| e.to_string())?;
        let font_texture = self
            .texture_creator
            .create_texture_from_surface(&font_surface)
            .map_err(|e| e.to_string())?;
        let TextureQuery {
            width: font_width,
            height: font_height,
            ..
        } = font_texture.query();
        let font_rect = rect!(16, 0, font_width, font_height);
        canvas.copy(&font_texture, None, font_rect)
    }
}

pub fn run() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG)?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let mut window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let window_icon = Surface::from_file(Path::new("icon.png"))?;
    window.set_icon(window_icon);
    let window = window;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let sprite_sheet = texture_creator.load_texture(Path::new(TILESET_PATH))?;

    let font = ttf_context.load_font("mago1.ttf", 64)?;

    // Time step in milliseconds
    const DELTA_TIME: u128 = 1;

    // Number of pixels to move per time step
    const VELOCITY: f64 = 0.3;

    let t_0 = Instant::now();
    let mut t: u128 = 0;
    let mut current_time: u128 = 0;
    let mut accumulator: u128 = 0;

    let text_renderer = TextRenderer {
        font: &font,
        texture_creator: &texture_creator,
    };
    let mut character = Character {
        x: 64.0,
        y: 112.0,
        velocity_x: 0.0,
        width: 64,
        height: 112,
        facing: Facing::Right,
        sprite_width: 16,
        sprite_height: 28,
        sprite_y: 4,
        idle_sprite_offset: 128,
        moving_sprite_offset: 192,
        num_frames: 4,
        velocity_y: 0.0,
        curr_frame: 0,
        sprite_sheet: &sprite_sheet,
    };
    let mut character_name = "Elf (F)";

    'mainloop: loop {
        if let Some(event) = sdl_context.event_pump()?.poll_event() {
            match event {
                Event::Quit { .. } | keydown!(Escape) => break 'mainloop,
                keydown!(Right) => {
                    character.velocity_x = VELOCITY;
                    character.facing = Facing::Right;
                }
                keydown!(Left) => {
                    character.velocity_x = -VELOCITY;
                    character.facing = Facing::Left;
                }
                keydown!(Down) => {
                    character.velocity_y = VELOCITY;
                }
                keydown!(Up) => {
                    character.velocity_y = -VELOCITY;
                }
                keyup!(Left) | keyup!(Right) => {
                    character.velocity_x = 0.0;
                }
                keyup!(Up) | keyup!(Down) => {
                    character.velocity_y = 0.0;
                }
                keyup!(Num1) => {
                    character.sprite_y = 4;
                    character_name = "Elf (F)";
                }
                keyup!(Num2) => {
                    character.sprite_y = 36;
                    character_name = "Elf (M)";
                }
                keyup!(Num3) => {
                    character.sprite_y = 68;
                    character_name = "Knight (F)";
                }
                keyup!(Num4) => {
                    character.sprite_y = 100;
                    character_name = "Knight (M)";
                }
                keyup!(Num5) => {
                    character.sprite_y = 132;
                    character_name = "Wizard (F)";
                }
                keyup!(Num6) => {
                    character.sprite_y = 164;
                    character_name = "Wizard (M)";
                }
                keyup!(Num7) => {
                    character.sprite_y = 196;
                    character_name = "Lizard (F)";
                }
                keyup!(Num8) => {
                    character.sprite_y = 228;
                    character_name = "Lizard (M)";
                }
                _ => {}
            }
        }

        let new_time = Instant::now().duration_since(t_0).as_millis();
        let frame_time = new_time - current_time;
        current_time = new_time;
        accumulator += frame_time;

        while accumulator >= DELTA_TIME {
            character.integrate(t, DELTA_TIME);
            accumulator -= DELTA_TIME;
            t += DELTA_TIME;
        }

        canvas.clear();

        text_renderer.render(character_name, &mut canvas)?;
        character.render_to(&mut canvas)?;

        canvas.present();
    }

    Ok(())
}

fn main() -> Result<(), String> {
    run()?;

    Ok(())
}
