
import init, { World } from "snake_game";

init().then(_ => {
    const world = World.new();
    const canvas = document.getElementById("snake_canvas");
    const canvas_context = canvas.getContext("2d");
})
