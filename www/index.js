import { Universe, Cell, Species } from "wasm-rust-sandy";
import { memory } from "wasm-rust-sandy/wasm_rust_sandy_bg";

const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC";

// Construct the universe, and get its width and height.
const universe = Universe.new();
const width = universe.width();
const height = universe.height();

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("sandy-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

const renderLoop = () => {
    universe.tick();

    drawGrid();
    drawCells();

    requestAnimationFrame(renderLoop);
};

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
};

const getIndex = (row, column) => {
    return row * height + column;
};

const drawCells = () => {
    const cellsPtr = universe.cells();
    const perChunk = 4;

    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height * 4);

    const cellsArr = cells.reduce((resultArray, item, index) => {
        const chunkIndex = Math.floor(index / perChunk)
        const modulus = Math.floor(index % perChunk)

        if (!resultArray[chunkIndex]) {
            resultArray[chunkIndex] = {} // start a new chunk
        }

        if (modulus === 0) {
            resultArray[chunkIndex].specie = item
        } else if (modulus === 1) {
            resultArray[chunkIndex].ra = item
        } else if (modulus === 2) {
            resultArray[chunkIndex].rb = item
        } else if (modulus === 3) {
            resultArray[chunkIndex].clock = item
        }


        return resultArray
    }, []);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);

            ctx.fillStyle = color(cellsArr[idx].specie)

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
};


const color = (specie) => {
    switch (specie) {
        case Species.Sand:
            return "#D2AA6D";
        case Species.Water:
            return "#0000FF";
        case Species.Wall:
            return "#000000";
        // case Species.Plant:
        //     return "#00FF00";
        // case Species.Fire:
        //     return "#FFA500";
        // case Species.Air:
        //     return "#FFFFFF";
        default:
            return "#FFFFFF";
    }
}


drawGrid();
drawCells();
requestAnimationFrame(renderLoop);
