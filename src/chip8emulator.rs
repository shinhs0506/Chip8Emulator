use rand::Rng;
use std::collections::HashMap;

pub struct Chip8Emulator {
    memory: Memory,
    registers: Registers,
    stack: Stack,
    graphic : Graphic,
    opcode: OpCode,
    font_set: FontSet,
    draw_flag: bool,
    input: Input,
    is_waiting_for_key: bool,
}

struct Memory {
    ram: Vec<u8>,
    size: usize,
    program_start_address: u16,
}

struct Registers {
    gp_registers: Vec<u8>,
    num_gp_registers: usize,
    i: u16,
    program_counter: u16,
    delay_timer: u8,
    sound_timer: u8,
}

struct Stack {
    stack: Vec<u16>,
    size: usize,
    stack_pointer: u8,
}

struct Graphic {
    color_array: Vec<u8>,
    size: usize,
}

struct OpCode {
    opcode: u16,
}

struct FontSet {
    font_set: Vec<u8>,
    size: usize
}

struct Input {
    pressed: Vec<bool>,
    num_keys: usize,
    val_to_idx: HashMap<u8, usize>
}

impl Chip8Emulator {
    pub fn new() -> Self {
        let chip8_emulator = Chip8Emulator {
            memory: Memory {
                size: 4096,
                ram: [0; 4096].to_vec(),
                program_start_address: 0x200,
            },
            registers: Registers {
                gp_registers: [0; 16].to_vec(),
                num_gp_registers: 16,
                i: 0,
                program_counter: 0x200,
                delay_timer: 0,
                sound_timer: 0,
            },
            stack: Stack {
                stack: [0; 16].to_vec(),
                size: 16,
                stack_pointer: 0,
            },
            graphic: Graphic {
                color_array: [0; 64 * 32].to_vec(),
                size: 64 * 32,
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
                ].to_vec(),
                size: 5 * 16
            },
            draw_flag: false,
            input: Input {
                pressed: [false; 16].to_vec(),
                num_keys: 16,
                val_to_idx: HashMap::from([
                                          (0x01, 0),
                                          (0x02, 1),
                                          (0x03, 2),
                                          (0x0C, 3),
                                          (0x04, 4),
                                          (0x05, 5),
                                          (0x06, 6),
                                          (0x0D, 7),
                                          (0x07, 8),
                                          (0x08, 9),
                                          (0x09, 10),
                                          (0x0E, 11),
                                          (0x0A, 12),
                                          (0x00, 13),
                                          (0x0B, 14),
                                          (0x0F, 15),])
            },
            is_waiting_for_key: false,
        };
        chip8_emulator
    }

    pub fn init(&mut self, buffer: Vec<u8>) {
        // load fontset
        for (i, val) in self.font_set.font_set.iter().enumerate() {
            self.memory.ram[i] = *val;
        }

        // laod program into memory
        for (i, val) in buffer.iter().enumerate() {
            self.memory.ram[(self.memory.program_start_address + i as u16) as usize] = *val;
        }
    }

    pub fn emulate_cycle(&mut self) {
        self.opcode.opcode = (self.memory.ram[self.registers.program_counter as usize] as u16) << 8 | self.memory.ram[self.registers.program_counter as usize + 1] as u16; 

        match self.opcode.opcode & 0xF000 {
            0x0000 => {
                match self.opcode.opcode {
                    0x00E0 => {
                        for x in self.graphic.color_array.iter_mut() {
                            *x = 0;
                        }
                        self.draw_flag = true;
                        self.registers.program_counter += 2;
                    },
                    0x00EE => {
                        self.stack.stack_pointer -= 1;
                        self.registers.program_counter = self.stack.stack[self.stack.stack_pointer as usize];
                    },
                    // 0NNN
                    _=> {

                    }
                }
            }
            // 1NNet
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
                let gp_register_index = self.opcode.opcode & 0x0300;
                if self.registers.gp_registers[gp_register_index as usize] as u16 == val {
                    self.registers.program_counter += 4;
                }
            },
            // 4XNN
            0x4000 => {
                let val = self.opcode.opcode & 0x00FF;
                let gp_register_index = self.opcode.opcode & 0x0300;
                if self.registers.gp_registers[gp_register_index as usize] as u16 != val {
                    self.registers.program_counter += 4;
                }
            },
            // 5XY0
            0x5000 => {
                let gp_register_index_x = self.opcode.opcode & 0x0300;
                let gp_register_index_y = self.opcode.opcode & 0x0C00;
                if self.registers.gp_registers[gp_register_index_x as usize] == self.registers.gp_registers[gp_register_index_y as usize] {
                    self.registers.program_counter += 4;
                }
            },
            // 6XNN
            0x6000 => {
                let val = self.opcode.opcode & 0x00FF;
                let gp_register_index = self.opcode.opcode & 0x0300;
                self.registers.gp_registers[gp_register_index as usize] = val as u8;
                self.registers.program_counter += 2;
            },
            // 7XNN
            0x7000 => {
                let val = self.opcode.opcode & 0x00FF;
                let gp_register_index = self.opcode.opcode & 0x0300;
                self.registers.gp_registers[gp_register_index as usize] += val as u8;
                self.registers.program_counter += 2;
            },
            0x8000 => {
                let gp_register_index_x = self.opcode.opcode & 0x0300;
                let gp_register_index_y = self.opcode.opcode & 0x00C0;
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
                        let should_set_carry = val_x > 0xFF - val_y;
                        self.registers.gp_registers[15] = if should_set_carry { 1 } else { 0 };
                        self.registers.gp_registers[gp_register_index_x as usize] = val_x + val_y;
                        self.registers.program_counter += 2;
                    },
                    // 0x8XY5
                    0x0005 => {
                        let val_x = self.registers.gp_registers[gp_register_index_x as usize];
                        let val_y = self.registers.gp_registers[gp_register_index_y as usize];
                        let should_set_carry = val_x > val_y;
                        self.registers.gp_registers[15] = if should_set_carry { 1 } else { 0 };
                        self.registers.gp_registers[gp_register_index_x as usize] = val_x - val_y;
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
                        let should_set_carry = val_y > val_x;
                        self.registers.gp_registers[15] = if should_set_carry { 1 } else { 0 };
                        self.registers.gp_registers[gp_register_index_x as usize] = val_y - val_x;
                        self.registers.program_counter += 2;
                    },
                    // 0x8XYE
                    0x000E => {
                        let val_x = self.registers.gp_registers[gp_register_index_x as usize];
                        self.registers.gp_registers[15] = val_x & 0x80;
                        self.registers.gp_registers[gp_register_index_x as usize] = val_x << 1;
                        self.registers.program_counter += 2;
                    },
                    _ => println!("Unknown opcode")
                }
            },
            // 0x9XY0
            0x9000 => {
                let gp_register_index_x = self.opcode.opcode & 0x0300;
                let gp_register_index_y = self.opcode.opcode & 0x00C0;
                if self.registers.gp_registers[gp_register_index_x as usize] != self.registers.gp_registers[gp_register_index_y as usize] {
                    self.registers.program_counter += 4;
                }
            },
            // 0xANNN
            0xA000 => {
                self.registers.i = self.opcode.opcode & 0x0FFF;
                self.registers.program_counter += 2;
            },
            // 0xBNNN
            0xB000 => {
                self.registers.program_counter = self.registers.gp_registers[0 as usize] as u16 + self.opcode.opcode & 0x0FFF;
                self.registers.program_counter += 2;
            },
            // 0xCXNN
            0xC000 => {
                self.registers.gp_registers[(self.opcode.opcode & 0x0C00) as usize] = rand::thread_rng().gen_range(0..255) & (self.opcode.opcode & 0x00FF) as u8;
                self.registers.program_counter += 2;
            },
            // 0xDXYN
            0xD000 => {
                let x_val = self.registers.gp_registers[(self.opcode.opcode & 0x0300) as usize];
                let y_val = self.registers.gp_registers[(self.opcode.opcode & 0x00C0) as usize];
                let n = self.opcode.opcode & 0x000F;
                self.registers.gp_registers[15] = 0;

                for col in 0..n {
                    let pixel = self.memory.ram[self.registers.i as usize];
                    for row in 0..8 {
                        if pixel & (0x80 >> row) != 0 {
                            if self.graphic.color_array[(x_val + row + ((y_val + col as u8) * 64)) as usize] == 1 {
                                self.registers.gp_registers[15] = 1;
                            }
                            self.graphic.color_array[(x_val + row + ((y_val + col as u8) * 64)) as usize] ^= 1; 
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
                        let x_val = self.registers.gp_registers[(self.opcode.opcode & 0x0300) as usize];
                        if self.input.pressed[*self.input.val_to_idx.get(&x_val).unwrap()] {
                            self.registers.program_counter += 4;
                        }
                    },
                    // 0xEXA1
                    0x00A1 => {
                        let x_val = self.registers.gp_registers[(self.opcode.opcode & 0x0300) as usize];
                        if !self.input.pressed[*self.input.val_to_idx.get(&x_val).unwrap()] {
                            self.registers.program_counter += 4;
                        }

                    },
                    _ => println!("Wrong opcode")
                }
            },
            0xF000 => {
                let gp_register_index = self.opcode.opcode & 0x0300;
                match self.opcode.opcode & 0x00FF {
                    // 0xFX07
                    0x0007 => {
                        self.registers.gp_registers[gp_register_index as usize] = self.registers.delay_timer;
                        self.registers.program_counter += 2;
                    },
                    // 0xFX0A
                    0x000A => {
                        self.is_waiting_for_key = true;
                    },
                    // 0xFX15
                    0x0015 => {
                        self.registers.delay_timer = self.registers.gp_registers[gp_register_index as usize];
                        self.registers.program_counter += 2;
                    },
                    // 0xFX18
                    0x0018 => {
                        self.registers.sound_timer = self.registers.gp_registers[gp_register_index as usize];
                        self.registers.program_counter += 2;
                    },
                    // 0xFX1E
                    0x001E => {
                        self.registers.i += self.registers.gp_registers[gp_register_index as usize] as u16;
                        self.registers.program_counter += 2;
                    },
                    // 0xFX29
                    0x0029 => {
                        let x_val = self.registers.gp_registers[(self.opcode.opcode & 0x0F00) as usize];
                        self.registers.i = self.memory.ram[(x_val * 5) as usize] as u16;
                        self.registers.program_counter += 2;
                    },
                    // 0xFX33
                    0x0033 => {
                        let mut val = self.opcode.opcode & 0x0F00;
                        let one_digit = val % 10;
                        self.memory.ram[(self.registers.i + 2) as usize] = one_digit as u8;
                        val /= 10; 
                        let ten_digit = val % 10;
                        self.memory.ram[(self.registers.i + 2) as usize] = ten_digit as u8;
                        val /= 10; 
                        let hundred_digit = val % 10;
                        self.memory.ram[(self.registers.i + 2) as usize] = hundred_digit as u8;
                    },
                    // 0xFX55
                    0x0055 => {
                        for (i, x) in self.registers.gp_registers.iter().enumerate() {
                            self.memory.ram[(self.registers.i + i as u16) as usize] = *x;
                        }
                        self.registers.program_counter += 2;
                    },
                    // 0xFX65
                    0x0065 => {
                        for (i, x) in self.registers.gp_registers.iter_mut().enumerate() {
                            *x = self.memory.ram[(self.registers.i + i as u16) as usize];
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

    pub fn get_wait_flag(&self) -> bool {
        self.is_waiting_for_key
    }

    pub fn set_wait_flag(&mut self, flag: bool) {
        self.is_waiting_for_key = flag;
    }

    pub fn resume_cycle(&mut self) {
        self.registers.program_counter += 2;
    }

    pub fn get_color_array(&self) -> Vec<u8> {
        self.graphic.color_array.clone()
    }
}
