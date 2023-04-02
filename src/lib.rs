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
                    // 0nnn
                    _=> {

                    }
                }
            }
            0xA000 => {
                self.registers.i = self.opcode.opcode & 0x0FFF;
                self.registers.program_counter += 2;
            },
            _ => {

            }
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
