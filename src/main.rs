extern crate sdl2;

use std::path::Path;
use std::time::Instant;
use sdl2::image::{LoadTexture, InitFlag};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const TILESET_PATH: &str = "tileset.png";
const FRAME_DURATION: u128 = 100;

pub fn run(png: &Path) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG)?;
    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().software().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture(png)?;

    let t_0 = Instant::now();
    let mut curr_x = 64;
    let mut curr_y = 112;

    'mainloop: loop {
        let event = sdl_context.event_pump()?.poll_event();
        match event {
            Some(event) =>
                match event {
                    Event::Quit { .. } |
                    Event::KeyDown { keycode: Option::Some(Keycode::Escape), .. } =>
                        break 'mainloop,
                    Event::KeyDown { keycode: Option::Some(Keycode::Right), .. } => {
                        curr_x += 4;
                    }
                    Event::KeyDown { keycode: Option::Some(Keycode::Left), .. } => {
                        curr_x -= 4;
                    }
                    Event::KeyDown { keycode: Option::Some(Keycode::Down), .. } => {
                        curr_y += 4;
                    }
                    Event::KeyDown { keycode: Option::Some(Keycode::Up), .. } => {
                        curr_y -= 4;
                    }
                    _ => {}
                }
            None => {
                let frame = Instant::now().duration_since(t_0).as_millis() / FRAME_DURATION % 4;

                let src_rect = sdl2::rect::Rect::new(128 + 16 * frame as i32, 4, 16, 28);
                let dst_rect = sdl2::rect::Rect::new(curr_x, curr_y, 64, 112);

                canvas.clear();
                canvas.copy(&texture, src_rect, dst_rect)?;
                canvas.present();
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), String> {
    run(Path::new(TILESET_PATH))?;

    Ok(())
}