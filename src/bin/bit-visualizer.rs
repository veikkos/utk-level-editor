use std::fs;
use std::fs::File;
use std::io::Read;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};
use std::time::Duration;
use sdl2::rect::Point;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window_w = 800;
    let window_h = 600;
    let window = video_subsystem.window("rust-sdl2 demo", window_w, window_h)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_scale(8.0, 8.0).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let data = get_file_as_bit_vec("TETRIS.FN2");
    let size = data.len();
    assert_eq!(size % 8, 0);

    println!("File size: {} bits, {} bytes", size, size / 8);
    // for i in 0..1024 {
    //     if i > 0 && i % (6 * 8) == 0 {
    //         println!();
    //     } else if i > 0 && (i % 8) == 0 {
    //         print!(" ");
    //     }
    //     print!("{}", if data[i] { 1 } else { 0 });
    // }
    // return;

    let mut width = 256;
    let mut skip = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    if width < 800 { width += 1; }
                    println!("Width: {}", width)
                }
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    if width > 0 { width -= 1; }
                    println!("Width: {}", width)
                }
                Event::KeyDown { keycode: Some(Keycode::Left), keymod, .. } => {
                    let add = if keymod == Mod::RSHIFTMOD { 256 } else { 1 };
                    if add > skip { skip = 0 } else { skip -= add };
                    println!("Skip: {}", skip)
                }
                Event::KeyDown { keycode: Some(Keycode::Right), keymod, .. } => {
                    let add = if keymod == Mod::RSHIFTMOD { 256 } else { 1 };
                    skip += add;
                    println!("Skip: {}", skip)
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 255, 0));
        canvas.clear();

        let mut i = skip;
        let mut y = 0;
        'draw: loop {
            for x in 0..width {
                let bit = data[i];
                canvas.set_draw_color(if bit { Color::RGB(255, 0, 0) } else { Color::RGB(0, 0, 0) });
                canvas.draw_point(Point::new(x, y)).unwrap();
                i += 1;
                if i >= size { break 'draw; }
            }
            y += 1;
        }

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn get_file_as_bit_vec(filename: &str) -> Vec<bool> {
    let mut f = File::open(filename).expect("no file found");
    let metadata = fs::metadata(filename).expect("unable to read metadata");
    let size = metadata.len() as usize;
    let mut buffer = vec![0; size];
    f.read(&mut buffer).expect("buffer overflow");

    let mut bits = vec![false; size * 8];
    for i in 0..size {
        let mut cell = buffer[i];
        for j in 0..8 {
            bits[i * 8 + j] = (cell & 0x80) != 0;
            cell = cell << 1;
        }
    }

    bits
}
