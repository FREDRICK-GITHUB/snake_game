
import init, { World, Direction, GameStatus } from "snake_game";
import { random } from "./utils/random";

init().then(wasm => {
    const CELL_SIZE = 20;
    const WORLD_WIDTH = 4;
    const snakeSpawnIndex = random(WORLD_WIDTH * WORLD_WIDTH);

    const world = World.new(WORLD_WIDTH, snakeSpawnIndex);
    const worldWidth = world.width();

    const points = document.getElementById("points");
    const gameStatus = document.getElementById("game-status");
    const gameCotrolButton = document.getElementById("game-control-btn");
    const canvas = <HTMLCanvasElement>document.getElementById("snake_canvas");
    const canvasCtx = canvas.getContext("2d");

    canvas.height = worldWidth * CELL_SIZE;
    canvas.width = worldWidth * CELL_SIZE;

    gameCotrolButton.addEventListener("click", _ => {
        const status = world.game_status();

        if (status === undefined) {
            gameCotrolButton.textContent = "Playing...";
            world.start_game();
            play();
        } else {
            location.reload();
        }

    })

    document.addEventListener("keydown", (keyEvent) => {
        switch (keyEvent.code) {
            case "ArrowUp":
                world.change_snake_dir(Direction.Up);
                break;

            case "ArrowRight":
                world.change_snake_dir(Direction.Right);
                break;

            case "ArrowDown":
                world.change_snake_dir(Direction.Down);
                break;

            case "ArrowLeft":
                world.change_snake_dir(Direction.Left);
                break;
        }
    })

    function drawWorld() {
        // Draw the rows and the columns for the game
        canvasCtx.beginPath();

        for (let x = 0; x < worldWidth + 1; x++) {
            canvasCtx.moveTo(CELL_SIZE * x, 0);
            canvasCtx.lineTo(CELL_SIZE * x, worldWidth * CELL_SIZE)
        }

        for (let y = 0; y < worldWidth + 1; y++) {
            canvasCtx.moveTo(0, CELL_SIZE * y);
            canvasCtx.lineTo(worldWidth * CELL_SIZE, CELL_SIZE * y)
        }

        canvasCtx.stroke();
    }

    function drawReward() {
        const index = world.reward_cell();
        const column = index % worldWidth;
        const row = Math.floor(index / worldWidth);

        canvasCtx.beginPath();
        canvasCtx.fillStyle = "#FF0000";
        canvasCtx.fillRect(
            column * CELL_SIZE,
            row * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE
        );

        canvasCtx.stroke();

        if (index === 1000) {
            alert("You won!");
        }
    }

    function drawSnake() {
        const snakeCells = new Uint32Array(
            wasm.memory.buffer,
            world.snake_cells(),
            world.snake_length(),
        )

        snakeCells
            .filter((cellIndex, i) => !(i > 0 && cellIndex === snakeCells[0]))
            // commented lines below here outline the second variant as the line above this comment for 
            // ensuring that snake head cell is always visible even after a crush
            // .slice() //creating a copy of the array
            // .reverse()
            .forEach((cellIndex, i) => {
                const col = cellIndex % worldWidth;
                const row = Math.floor(cellIndex / worldWidth);

                // canvasCtx.fillStyle = i === snakeCells.length - 1 ? "#7878db" : "#000000";
                canvasCtx.fillStyle = i === 0 ? "#7878db" : "#000000";

                canvasCtx.beginPath();
                canvasCtx.fillRect(
                    col * CELL_SIZE,
                    row * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE
                );
            })

        canvasCtx.stroke();
    }

    function drawGameStatus() {
        const status = world.game_status();
        gameStatus.textContent = world.game_status_text();
        points.textContent = world.points().toString();

        if (status == GameStatus.Won || status == GameStatus.Lost) {
            gameCotrolButton.textContent = "Re-Play";
        }
    }

    function paint() {
        drawWorld();
        drawSnake();
        drawReward();
        drawGameStatus();
    }

    function play() {
        const fps = 3;
        setTimeout(() => {
            canvasCtx.clearRect(0, 0, canvas.width, canvas.height);
            world.step();
            paint();
            //method below makes the callback to be invoked before the next repaint
            requestAnimationFrame(play)
        }, 1000 / fps)
    }

    paint();

})
