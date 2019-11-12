extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use std::path::Path;
use std::time::Instant;

const TILESET_PATH: &str = "tileset.png";
const FRAME_DURATION: u128 = 100;

#[derive(PartialEq)]
enum Facing {
    Left,
    Right,
}

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

    let t_0 = Instant::now();
    let mut curr_x = 64;
    let mut curr_y = 112;
    let mut velocity_x = 0;
    let mut velocity_y = 0;
    let mut facing = Facing::Right;

    'mainloop: loop {
        match sdl_context.event_pump()?.poll_event() {
            Some(event) => match event {
                Event::Quit { .. } | keydown!(Escape) => break 'mainloop,
                keydown!(Right) => {
                    velocity_x = 1;
                    facing = Facing::Right;
                }
                keydown!(Left) => {
                    velocity_x = -1;
                    facing = Facing::Left;
                }
                keydown!(Down) => {
                    velocity_y = 1;
                }
                keydown!(Up) => {
                    velocity_y = -1;
                }
                keyup!(Left) | keyup!(Right) => {
                    velocity_x = 0;
                }
                keyup!(Up) | keyup!(Down) => {
                    velocity_y = 0;
                }
                _ => {}
            },
            None => {}
        }
        let frame = Instant::now().duration_since(t_0).as_millis() / FRAME_DURATION % 4;

        curr_x += 2 * velocity_x;
        curr_y += 2 * velocity_y;
        let moving = velocity_x > 0 || velocity_y > 0;
        let offset = if moving { 192 } else { 128 };
        let src_rect = rect!(offset + 16 * frame as i32, 4, 16, 28);
        let dst_rect = rect!(curr_x, curr_y, 64, 112);

        canvas.clear();
        canvas.copy(&font_texture, None, font_rect)?;
        canvas.copy_ex(
            &texture,
            src_rect,
            dst_rect,
            0.,
            None,
            facing == Facing::Left,
            false,
        )?;
        canvas.present();
    }

    Ok(())
}

fn main() -> Result<(), String> {
    run()?;

    Ok(())
}
