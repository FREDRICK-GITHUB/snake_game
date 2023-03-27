
import init, { World } from "snake_game";

init().then(_ => {
    const world = World.new();
    console.log(world.width);
})

// This is an alternate function to the function above (init)
// async function start() {
//     const wasm = await init();
//     greet("Fredrick");
//     console.log("OK!");
// }
