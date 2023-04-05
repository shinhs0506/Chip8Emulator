mod chip8emulator;

use sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::rect::Rect;

use std::ffi::OsStr;
use std::fs;
use std::env;
use std::path::Path;
use std::{thread, time};

use chip8emulator::Chip8Emulator;

const CELL_SIZE:u32 = 18;
const HEIGHT: u32 = 32;
const WIDTH: u32 = 64;

fn main () {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Run: cargo run /path/to/.ch/file");
        return;
    }
    match Path::new(&args[1]).extension().and_then(OsStr::to_str) {
        Some(ext) => {
            if ext != "ch8" {
                println!("Provide a .ch file");
                return;
            }
        },
        _ => {
            println!("Could not parse file");
            return;
        }
    }

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Chip-8 Emulator", WIDTH * CELL_SIZE, HEIGHT * CELL_SIZE).position_centered().build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::WHITE);
    canvas.clear();
    canvas.present();

    let buffer = fs::read(&args[1]).unwrap();
    let mut chip8_emulator = Chip8Emulator::new();
    chip8_emulator.init(&buffer);

    let mut event_pump = sdl_context.event_pump().unwrap();
    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main_loop,
                Event::KeyDown { scancode: Some(scancode), .. } => {
                    if let Some(idx) = scancode2idx(scancode) {
                        chip8_emulator.set_key(idx, true)
                    }
                }
                Event::KeyUp { scancode: Some(scancode), .. } => {
                    if let Some(idx) = scancode2idx(scancode) {
                        chip8_emulator.set_key(idx, false)
                    }
                }
                _ => (),
            }
        }
        
        chip8_emulator.emulate_cycle();
        chip8_emulator.advance_timers();

        if chip8_emulator.should_render() {
            canvas.clear();
            for (idx, val) in chip8_emulator.get_color_array().into_iter().enumerate() {
                if *val == 0x01 {
                    canvas.set_draw_color(Color::WHITE);
                } else {
                    canvas.set_draw_color(Color::BLACK);
                }
                let ul_x = (idx % 64) as u32;
                let ul_y = (idx / 64) as u32;
                canvas.fill_rect(Rect::new((ul_x * CELL_SIZE) as i32, (ul_y * CELL_SIZE) as i32, CELL_SIZE, CELL_SIZE)).unwrap();
            }
            canvas.present();

            chip8_emulator.set_draw_flag(false);
        }

        thread::sleep(time::Duration::from_millis(1));
    }
}

fn scancode2idx(code: Scancode) -> Option<usize> {
    match code {
        Scancode::Num1 => Some(0x1),
        Scancode::Num2 => Some(0x2),
        Scancode::Num3 => Some(0x3),
        Scancode::Num4 => Some(0xC),
        Scancode::Q => Some(0x4),
        Scancode::W => Some(0x5),
        Scancode::E => Some(0x6),
        Scancode::R => Some(0xD),
        Scancode::A => Some(0x7),
        Scancode::S => Some(0x8),
        Scancode::D => Some(0x9),
        Scancode::F => Some(0xE),
        Scancode::Z => Some(0xA),
        Scancode::X => Some(0x0),
        Scancode::C => Some(0xB),
        Scancode::V => Some(0xF),
        _ => None,
    }
}
