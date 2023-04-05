/* tslint:disable */
/* eslint-disable */
/**
*/
export class Chip8EmulatorWasm {
  free(): void;
/**
*/
  constructor();
/**
* @param {Uint8Array} data
*/
  init(data: Uint8Array): void;
/**
*/
  emulate_cycle(): void;
/**
*/
  advance_timers(): void;
/**
* @param {KeyboardEvent} evt
* @param {boolean} pressed
*/
  keypress(evt: KeyboardEvent, pressed: boolean): void;
/**
*/
  reset(): void;
/**
* @returns {boolean}
*/
  should_render(): boolean;
/**
* @param {boolean} draw_flag
*/
  set_draw_flag(draw_flag: boolean): void;
/**
* @param {number} cell_size
*/
  render(cell_size: number): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_chip8emulatorwasm_free: (a: number) => void;
  readonly chip8emulatorwasm_new: (a: number) => void;
  readonly chip8emulatorwasm_init: (a: number, b: number) => void;
  readonly chip8emulatorwasm_emulate_cycle: (a: number) => void;
  readonly chip8emulatorwasm_advance_timers: (a: number) => void;
  readonly chip8emulatorwasm_keypress: (a: number, b: number, c: number) => void;
  readonly chip8emulatorwasm_reset: (a: number) => void;
  readonly chip8emulatorwasm_should_render: (a: number) => number;
  readonly chip8emulatorwasm_set_draw_flag: (a: number, b: number) => void;
  readonly chip8emulatorwasm_render: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
