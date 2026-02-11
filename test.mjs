import { readFile } from "node:fs/promises";

async function main(){
    const wasmBytes = await readFile("./rs_img2sz_wasm.wasm");
    const { instance } = await WebAssembly.instantiate(wasmBytes);
    const { exports } = instance;
    const {
        memory,
        height,
        input_img_hdr_small_ptr,
        width,
        guess_size,
    } = exports;

    const ptr = input_img_hdr_small_ptr();
    console.info({ptr});
    const view = new Uint8Array(memory.buffer, ptr, 1024);
    const dat = Buffer.from([
        "P2",
        "2 2",
        "255",
        "1 2",
        "3 4",
        "",
    ].join("\n"));
    view.set(dat);

    const ok = 0 <= guess_size();

    if(!ok) {
        console.error("unable to guess the size");
        return;
    }

    const w = width();
    const h = height();

    console.info(`width: ${w}, height: ${h}`);
}

main()
