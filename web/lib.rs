use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};
use js_sys::Uint8Array;

use chip8emulator::*;

#[wasm_bindgen]
pub struct Chip8EmulatorWasm {
    chip8_emulator: Chip8Emulator,
    ctx: CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl Chip8EmulatorWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Chip8EmulatorWasm, JsValue> {
        let chip8_emulator = Chip8Emulator::new();

        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: HtmlCanvasElement = canvas
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let ctx = canvas.get_context("2d")
            .unwrap().unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        Ok(Chip8EmulatorWasm{ chip8_emulator, ctx })
    }

    #[wasm_bindgen]
    pub fn init(&mut self, data: Uint8Array) {
        self.chip8_emulator.init(&data.to_vec());
    }

    #[wasm_bindgen]
    pub fn emulate_cycle(&mut self) {
        self.chip8_emulator.emulate_cycle();
    }

    #[wasm_bindgen]
    pub fn advance_timers(&mut self) {
        self.chip8_emulator.advance_timers();
    }

    #[wasm_bindgen]
    pub fn keypress(&mut self, evt: KeyboardEvent, pressed: bool) {
        let key = evt.key();
        if let Some(k) = key2idx(&key) {
            self.chip8_emulator.set_key(k, pressed);
        }
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.chip8_emulator.reset();
    }

    #[wasm_bindgen]
    pub fn render(&mut self, cell_size: usize) {
        for (idx, val) in self.chip8_emulator.get_color_array().into_iter().enumerate() {
            if *val == 0x01 {
                let ul_x = idx % WIDTH;
                let ul_y = idx / WIDTH;
                self.ctx.fill_rect(
                    (ul_x * cell_size) as f64,
                    (ul_y * cell_size) as f64,
                    cell_size as f64,
                    cell_size as f64
                );
            }
        }
    }
}

fn key2idx(key: &str) -> Option<usize> {
    match key {
        "1" => Some(0x1),
        "2" => Some(0x2),
        "3" => Some(0x3),
        "4" => Some(0xC),
        "q" => Some(0x4),
        "w" => Some(0x5),
        "e" => Some(0x6),
        "r" => Some(0xD),
        "a" => Some(0x7),
        "s" => Some(0x8),
        "d" => Some(0x9),
        "f" => Some(0xE),
        "z" => Some(0xA),
        "x" => Some(0x0),
        "c" => Some(0xB),
        "v" => Some(0xF),
        _ =>   None,
    }
}
