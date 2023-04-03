mod chip8emulator;

use sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::rect::{Rect, Point};

use std::fs;

use chip8emulator::Chip8Emulator;

fn main () {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Chip-8 Emulator", 800, 600).position_centered().build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();

    let mut chip8_emulator = Chip8Emulator::new();
    let buffer = fs::read("game.ch8").unwrap();
    chip8_emulator.init(buffer);

    let mut event_pump = sdl_context.event_pump().unwrap();

    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main_loop,
                Event::KeyDown { timestamp: _, window_id: _, keycode: _, scancode, keymod: _, repeat: _ } => {
                    if chip8_emulator.get_wait_flag() {
                        chip8_emulator.set_wait_flag(false);
                        chip8_emulator.resume_cycle();
                    }
                    match scancode {
                        Some(Scancode::Num1) => {
                            chip8_emulator.set_key(0, true)
                        },
                        Some(Scancode::Num2) => {
                            chip8_emulator.set_key(1, true)
                        },
                        Some(Scancode::Num3) => {
                            chip8_emulator.set_key(2, true)
                        },
                        Some(Scancode::Num4) => {
                            chip8_emulator.set_key(3, true)
                        },
                        Some(Scancode::Q) => {
                            chip8_emulator.set_key(4, true)
                        },
                        Some(Scancode::W) => {
                            chip8_emulator.set_key(5, true)
                        },
                        Some(Scancode::E) => {
                            chip8_emulator.set_key(6, true)
                        },
                        Some(Scancode::R) => {
                            chip8_emulator.set_key(7, true)
                        },
                        Some(Scancode::A) => {
                            chip8_emulator.set_key(8, true)
                        },
                        Some(Scancode::S) => {
                            chip8_emulator.set_key(9, true)
                        },
                        Some(Scancode::D) => {
                            chip8_emulator.set_key(10, true)
                        },
                        Some(Scancode::F) => {
                            chip8_emulator.set_key(11, true)
                        },
                        Some(Scancode::Z) => {
                            chip8_emulator.set_key(12, true)
                        },
                        Some(Scancode::X) => {
                            chip8_emulator.set_key(13, true)
                        },
                        Some(Scancode::C) => {
                            chip8_emulator.set_key(14, true)
                        },
                        Some(Scancode::V) => {
                            chip8_emulator.set_key(15, true)
                        },
                        _ => (),
                    }
                }
                Event::KeyUp { timestamp: _, window_id: _, keycode: _, scancode, keymod: _, repeat: _ } => {
                    match scancode {
                        Some(Scancode::Num1) => {
                            chip8_emulator.set_key(0, false)
                        },
                        Some(Scancode::Num2) => {
                            chip8_emulator.set_key(1, false)
                        },
                        Some(Scancode::Num3) => {
                            chip8_emulator.set_key(2, false)
                        },
                        Some(Scancode::Num4) => {
                            chip8_emulator.set_key(3, false)
                        },
                        Some(Scancode::Q) => {
                            chip8_emulator.set_key(4, false)
                        },
                        Some(Scancode::W) => {
                            chip8_emulator.set_key(5, false)
                        },
                        Some(Scancode::E) => {
                            chip8_emulator.set_key(6, false)
                        },
                        Some(Scancode::R) => {
                            chip8_emulator.set_key(7, false)
                        },
                        Some(Scancode::A) => {
                            chip8_emulator.set_key(8, false)
                        },
                        Some(Scancode::S) => {
                            chip8_emulator.set_key(9, false)
                        },
                        Some(Scancode::D) => {
                            chip8_emulator.set_key(10, false)
                        },
                        Some(Scancode::F) => {
                            chip8_emulator.set_key(11, false)
                        },
                        Some(Scancode::Z) => {
                            chip8_emulator.set_key(12, false)
                        },
                        Some(Scancode::X) => {
                            chip8_emulator.set_key(13, false)
                        },
                        Some(Scancode::C) => {
                            chip8_emulator.set_key(14, false)
                        },
                        Some(Scancode::V) => {
                            chip8_emulator.set_key(15, false)
                        },
                        _ => (),
                    }

                }
                _ => (),
            }
        }
        
        if !chip8_emulator.get_wait_flag() {
            chip8_emulator.emulate_cycle();
        }

        draw
        canvas.clear();
        for (idx, val) in chip8_emulator.get_color_array().into_iter().enumerate() {
            if val == 0x01 {
                canvas.set_draw_color(Color::BLACK);
            } else {
                canvas.set_draw_color(Color::WHITE);
            }
            let row: i32 = idx as i32 / 64;
            let col: i32 = idx as i32 % 64;
            let _width: u32 = 5;
            canvas.draw_point(Point::new(row, col)).unwrap();
            // canvas.fill_rect(Rect::new(row, col, width, width)).unwrap();
            canvas.present();
        }
    }
}
