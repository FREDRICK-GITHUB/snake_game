async function init() {

    const importObject = {
        console: {
            log: () => {
                console.log("Just logging something");
            },
            error: () => {
                console.log("I am just an error");
            }
        }
    }
    
    const response = await fetch("sum.wasm");
    const buffer = await response.arrayBuffer();
    const wasm = await WebAssembly.instantiate(buffer, importObject);

    const sum_function = wasm.instance.exports.sum;
    const wasm_memory = wasm.instance.exports.memory;
    const uint8_array = new Uint8Array(wasm_memory.buffer, 0, 2);

    const hi_text = new TextDecoder().decode(uint8_array);
    console.log(hi_text);
}

init();