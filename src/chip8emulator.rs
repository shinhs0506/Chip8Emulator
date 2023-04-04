use rand::Rng;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

const MEMORY_SIZE: usize = 4096;
const NUM_GP_REGISTERS: usize = 16;
const STACK_SIZE: usize = 16;
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
        self.load_font_set();
        self.load_program(buffer);
    }

    fn load_font_set(&mut self) {
        self.memory.ram[..(NUM_FONTS * FONT_ADDRESS_OFFSET)].copy_from_slice(&self.font_set.font_set);
    }

    fn load_program(&mut self, buffer: &[u8]) {
        let program_start_memory_address = self.memory.program_start_address as usize;
        let program_end_memory_adderess = program_start_memory_address as usize + buffer.len();
        self.memory.ram[program_start_memory_address..program_end_memory_adderess].copy_from_slice(buffer);
    }

    pub fn emulate_cycle(&mut self) {
        let opcode = self.fetch_opcode();
        let (op1, op2, op3, op4, x, y, n, nnn, kk) = self.decode_opcode(opcode);

        match (op1, op2, op3, op4) {
            (0x0, 0x0, 0xE, 0x0) => {
                self.graphic.pixels = [0; WIDTH * HEIGHT];
                self.registers.program_counter += 2;
            },
            (0x0, 0x0, 0xE, 0xE) => {
                self.stack.stack_pointer -= 1;
                self.registers.program_counter = self.stack.stack[self.stack.stack_pointer as usize];
                self.registers.program_counter += 2;
            },
            (0x1, _, _, _) => {
                self.registers.program_counter = nnn;
            },
            (0x2, _, _, _) => {
                self.stack.stack[self.stack.stack_pointer as usize] = self.registers.program_counter;
                self.stack.stack_pointer += 1;
                self.registers.program_counter = nnn;
            },
            (0x3, _, _, _) => {
                if self.registers.gp_registers[x] == kk as u8 {
                    self.registers.program_counter += 4;
                } else {
                    self.registers.program_counter += 2;
                }
            }, 
            (0x4, _, _, _) => {
                if self.registers.gp_registers[x] != kk as u8 {
                    self.registers.program_counter += 4;
                } else {
                    self.registers.program_counter += 2;
                }
            },
            (0x5, _, _, 0x0) => {
                if self.registers.gp_registers[x] == self.registers.gp_registers[y] {
                    self.registers.program_counter += 4;
                } else {
                    self.registers.program_counter += 2;
                }
            },
            (0x6, _, _, _) => {
                self.registers.gp_registers[x] = kk as u8;
                self.registers.program_counter += 2;
            },
            (0x7, _, _, _) => {
                self.registers.gp_registers[x] = self.registers.gp_registers[x].wrapping_add(kk as u8);
                self.registers.program_counter += 2;
            },
            (0x8, _, _, 0x0) => {
                self.registers.gp_registers[x] = self.registers.gp_registers[y];
                self.registers.program_counter += 2;
            },
            (0x8, _, _, 0x1) => {
                self.registers.gp_registers[x] |= self.registers.gp_registers[y];
                self.registers.program_counter += 2;
            },
            (0x8, _, _, 0x2) => {
                self.registers.gp_registers[x] &= self.registers.gp_registers[y];
                self.registers.program_counter += 2;
            },
            (0x8, _, _, 0x3) => {
                self.registers.gp_registers[x] ^= self.registers.gp_registers[y];
                self.registers.program_counter += 2;
            },
            (0x8, _, _, 0x4) => {
                let (res, overflow) = self.registers.gp_registers[x].overflowing_add(self.registers.gp_registers[y]);
                self.registers.gp_registers[x] = res;
                self.registers.gp_registers[NUM_GP_REGISTERS-1] = if overflow { 1 } else { 0 };
                self.registers.program_counter += 2;
            },
            (0x8, _, _, 0x5) => {
                let (res, borrow) = self.registers.gp_registers[x].overflowing_sub(self.registers.gp_registers[y]);
                self.registers.gp_registers[x] = res;
                self.registers.gp_registers[NUM_GP_REGISTERS-1] = if borrow { 0 } else { 1 };
                self.registers.program_counter += 2;
            },
            (0x8, _, _, 0x6) => {
                let val = self.registers.gp_registers[x];
                self.registers.gp_registers[x] = val >> 1;
                self.registers.gp_registers[NUM_GP_REGISTERS-1] = val & 0x01;
                self.registers.program_counter += 2;
            },
            (0x8, _, _, 0x7) => {
                let (res, borrow) = self.registers.gp_registers[y].overflowing_sub(self.registers.gp_registers[x]);
                self.registers.gp_registers[y] = res;
                self.registers.gp_registers[NUM_GP_REGISTERS-1] = if borrow { 0 } else { 1 };
                self.registers.program_counter += 2;
            },
            (0x8, _, _, 0xE) => {
                let val = self.registers.gp_registers[x];
                self.registers.gp_registers[x] = val << 1;
                self.registers.gp_registers[NUM_GP_REGISTERS-1] = (val >> 7) & 0x01;
                self.registers.program_counter += 2;
            },
            (0x9, _, _, 0x0) => {
                if self.registers.gp_registers[x] != self.registers.gp_registers[y] {
                    self.registers.program_counter += 4;
                } else {
                    self.registers.program_counter += 2;
                }
            },
            (0xA, _, _, _) => {
                self.registers.i = nnn;
                self.registers.program_counter += 2;
            },
            (0xB, _, _, _) => {
                self.registers.program_counter = self.registers.gp_registers[0 as usize] as u16 + nnn;
            },
            (0xC, _, _, _) => {
                let rand: u8 = rand::thread_rng().gen();
                self.registers.gp_registers[x] = rand & kk as u8;
                self.registers.program_counter += 2;
            },
            (0xD, _, _, _) => {
                let x_val = self.registers.gp_registers[x];
                let y_val = self.registers.gp_registers[y];

                self.registers.gp_registers[NUM_GP_REGISTERS-1] = 0;
                
                for row in 0..n {
                    let pixel = self.memory.ram[(self.registers.i + row) as usize];
                    for col in 0..8 {
                        if pixel & (0x80 >> col) != 0 {
                            let r = (y_val as u16 + row) as usize % 32;
                            let c = (x_val as u16 + col) as usize % 64;
                            if self.graphic.pixels[(c + 64 * r)] == 1 {
                                self.registers.gp_registers[NUM_GP_REGISTERS-1] = 1;
                            }
                            self.graphic.pixels[(c + 64 * r)] ^= 1; 
                        }
                    }
                } 
                self.registers.program_counter += 2;
            },
            (0xE, _, 0x9, 0xE) => {
                let val = self.registers.gp_registers[x];
                if self.input.pressed[val as usize] {
                    self.registers.program_counter += 4;
                } else {
                    self.registers.program_counter += 2;
                }
            },
            (0xE, _, 0xA, 0x1) => {
                let val = self.registers.gp_registers[x];
                if !self.input.pressed[val as usize] {
                    self.registers.program_counter += 4;
                } else {
                    self.registers.program_counter += 2;
                }
            },
            (0xF, _, 0x0, 0x7) => {
                self.registers.gp_registers[x] = self.registers.delay_timer;
                self.registers.program_counter += 2;
            },
            (0xF, _, 0x0, 0xA) => {
                let mut pressed = false;
                for (idx, val) in self.input.pressed.iter().enumerate() {
                    if *val {
                        self.registers.gp_registers[x] = idx as u8;
                        pressed = true;
                        break;
                    }
                }

                if pressed {
                    self.registers.program_counter += 2;
                }
            },
            (0xF, _, 0x1, 0x5) => {
                self.registers.delay_timer = self.registers.gp_registers[x];
                self.registers.program_counter += 2;
            },
            (0xF, _, 0x1, 0x8) => {
                self.registers.sound_timer = self.registers.gp_registers[x];
                self.registers.program_counter += 2;
            },
            (0xF, _, 0x1, 0xE) => {
                self.registers.i = self.registers.i.wrapping_add(self.registers.gp_registers[x].into());
                self.registers.program_counter += 2;
            },
            (0xF, _, 0x2, 0x9) => {
                self.registers.i = (self.registers.gp_registers[x] * FONT_ADDRESS_OFFSET as u8) as u16;
                self.registers.program_counter += 2;
            },
            (0xF, _, 0x3, 0x3) => {
                let val = self.registers.gp_registers[x] as f32;
                self.memory.ram[(self.registers.i) as usize] = ((val / 100.0) % 10.0).floor() as u8;
                self.memory.ram[(self.registers.i + 1) as usize] = ((val / 10.0) % 10.0).floor() as u8;
                self.memory.ram[(self.registers.i + 2) as usize] = (val % 10.0).floor() as u8;
                self.registers.program_counter += 2;
            },
            (0xF, _, 0x5, 0x5) => {
                for idx in 0..=x {
                    self.memory.ram[self.registers.i as usize + idx as usize] = self.registers.gp_registers[idx as usize];
                }
                self.registers.program_counter += 2;
            },
            (0xF, _, 0x6, 0x5) => {
                for idx in 0..=x {
                    self.registers.gp_registers[idx as usize] = self.memory.ram[self.registers.i as usize + idx as usize];
                }
                self.registers.program_counter += 2;
            },
            (_, _, _, _) => print!("Wrong Opcode"),
        }
    }

    fn fetch_opcode(&self) -> u16 {
        (self.memory.ram[self.registers.program_counter as usize] as u16) << 8 | self.memory.ram[self.registers.program_counter as usize + 1] as u16
    }

    fn decode_opcode(&self, opcode: u16) -> (u8, u8, u8, u8, usize, usize, u16, u16, u16){
        (
            ((opcode & 0xF000) >> 12) as u8,
            ((opcode & 0x0F00) >> 8) as u8,
            ((opcode & 0x00F0) >> 4) as u8,
            (opcode & 0x000F) as u8,
            ((opcode & 0x0F00) >> 8) as usize,
            ((opcode & 0x00F0) >> 4) as usize,
            (opcode & 0x000F),
            (opcode & 0x0FFF),
            (opcode & 0x00FF),
        )
    }

    pub fn advance_timers(&mut self) {
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
