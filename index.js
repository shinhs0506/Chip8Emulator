import init, * as wasm from "./pkg/chip8emu.js"

const WIDTH = 64;
const HEIGHT = 32;
const CELL_SIZE = 18;
let animation_frame = 0;

const TICKS_PER_FRAME = 3;

const canvas = document.getElementById("canvas");
canvas.width = WIDTH * CELL_SIZE;
canvas.height = HEIGHT * CELL_SIZE;

const ctx = canvas.getContext("2d");
ctx.fillStyle = "black";
ctx.fillRect(0, 0, WIDTH * CELL_SIZE, HEIGHT * CELL_SIZE);

async function run() {
    await init();
    let chip8_emulator_wasm = new wasm.Chip8EmulatorWasm();


    document.addEventListener("keydown", function(evt) {
        chip8_emulator_wasm.keypress(evt, true)
    });

    document.addEventListener("keyup", function(evt) {
        chip8_emulator_wasm.keypress(evt, false)
    });

    let file_input = document.getElementById("fileinput");
    file_input.addEventListener("change", function(evt) {
        if (animation_frame != 0) {
            window.cancelAnimationFrame(animation_frame);
        }

        let file = evt.target.files[0]
        if (!file) {
            alert("Failed to read file")
            return
        }

        let fr = new FileReader()
        fr.onload = function(e) {
            ctx.fillStyle = "black";
            ctx.fillRect(0, 0, WIDTH * CELL_SIZE, HEIGHT * CELL_SIZE);
            const  buffer = new Uint8Array(fr.result);
            chip8_emulator_wasm.reset();
            chip8_emulator_wasm.init(buffer);
            emulate_cycle_and_draw(chip8_emulator_wasm);
        }
        fr.readAsArrayBuffer(file);
    }, false);
}

function emulate_cycle_and_draw(chip8_emulator_wasm) {
    for (let i = 0; i < TICKS_PER_FRAME; i++) {
        chip8_emulator_wasm.emulate_cycle();
    }
    chip8_emulator_wasm.advance_timers();

    ctx.fillStyle = "black";
    ctx.fillRect(0, 0, WIDTH * CELL_SIZE, HEIGHT * CELL_SIZE);
    ctx.fillStyle = "white";
    chip8_emulator_wasm.render(CELL_SIZE);

    animation_frame = window.requestAnimationFrame(() => {
        emulate_cycle_and_draw(chip8_emulator_wasm);
    });
}

run()
