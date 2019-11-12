use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{TextureQuery, WindowCanvas, Texture};
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

struct Character {
    x: f64,
    y: f64,
    velocity_x: f64,
    velocity_y: f64,
    facing: Facing,
    curr_frame: i32,
    width: u32,
    height: u32,
    sprite_width: u32,
    sprite_height: u32,
    sprite_y: i32,
    idle_sprite_offset: i32,
    moving_sprite_offset: i32,
    num_frames: i32,
}

impl Character {
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

    fn render_to(&self, texture: &Texture, canvas: &mut WindowCanvas) -> Result<(), String> {
        let offset = if self.is_moving() { self.moving_sprite_offset } else { self.idle_sprite_offset };
        let src_rect = rect!(offset + 16 * self.curr_frame, self.sprite_y, self.sprite_width, self.sprite_height);
        let dst_rect = rect!(self.pos_x(), self.pos_y(), self.width, self.height);

        canvas.copy_ex(
            texture,
            src_rect,
            dst_rect,
            0.,
            None,
            self.facing == Facing::Left,
            false,
        )
    }
}

pub fn run() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG)?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture(Path::new(TILESET_PATH))?;

    let font = ttf_context.load_font("mago1.ttf", 64)?;
    let font_surface = font
        .render("Hello Rust!")
        .blended(Color::RGBA(255, 0, 0, 255))
        .map_err(|e| e.to_string())?;
    let font_texture = texture_creator
        .create_texture_from_surface(&font_surface)
        .map_err(|e| e.to_string())?;
    let TextureQuery {
        width: font_width,
        height: font_height,
        ..
    } = font_texture.query();
    let font_rect = rect!(0, 0, font_width, font_height);

    // Time step in milliseconds
    const DELTA_TIME: u128 = 1;

    // Number of pixels to move per time step
    const VELOCITY: f64 = 0.3;

    let t_0 = Instant::now();
    let mut t: u128 = 0;
    let mut current_time: u128 = 0;
    let mut accumulator: u128 = 0;

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
    };

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
                }
                keyup!(Num2) => {
                    character.sprite_y = 36;
                }
                keyup!(Num3) => {
                    character.sprite_y = 68;
                }
                keyup!(Num4) => {
                    character.sprite_y = 100;
                }
                keyup!(Num5) => {
                    character.sprite_y = 132;
                }
                keyup!(Num6) => {
                    character.sprite_y = 164;
                }
                keyup!(Num7) => {
                    character.sprite_y = 196;
                }
                keyup!(Num8) => {
                    character.sprite_y = 228;
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
        character.render_to(&texture, &mut canvas)?;
        canvas.copy(&font_texture, None, font_rect)?;

        canvas.present();
    }

    Ok(())
}

fn main() -> Result<(), String> {
    run()?;

    Ok(())
}
