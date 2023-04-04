use rand::Rng;

const MEMORY_SIZE: usize = 4096;
const NUM_GP_REGISTERS: usize = 16;
const STACK_SIZE: usize = 16;
const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const NUM_FONTS: usize = 16;
const FONT_ADDRESS_OFFSET: usize = 5;
const NUM_KEYS: usize = 16;

pub struct Chip8Emulator {
    memory: Memory,
    registers: Registers,
    stack: Stack,
    graphic : Graphic,
    opcode: OpCode,
    font_set: FontSet,
    draw_flag: bool,
    input: Input,
}

struct Memory {
    ram: [u8; MEMORY_SIZE],
    program_start_address: u16,
}

struct Registers {
    gp_registers: [u8; NUM_GP_REGISTERS],
    i: u16,
    program_counter: u16,
    delay_timer: u8,
    sound_timer: u8,
}

struct Stack {
    stack: [u16; STACK_SIZE],
    stack_pointer: u8,
}

struct Graphic {
    pixels: [u8; WIDTH * HEIGHT],
}

struct OpCode {
    opcode: u16,
}

struct FontSet {
    font_set: [u8; NUM_FONTS * FONT_ADDRESS_OFFSET],
}

struct Input {
    pressed: [bool; NUM_KEYS],
}

impl Chip8Emulator {
    pub fn new() -> Self {
        Self {
            memory: Memory {
                ram: [0; MEMORY_SIZE],
                program_start_address: 0x200,
            },
            registers: Registers {
                gp_registers: [0; NUM_GP_REGISTERS],
                i: 0,
                program_counter: 0x200,
                delay_timer: 0,
                sound_timer: 0,
            },
            stack: Stack {
                stack: [0; STACK_SIZE],
                stack_pointer: 0,
            },
            graphic: Graphic {
                pixels: [0; WIDTH * HEIGHT],
            },
            opcode: OpCode {
                opcode: 0,
            },
            font_set: FontSet {
                font_set: [
                    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
                    0x20, 0x60, 0x20, 0x20, 0x70, // 1
                    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
                    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
                    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
                    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
                    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
                    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
                    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
                    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
                    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
                    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
                    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
                    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
                    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
                    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
                ],
            },
            draw_flag: false,
            input: Input {
                pressed: [false; NUM_KEYS],
            },
        }
    }

    pub fn init(&mut self, buffer: &[u8]) {
        // load fontset
        self.memory.ram[..(NUM_FONTS * FONT_ADDRESS_OFFSET)].copy_from_slice(&self.font_set.font_set);

        // laod program into memory
        let program_start_memory_address = self.memory.program_start_address as usize;
        let program_end_memory_adderess = program_start_memory_address as usize + buffer.len();
        self.memory.ram[program_start_memory_address..program_end_memory_adderess].copy_from_slice(buffer);
    }

