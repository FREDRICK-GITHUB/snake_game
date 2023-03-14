async function init() {

    const memory = new WebAssembly.Memory({initial: 1});

    const importObject = {
        js: {
            mem: memory
        },
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
    await WebAssembly.instantiate(buffer, importObject);

    const uint8_array = new Uint8Array(memory.buffer, 0, 2);

    const hi_text = new TextDecoder().decode(uint8_array);
    console.log(hi_text);
}

init();