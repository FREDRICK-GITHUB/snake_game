
import init, { World } from "snake_game";

init().then(_ => {
    const CELL_SIZE = 20;

    const world = World.new();
    const world_width = world.width();

    const canvas = document.getElementById("snake_canvas");
    const canvas_ctx = canvas.getContext("2d");

    canvas.height = world_width * CELL_SIZE;
    canvas.width = world_width * CELL_SIZE;

    function drawWorld() {
        // Draw the rows and the columns for the game
        canvas_ctx.beginPath();
        
        for (let x = 0; x < world_width + 1; x++){
            canvas_ctx.moveTo(CELL_SIZE * x, 0);
            canvas_ctx.lineTo(CELL_SIZE * x, world_width * CELL_SIZE)
        }

        for (let y = 0; y < world_width + 1; y++){
            canvas_ctx.moveTo(0, CELL_SIZE * y);
            canvas_ctx.lineTo(world_width * CELL_SIZE, CELL_SIZE * y)
        }

        canvas_ctx.stroke();
    }

    drawWorld();
})
