/* tslint:disable */
/* eslint-disable */
/**
* @returns {string}
*/
export function helloworld(): string;
/**
* @param {CanvasRenderingContext2D} context
* @param {number} width
* @param {number} height
* @param {Float32Array} pos
* @param {Float32Array} pyr
*/
export function render_func(context: CanvasRenderingContext2D, width: number, height: number, pos: Float32Array, pyr: Float32Array): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly helloworld: (a: number) => void;
  readonly render_func: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
        