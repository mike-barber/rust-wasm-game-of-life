//import * as wasm from "hello-wasm-pack";
//import * as wasm from "wasm-game-of-life";
import { Universe, Cell, wasm_draw_grid, wasm_draw_cells } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

const CELL_SIZE = 5;
const GRID_COLOR = "#004000";
const DEAD_COLOR = "#001000";
const LIVE_COLOR = "#00FF00";

const universe = Universe.new();
const width = universe.width();
const height = universe.height();

const canvas = document.getElementById("game-of-life-canvas");
canvas.height = CELL_SIZE * height;
canvas.width = CELL_SIZE * width;

const ctx = canvas.getContext("2d");



let animationId = null;

const renderLoop = () => {
    fps.render();

    //debugger; -- useful for a breakpoint
    universe.tick();
    drawBoth();
        
    animationId = requestAnimationFrame(renderLoop);
}

const drawBoth = () => {
    // draw using JS
    drawCells();
    drawGrid();

    // draw using WASM
    //wasm_draw_cells(ctx, CELL_SIZE, universe);
    //wasm_draw_grid(ctx, CELL_SIZE, universe);
}


const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // vertical lines
    let yc = CELL_SIZE * height;
    for (let i = 0; i <= width; ++i) {
        let xi = i * CELL_SIZE;
        ctx.moveTo(xi, 0);
        ctx.lineTo(xi, yc);
    }

    // horizontal lines
    let xc = CELL_SIZE * width;
    for (let i = 0; i <= height; ++i) {
        let yi = i * CELL_SIZE;
        ctx.moveTo(0, yi);
        ctx.lineTo(xc, yi);
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

    // draw live cells
    ctx.fillStyle = LIVE_COLOR;
    for (let row = 0; row < height; ++row) {
        for (let col = 0; col < width; ++col) {
            const idx = getIndex(row, col);
            if (cells[idx] === Cell.Alive) {
                ctx.fillRect(
                    col * CELL_SIZE,
                    row * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE
                )
            }
        }
    }

    // draw dead cells
    ctx.fillStyle = DEAD_COLOR;
    for (let row = 0; row < height; ++row) {
        for (let col = 0; col < width; ++col) {
            const idx = getIndex(row, col);
            if (cells[idx] === Cell.Dead) {
                ctx.fillRect(
                    col * CELL_SIZE,
                    row * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE
                )
            }
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

    const row = Math.min(Math.floor(canvasTop / CELL_SIZE), height);
    const col = Math.min(Math.floor(canvasLeft / CELL_SIZE), width);

    universe.flip_cell(row,col);
    drawBoth();
});

blankButton.addEventListener("click", _ => {
    universe.reset_zero();
    drawBoth();
});

randomButton.addEventListener("click", _ => {
    universe.reset_random();
    drawBoth();
});


// performance measurement
const fps = new class {
    constructor() {
        this.fps = document.getElementById("fps");
        this.frames = [];
        this.lastFrameTimeStamp = performance.now();
    }

    render() {
        const now = performance.now();
        const delta = now - this.lastFrameTimeStamp;
        this.lastFrameTimeStamp = now;

        const lastFps = 1000 / delta;

        // save the last 100 timings
        this.frames.push(lastFps);
        if (this.frames.length > 100) {
            this.frames.shift();
        }

        // max, min, mean
        let min = Infinity;
        let max = -Infinity;
        let sum = 0;
        for (let i = 0; i < this.frames.length; ++i) {
            let curr = this.frames[i];
            sum += curr;
            min = Math.min(min, curr);
            max = Math.max(max, curr);
        }
        let mean = sum / this.frames.length;

        // render
        this.fps.textContent = `
Frames per second:
    latest = ${Math.round(lastFps)}
    avg    = ${Math.round(mean)}
    min    = ${Math.round(min)}
    max    = ${Math.round(max)}
`.trim();
    }
}


// start
drawBoth();
play();
