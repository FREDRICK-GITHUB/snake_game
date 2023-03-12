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
    const result = sum_function(100,500);
    console.log(result);
}

init();