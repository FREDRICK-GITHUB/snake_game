async function init() {
    
    const response = await fetch("sum.wasm");
    const buffer = await response.arrayBuffer();
    const wasm = await WebAssembly.instantiate(buffer);
    
    const sum_function = wasm.instance.exports.sum;
    const result = sum_function(100, 500);
    console.log(result);
}

init();