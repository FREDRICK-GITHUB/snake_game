
import init, { World } from "snake_game";

init().then(_ => {
    const CELL_SIZE = 20;

    const world = World.new();
    const worldWidth = world.width();

    const canvas = document.getElementById("snake_canvas");
    const canvasCtx = canvas.getContext("2d");

    canvas.height = worldWidth * CELL_SIZE;
    canvas.width = worldWidth * CELL_SIZE;

    function drawWorld() {
        // Draw the rows and the columns for the game
        canvasCtx.beginPath();
        
        for (let x = 0; x < worldWidth + 1; x++){
            canvasCtx.moveTo(CELL_SIZE * x, 0);
            canvasCtx.lineTo(CELL_SIZE * x, worldWidth * CELL_SIZE)
        }

        for (let y = 0; y < worldWidth + 1; y++){
            canvasCtx.moveTo(0, CELL_SIZE * y);
            canvasCtx.lineTo(worldWidth * CELL_SIZE, CELL_SIZE * y)
        }

        canvasCtx.stroke();
    }
    
    function drawSnake() {
        const snakeIndex = world.snake_head_index();
        const col = snakeIndex % worldWidth;
        const row = Math.floor(snakeIndex / worldWidth);

        canvasCtx.beginPath();
        canvasCtx.fillRect(
            col * CELL_SIZE,
            row * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE
        );
        canvasCtx.stroke();
    }

    function paint() {
        drawWorld();
        drawSnake();
    }

    function update() {
        setTimeout(() => {
            canvasCtx.clearRect(0, 0, canvas.width, canvas.height);
            world.update();
            paint();
            //method below makes the callback to be invoked before the next repaint
            requestAnimationFrame(update)
        }, 100)
    }

    paint();
    update();
    
})
