import { Universe } from "./pkg";

const CELL_SIZE = 5; // pixels
const GRID_COLOR = "#222222";

// Construct the universe
const universe = Universe.new();
const width = universe.width();
const height = universe.height();

// Give the canvas room for all of our cells and a 1px border around each of them
const canvas = document.getElementById("sandpile-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext("2d");

let animationId = null;
let tickCount = 0;
let fps = 30;
let lastFrameTime = Date.now();
let frameDelay = 1000 / fps;

// Transition tracking for blinking effect
let previousCells = null;
let transitions = new Map(); // Maps cell index to {oldValue, newValue, startTime}
const TRANSITION_DURATION = 300; // milliseconds
const BLINK_INTERVAL = 100; // milliseconds between color switches

// Color mapping for different grain counts
const getColor = (value) => {
  if (value === 0) return "rgb(0, 0, 0)";      // Black
  if (value === 1) return "rgb(0, 100, 200)";  // Blue
  if (value === 2) return "rgb(0, 200, 100)";  // Green
  if (value === 3) return "rgb(255, 200, 0)";  // Yellow
  return "rgb(255, 0, 0)";                      // Red (4 or more)
};

// Detect cell changes and add them to transitions
const updateTransitions = (oldCells, newCells) => {
  const now = Date.now();

  for (let i = 0; i < newCells.length; i++) {
    if (oldCells && oldCells[i] !== newCells[i]) {
      transitions.set(i, {
        oldValue: oldCells[i],
        newValue: newCells[i],
        startTime: now
      });
    }
  }

  // Remove expired transitions
  for (const [idx, transition] of transitions.entries()) {
    if (now - transition.startTime > TRANSITION_DURATION) {
      transitions.delete(idx);
    }
  }
};

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  // Horizontal lines
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
};

const drawCells = () => {
  const cells = universe.cells();
  const now = Date.now();

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = row * width + col;
      const value = cells[idx];

      let color = getColor(value);

      // Check if this cell is in transition
      if (transitions.has(idx)) {
        const transition = transitions.get(idx);
        const elapsed = now - transition.startTime;

        // Calculate which color to show based on blink interval
        const blinkCycle = Math.floor(elapsed / BLINK_INTERVAL) % 2;

        // Alternate between old and new colors
        if (blinkCycle === 0) {
          color = getColor(transition.oldValue);
        } else {
          color = getColor(transition.newValue);
        }
      }

      ctx.fillStyle = color;

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

const updateStats = () => {
  document.getElementById("tick-count").textContent = tickCount;
  document.getElementById("grid-size").textContent = `${width} × ${height}`;
  document.getElementById("is-stable").textContent = universe.stable() ? "Yes" : "No";
};

const renderLoop = () => {
  const now = Date.now();
  const elapsed = now - lastFrameTime;

  if (elapsed > frameDelay) {
    // Store previous state before ticking
    const cells = universe.cells();
    previousCells = Array.from(cells);

    universe.tick();
    tickCount++;

    // Detect changes and update transitions
    const newCells = universe.cells();
    updateTransitions(previousCells, Array.from(newCells));

    updateStats();

    lastFrameTime = now - (elapsed % frameDelay);
  }

  // Always redraw to show blinking transitions
  drawGrid();
  drawCells();

  animationId = requestAnimationFrame(renderLoop);
};

const isPaused = () => {
  return animationId === null;
};

const play = () => {
  if (!isPaused()) {
    return;
  }

  startBtn.disabled = true;
  stopBtn.disabled = false;
  stepBtn.disabled = true;

  renderLoop();
};

const pause = () => {
  if (isPaused()) {
    return;
  }

  startBtn.disabled = false;
  stopBtn.disabled = true;
  stepBtn.disabled = false;

  cancelAnimationFrame(animationId);
  animationId = null;
};

const reset = () => {
  pause();
  tickCount = 0;

  // Create a new universe (this will reinitialize with random values)
  window.location.reload();
};

const step = () => {
  if (!isPaused()) {
    return;
  }

  // Store previous state before ticking
  const cells = universe.cells();
  previousCells = Array.from(cells);

  universe.tick();
  tickCount++;

  // Detect changes and update transitions
  const newCells = universe.cells();
  updateTransitions(previousCells, Array.from(newCells));

  drawGrid();
  drawCells();
  updateStats();

  // Continue rendering for transition animation
  if (transitions.size > 0) {
    const animateTransitions = () => {
      drawGrid();
      drawCells();

      // Keep animating while transitions are active
      if (transitions.size > 0 && isPaused()) {
        requestAnimationFrame(animateTransitions);
      }
    };
    requestAnimationFrame(animateTransitions);
  }
};

// UI Controls
const startBtn = document.getElementById("start-btn");
const stopBtn = document.getElementById("stop-btn");
const resetBtn = document.getElementById("reset-btn");
const stepBtn = document.getElementById("step-btn");
const speedSlider = document.getElementById("speed-slider");
const speedValue = document.getElementById("speed-value");

startBtn.addEventListener("click", () => {
  play();
});

stopBtn.addEventListener("click", () => {
  pause();
});

resetBtn.addEventListener("click", () => {
  reset();
});

stepBtn.addEventListener("click", () => {
  step();
});

speedSlider.addEventListener("input", (e) => {
  fps = parseInt(e.target.value);
  frameDelay = 1000 / fps;
  speedValue.textContent = `${fps} fps`;
});

// Initial render
drawGrid();
drawCells();
updateStats();