    pub fn emulate_cycle(&mut self) {
        self.opcode.opcode = (self.memory.ram[self.registers.program_counter as usize] as u16) << 8 | self.memory.ram[self.registers.program_counter as usize + 1] as u16; 

        match self.opcode.opcode & 0xF000 {
            0x0000 => {
                match self.opcode.opcode {
                    0x00E0 => {
                        for x in self.graphic.pixels.iter_mut() {
                            *x = 0;
                        }
                        self.draw_flag = true;
                        self.registers.program_counter += 2;
                    },
                    0x00EE => {
                        self.stack.stack_pointer -= 1;
                        self.registers.program_counter = self.stack.stack[self.stack.stack_pointer as usize];
                        self.registers.program_counter += 2;
                    },
                    // 0NNN
                    _=> {
                        println!("what? {}", self.opcode.opcode);
                    }
                }
            }
            // 1NNN
            0x1000 => {
                self.registers.program_counter = self.opcode.opcode & 0x0FFF;
            },
            // 2NNN
            0x2000 => {
                self.stack.stack[self.stack.stack_pointer as usize] = self.registers.program_counter;
                self.stack.stack_pointer += 1;
                self.registers.program_counter = self.opcode.opcode & 0x0FFF;
            },
            // 3XNN
            0x3000 => {
                let val = self.opcode.opcode & 0x00FF;
                let gp_register_index = (self.opcode.opcode & 0x0F00) >> 8;
                if self.registers.gp_registers[gp_register_index as usize] == val as u8 {
                    self.registers.program_counter += 4;
                } else {
                    self.registers.program_counter += 2;
                }
            },
            // 4XNN
            0x4000 => {
                let val = self.opcode.opcode & 0x00FF;
                let gp_register_index = (self.opcode.opcode & 0x0F00) >> 8;
                if self.registers.gp_registers[gp_register_index as usize] != val as u8 {
                    self.registers.program_counter += 4;
                } else {
                    self.registers.program_counter += 2;
                }
            },
            // 5XY0
            0x5000 => {
                let gp_register_index_x = (self.opcode.opcode & 0x0F00) >> 8;
                let gp_register_index_y = (self.opcode.opcode & 0x00F0) >> 4;
                if self.registers.gp_registers[gp_register_index_x as usize] == self.registers.gp_registers[gp_register_index_y as usize] {
                    self.registers.program_counter += 4;
                } else {
                    self.registers.program_counter += 2;
                }
            },
            // 6XNN
            0x6000 => {
                let val = self.opcode.opcode & 0x00FF;
                let gp_register_index = (self.opcode.opcode & 0x0F00) >> 8;
                self.registers.gp_registers[gp_register_index as usize] = val as u8;
                self.registers.program_counter += 2;
            },
            // 7XNN
            0x7000 => {
                let val = self.opcode.opcode & 0x00FF;
                let gp_register_index = (self.opcode.opcode & 0x0F00) >> 8;
                let res = self.registers.gp_registers[gp_register_index as usize].wrapping_add(val as u8);
                self.registers.gp_registers[gp_register_index as usize] = res;
                self.registers.program_counter += 2;
            },
            0x8000 => {
                let gp_register_index_x = (self.opcode.opcode & 0x0F00) >> 8;
                let gp_register_index_y = (self.opcode.opcode & 0x00F0) >> 4;
                match self.opcode.opcode & 0x000F {
                    // 0x8XY0
                    0x0000 => {
                        self.registers.gp_registers[gp_register_index_x as usize] = self.registers.gp_registers[gp_register_index_y as usize];
                        self.registers.program_counter += 2;
                    },
                    // 0x8XY1
                    0x0001 => {
                        self.registers.gp_registers[gp_register_index_x as usize] |= self.registers.gp_registers[gp_register_index_y as usize];
                        self.registers.program_counter += 2;
                    },
                    // 0x8XY2
                    0x0002 => {
                        self.registers.gp_registers[gp_register_index_x as usize] &= self.registers.gp_registers[gp_register_index_y as usize];
                        self.registers.program_counter += 2;
                    },
                    // 0x8XY3
                    0x0003 => {
                        self.registers.gp_registers[gp_register_index_x as usize] ^= self.registers.gp_registers[gp_register_index_y as usize];
                        self.registers.program_counter += 2;
                    },
                    // 0x8XY4
                    0x0004 => {
                        let val_x = self.registers.gp_registers[gp_register_index_x as usize];
                        let val_y = self.registers.gp_registers[gp_register_index_y as usize];
                        let (res, overflow) = val_x.overflowing_add(val_y);
                        self.registers.gp_registers[15] = if overflow { 1 } else { 0 };
                        self.registers.gp_registers[gp_register_index_x as usize] = res;
                        self.registers.program_counter += 2;
                    },
                    // 0x8XY5
                    0x0005 => {
                        let val_x = self.registers.gp_registers[gp_register_index_x as usize];
                        let val_y = self.registers.gp_registers[gp_register_index_y as usize];
                        let (res, borrow) = val_x.overflowing_sub(val_y);
                        self.registers.gp_registers[15] = if borrow { 0 } else { 1 };
                        self.registers.gp_registers[gp_register_index_x as usize] = res;
                        self.registers.program_counter += 2;
                    },
                    // 0x8XY6
                    0x0006 => {
                        let val_x = self.registers.gp_registers[gp_register_index_x as usize];
                        self.registers.gp_registers[15] = val_x & 0x01;
                        self.registers.gp_registers[gp_register_index_x as usize] = val_x >> 1;
                        self.registers.program_counter += 2;
                    },
                    // 0x8XY7
                    0x0007 => {
                        let val_x = self.registers.gp_registers[gp_register_index_x as usize];
                        let val_y = self.registers.gp_registers[gp_register_index_y as usize];
                        let (res, borrow) = val_y.overflowing_sub(val_x);
                        self.registers.gp_registers[15] = if borrow { 0 } else { 1 };
                        self.registers.gp_registers[gp_register_index_x as usize] = res;
                        self.registers.program_counter += 2;
                    },
                    // 0x8XYE
                    0x000E => {
                        let val_x = self.registers.gp_registers[gp_register_index_x as usize];
                        self.registers.gp_registers[15] = (val_x >> 7) & 0x1;
                        self.registers.gp_registers[gp_register_index_x as usize] = val_x << 1;
                        self.registers.program_counter += 2;
                    },
                    _ => println!("Unknown opcode")
                }
            },
            // 0x9XY0
            0x9000 => {
                let gp_register_index_x = (self.opcode.opcode & 0x0F00) >> 8;
                let gp_register_index_y = (self.opcode.opcode & 0x00F0) >> 4;
                if self.registers.gp_registers[gp_register_index_x as usize] != self.registers.gp_registers[gp_register_index_y as usize] {
                    self.registers.program_counter += 4;
                } else {
                    self.registers.program_counter += 2;
                }
            },
            // 0xANNN
            0xA000 => {
                self.registers.i = self.opcode.opcode & 0x0FFF;
                self.registers.program_counter += 2;
            },
            // 0xBNNN
            0xB000 => {
                self.registers.program_counter = self.registers.gp_registers[0 as usize] as u16 + (self.opcode.opcode & 0x0FFF);
            },
            // 0xCXNN
            0xC000 => {
                let gp_register_index_x = (self.opcode.opcode & 0x0F00) >> 8;
                let val = self.opcode.opcode & 0x00FF;
                let rand: u8 = rand::thread_rng().gen();
                self.registers.gp_registers[gp_register_index_x as usize] = rand & val as u8;
                self.registers.program_counter += 2;
            },
            // 0xDXYN
            0xD000 => {
                let gp_register_index_x = (self.opcode.opcode & 0x0F00) >> 8;
                let gp_register_index_y = (self.opcode.opcode & 0x00F0) >> 4;
                let x_val = self.registers.gp_registers[gp_register_index_x as usize];
                let y_val = self.registers.gp_registers[gp_register_index_y as usize];
                let n = self.opcode.opcode & 0x000F;
                self.registers.gp_registers[15] = 0;

                for row in 0..n {
                    let pixel = self.memory.ram[(self.registers.i + row) as usize];
                    for col in 0..8 {
                        if pixel & (0x80 >> col) != 0 {
                            let r = (y_val as u16 + row) as usize % 32;
                            let c = (x_val as u16 + col) as usize % 64;
                            if self.graphic.pixels[(c + 64 * r)] == 1 {
                                self.registers.gp_registers[15] = 1;
                            }
                            self.graphic.pixels[(c + 64 * r)] ^= 1; 
                        }
                    }
                } 

                self.draw_flag = true;
                self.registers.program_counter += 2;
            },
            0xE000 => {
                match self.opcode.opcode & 0x00FF {
                    // 0xEX9E
                    0x009E => {
                        let gp_register_index_x = (self.opcode.opcode & 0x0F00) >> 8;
                        let x_val = self.registers.gp_registers[gp_register_index_x as usize];
                        if self.input.pressed[x_val as usize] {
                            self.registers.program_counter += 4;
                        } else {
                            self.registers.program_counter += 2;
                        }
                    },
                    // 0xEXA1
                    0x00A1 => {
                        let gp_register_index_x = (self.opcode.opcode & 0x0F00) >> 8;
                        let x_val = self.registers.gp_registers[gp_register_index_x as usize];
                        if !self.input.pressed[x_val as usize] {
                            self.registers.program_counter += 4;
                        } else {
                            self.registers.program_counter += 2;
                        }

                    },
                    _ => println!("Wrong opcode")
                }
            },
            0xF000 => {
                let gp_register_index_x = (self.opcode.opcode & 0x0F00) >> 8;
                match self.opcode.opcode & 0x00FF {
                    // 0xFX07
                    0x0007 => {
                        self.registers.gp_registers[gp_register_index_x as usize] = self.registers.delay_timer;
                        self.registers.program_counter += 2;
                    },
                    // 0xFX0A
                    0x000A => {
                        let mut pressed = false;
                        for (idx, x) in self.input.pressed.iter().enumerate() {
                            if *x {
                                self.registers.gp_registers[gp_register_index_x as usize] = idx as u8;
                                pressed = true;
                                break;
                            }
                        }

                        if pressed {
                            self.registers.program_counter += 2;
                        }
                    },
                    // 0xFX15
                    0x0015 => {
                        self.registers.delay_timer = self.registers.gp_registers[gp_register_index_x as usize];
                        self.registers.program_counter += 2;
                    },
                    // 0xFX18
                    0x0018 => {
                        self.registers.sound_timer = self.registers.gp_registers[gp_register_index_x as usize];
                        self.registers.program_counter += 2;
                    },
                    // 0xFX1E
                    0x001E => {
                        let val = self.registers.gp_registers[gp_register_index_x as usize] as u16;
                        let res = self.registers.i.wrapping_add(val);
                        self.registers.i = res;
                        self.registers.program_counter += 2;
                    },
                    // 0xFX29
                    0x0029 => {
                        let x_val = self.registers.gp_registers[gp_register_index_x as usize] as u16;
                        self.registers.i = x_val * 5;
                        self.registers.program_counter += 2;
                    },
                    // 0xFX33
                    0x0033 => {
                        let val = self.registers.gp_registers[((self.opcode.opcode & 0x0F00) >> 8) as usize] as f32;
                        self.memory.ram[(self.registers.i) as usize] = ((val / 100.0) % 10.0).floor() as u8;
                        self.memory.ram[(self.registers.i + 1) as usize] = ((val / 10.0) % 10.0).floor() as u8;
                        self.memory.ram[(self.registers.i + 2) as usize] = (val % 10.0).floor() as u8;
                        self.registers.program_counter += 2;
                    },
                    // 0xFX55
                    0x0055 => {
                        for idx in 0..=gp_register_index_x {
                            self.memory.ram[self.registers.i as usize + idx as usize] = self.registers.gp_registers[idx as usize];
                        }
                        self.registers.program_counter += 2;
                    },
                    // 0xFX65
                    0x0065 => {
                        for idx in 0..=gp_register_index_x {
                             self.registers.gp_registers[idx as usize] = self.memory.ram[self.registers.i as usize + idx as usize];
                        }
                        self.registers.program_counter += 2;
                    },
                    _ => println!("Wrong opcode")
                }

            }
            _ => println!("Wrong opcode")
        }

        if self.registers.delay_timer > 0 {
            self.registers.delay_timer -= 1;
        }

        if self.registers.sound_timer > 0 {
            if self.registers.sound_timer == 1 {
                // BEEP
            }
            self.registers.sound_timer -= 1;
        }
    }

    pub fn set_key(&mut self, index: usize, pressed: bool) {
        self.input.pressed[index] = pressed;
    }

    pub fn get_color_array(&self) -> &[u8] {
        &self.graphic.pixels
    }
}
