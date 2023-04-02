struct Chip8Emulator {
    memory: Memory,
    registers: Registers,
    stack: Stack,
    graphic : Graphic,
    opcode: OpCode,
}

struct Memory {
    ram: [u8; 4096],
    program_start_address: u16,
}

struct Registers {
    gp_registers: [u8; 16],
    i: u16,
    program_counter: u16,
    delay_timer: u8,
    sound_timer: u8,
}

struct Stack {
    stack: [u16; 16],
    stack_pointer: u8,
}

struct Graphic {
    color_array: [bool; 64 * 32],
}

struct OpCode {
    opcode: u16,
}

impl Chip8Emulator {
    pub fn emulate_cycle(&mut self) {
        self.opcode.opcode = (self.memory.ram[self.registers.program_counter as usize] as u16) << 8 | self.memory.ram[self.registers.program_counter as usize + 1] as u16; 

        match self.opcode.opcode & 0xF000 {
            0x0000 => {
                match self.opcode.opcode {
                    0x00E0 => {

                    },
                    0x00EE => {

                    },
                    // 0NNN
                    _=> {

                    }
                }
            }
            // 1NNN
            0x1000 => {

            },
            // 2NNN
            0x2000 => {

            },
            // 3XNN
            0x3000 => {

            },
            // 4XNN
            0x4000 => {

            },
            // 5XY0
            0x5000 => {

            },
            // 6XNN
            0x6000 => {

            },
            // 7XNN
            0x7000 => {

            },
            0x8000 => {
                match self.opcode.opcode & 0x000F {
                    // 0x8XY0
                    0x0000 => {

                    },
                    // 0x8XY1
                    0x0001 => {

                    },
                    // 0x8XY2
                    0x0002 => {

                    },
                    // 0x8XY3
                    0x0003 => {

                    },
                    // 0x8XY4
                    0x0004 => {

                    },
                    // 0x8XY5
                    0x0005 => {

                    },
                    // 0x8XY6
                    0x0006 => {

                    },
                    // 0x8XY7
                    0x0007 => {

                    },
                    // 0x8XYE
                    0x000E => {

                    },
                    _ => println!("Unknown opcode")
                }
            },
            // 0x9XY0
            0x9000 => {

            },
            // 0xANNN
            0xA000 => {
                self.registers.i = self.opcode.opcode & 0x0FFF;
                self.registers.program_counter += 2;
            },
            // 0xBNNN
            0xB000 => {

            },
            // 0xCXNN
            0xC000 => {

            },
            // 0xDXYN
            0xD000 => {

            },
            0xE000 => {
                match self.opcode.opcode & 0x00FF {
                    // 0xEX9E
                    0x009E => {

                    },
                    // 0xEXA1
                    0x00A1 => {

                    },
                    _ => println!("Wrong opcode")
                }
            },
            0xF000 => {
                match self.opcode.opcode & 0x00FF {
                    // 0xFX07
                    0x0007 => {

                    },
                    // 0xFX0A
                    0x000A => {

                    },
                    // 0xFX15
                    0x0015 => {

                    },
                    // 0xFX18
                    0x0018 => {

                    },
                    // 0xFX1E
                    0x001E => {

                    },
                    // 0xFX29
                    0x0029 => {

                    },
                    // 0xFX33
                    0x0033 => {

                    },
                    // 0xFX55
                    0x0055 => {

                    },
                    // 0xFX65
                    0x0065 => {

                    },
                    _ => println!("Wrong opcode")
                }

            }
            _ => println!("Wrong opcode")
        }
    }
}

pub fn run() {
    let mut chip8_emulator = Chip8Emulator {
        memory: Memory {
            ram: [0; 4096],
            program_start_address: 0x200,
        },
        registers: Registers {
            gp_registers: [0; 16],
            i: 0,
            program_counter: 0x200,
            delay_timer: 0,
            sound_timer: 0,
        },
        stack: Stack {
            stack: [0; 16],
            stack_pointer: 0,
        },
        graphic: Graphic {
            color_array: [false; 64 * 32],
        },
        opcode: OpCode {
            opcode: 0,
        }
    };

    // load fontset

    // laod program into memory
    

    chip8_emulator.emulate_cycle()
} 
