mod chip8emulator;

use sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::rect::Rect;

use std::fs;

use chip8emulator::Chip8Emulator;

fn main () {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Chip-8 Emulator", 1200, 900).position_centered().build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::WHITE);
    canvas.clear();
    canvas.present();

    let buffer = fs::read("./game.ch8").unwrap();
    let mut chip8_emulator = Chip8Emulator::new();
    chip8_emulator.init(&buffer);

    let mut event_pump = sdl_context.event_pump().unwrap();
    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main_loop,
                Event::KeyDown { timestamp: _, window_id: _, keycode: _, scancode, keymod: _, repeat: _ } => {
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
                            println!("Q Q Q");
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
        
        chip8_emulator.emulate_cycle();
        chip8_emulator.advance_timers();

        canvas.clear();
        for (idx, val) in chip8_emulator.get_color_array().into_iter().enumerate() {
            if *val == 0x01 {
                canvas.set_draw_color(Color::WHITE);
            } else {
                canvas.set_draw_color(Color::BLACK);
            }
            let width: u32 = 15;
            // canvas.draw_point(Point::new(row_idx as i32, col_idx as i32)).unwrap();
            let ul_y = (idx / 64) as u32;
            let ul_x = (idx % 64) as u32;
            canvas.fill_rect(Rect::new((ul_x * width) as i32, (ul_y * width) as i32, width, width)).unwrap();
        }
        canvas.present();
    }
}
