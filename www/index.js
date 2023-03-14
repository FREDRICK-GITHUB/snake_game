
import init, { greet } from "snake_game";

init().then(_ => {
    greet("Fredrick");
})

// This is an alternate function to the function above (init)
// async function start() {
//     const wasm = await init();
//     greet("Fredrick");
//     console.log("OK!");
// }
