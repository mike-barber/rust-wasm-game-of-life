//import * as wasm from "hello-wasm-pack";
//import * as wasm from "wasm-game-of-life";
import { Universe, Cell } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

const CELL_SIZE = 5;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const LIVE_COLOR = "#000000";

const universe = Universe.new();
const width = universe.width();
const height = universe.height();

const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext("2d");



let animationId = null;

const renderLoop = () => {
    //debugger; -- useful for a breakpoint
    universe.tick();

    drawGrid();
    drawCells();
    animationId = requestAnimationFrame(renderLoop);
}


const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // vertical lines
    for (let i = 0; i <= width; ++i) {
        let xc = i * (CELL_SIZE + 1) + 1;
        let yi = i * (CELL_SIZE + 1) * height + 1;
        ctx.moveTo(xc, 0);
        ctx.lineTo(xc, yi);
    }

    // horizontal lines
    for (let i = 0; i <= height; ++i) {
        let xi = (CELL_SIZE + 1) * width + 1;
        let yc = i * (CELL_SIZE + 1) + 1;
        ctx.moveTo(0, yc);
        ctx.lineTo(xi, yc);
    }

    ctx.stroke();
}

const getIndex = (row, column) => {
    return row * width + column;
}

const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();
    for (let row = 0; row < height; ++row) {
        for (let col = 0; col < width; ++col) {
            const idx = getIndex(row, col);

            ctx.fillStyle = cells[idx] === Cell.Dead
                ? DEAD_COLOR
                : LIVE_COLOR;

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            )
        }
    }
    ctx.stroke();
}

/**
 * Interactions
 */

const playPauseButton = document.getElementById("btn-play-pause");
const blankButton = document.getElementById("btn-blank");
const randomButton = document.getElementById("btn-random");

const isPaused = () => {
    return animationId === null;
};

const play = () => {
    playPauseButton.textContent = "⏸";
    renderLoop();
};

const pause = () => {
    playPauseButton.textContent = "▶";
    cancelAnimationFrame(animationId);
    animationId = null;
};



// hook up the event handler for the button
playPauseButton.addEventListener("click", _ => {
    if (isPaused()) {
        play();
    } else {
        pause();
    }
});

// flip cells when the canvas is clicked (and redraw)
canvas.addEventListener("click", event => {
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;

    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

    universe.flip_cell(row,col);
    drawGrid();
    drawCells();
});

blankButton.addEventListener("click", _ => {
    universe.reset_zero();
    drawGrid();
    drawCells();
});

randomButton.addEventListener("click", _ => {
    universe.reset_random();
    drawGrid();
    drawCells();
});


// start
drawGrid();
drawCells();
play();
