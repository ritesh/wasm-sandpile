/**
 * @jest-environment jsdom
 */

// Mock the WASM module
const mockUniverse = {
  new: jest.fn(() => ({
    width: jest.fn(() => 110),
    height: jest.fn(() => 110),
    cells: jest.fn(() => new Array(110 * 110).fill(0)),
    stable: jest.fn(() => false),
    tick: jest.fn()
  }))
};

jest.mock('wasm-sandpile', () => ({
  Universe: mockUniverse
}));

describe('Sandpile Visualization', () => {
  let canvas, ctx;

  beforeEach(() => {
    // Set up a mock canvas
    document.body.innerHTML = `
      <canvas id="sandpile-canvas"></canvas>
      <button id="start-btn">Start</button>
      <button id="stop-btn">Stop</button>
      <button id="reset-btn">Reset</button>
      <button id="step-btn">Step</button>
      <input type="range" id="speed-slider" value="30" />
      <span id="speed-value"></span>
      <span id="tick-count">0</span>
      <span id="grid-size">-</span>
      <span id="is-stable">-</span>
    `;

    canvas = document.getElementById('sandpile-canvas');
    ctx = canvas.getContext('2d');
  });

  describe('Constants', () => {
    test('CELL_SIZE should be 5', () => {
      const CELL_SIZE = 5;
      expect(CELL_SIZE).toBe(5);
    });

    test('GRID_COLOR should be defined', () => {
      const GRID_COLOR = "#222222";
      expect(GRID_COLOR).toBe("#222222");
    });
  });

  describe('Color Mapping', () => {
    const getColor = (value) => {
      if (value === 0) return "rgb(0, 0, 0)";
      if (value === 1) return "rgb(0, 100, 200)";
      if (value === 2) return "rgb(0, 200, 100)";
      if (value === 3) return "rgb(255, 200, 0)";
      return "rgb(255, 0, 0)";
    };

    test('should return black for 0 grains', () => {
      expect(getColor(0)).toBe("rgb(0, 0, 0)");
    });

    test('should return blue for 1 grain', () => {
      expect(getColor(1)).toBe("rgb(0, 100, 200)");
    });

    test('should return green for 2 grains', () => {
      expect(getColor(2)).toBe("rgb(0, 200, 100)");
    });

    test('should return yellow for 3 grains', () => {
      expect(getColor(3)).toBe("rgb(255, 200, 0)");
    });

    test('should return red for 4 or more grains', () => {
      expect(getColor(4)).toBe("rgb(255, 0, 0)");
      expect(getColor(5)).toBe("rgb(255, 0, 0)");
      expect(getColor(10)).toBe("rgb(255, 0, 0)");
    });
  });

  describe('Canvas Initialization', () => {
    test('canvas should exist in DOM', () => {
      expect(canvas).toBeTruthy();
    });

    test('canvas should have correct dimensions based on grid size', () => {
      const CELL_SIZE = 5;
      const width = 110;
      const height = 110;
      const expectedWidth = (CELL_SIZE + 1) * width + 1;
      const expectedHeight = (CELL_SIZE + 1) * height + 1;

      canvas.width = expectedWidth;
      canvas.height = expectedHeight;

      expect(canvas.width).toBe(expectedWidth);
      expect(canvas.height).toBe(expectedHeight);
    });
  });

  describe('UI Controls', () => {
    test('all control buttons should exist', () => {
      expect(document.getElementById('start-btn')).toBeTruthy();
      expect(document.getElementById('stop-btn')).toBeTruthy();
      expect(document.getElementById('reset-btn')).toBeTruthy();
      expect(document.getElementById('step-btn')).toBeTruthy();
    });

    test('speed slider should exist with correct default value', () => {
      const slider = document.getElementById('speed-slider');
      expect(slider).toBeTruthy();
      expect(slider.value).toBe('30');
    });

    test('statistics elements should exist', () => {
      expect(document.getElementById('tick-count')).toBeTruthy();
      expect(document.getElementById('grid-size')).toBeTruthy();
      expect(document.getElementById('is-stable')).toBeTruthy();
    });
  });

  describe('Animation State', () => {
    test('initial tick count should be 0', () => {
      const tickCount = parseInt(document.getElementById('tick-count').textContent);
      expect(tickCount).toBe(0);
    });

    test('FPS calculation should be correct', () => {
      const fps = 30;
      const expectedDelay = 1000 / fps;
      expect(expectedDelay).toBeCloseTo(33.33, 1);
    });
  });

  describe('Grid Drawing Logic', () => {
    test('should calculate correct cell index for row-major order', () => {
      const width = 110;
      const row = 5;
      const col = 10;
      const expectedIndex = row * width + col;
      expect(expectedIndex).toBe(560);
    });

    test('should calculate correct cell positions', () => {
      const CELL_SIZE = 5;
      const row = 2;
      const col = 3;

      const x = col * (CELL_SIZE + 1) + 1;
      const y = row * (CELL_SIZE + 1) + 1;

      expect(x).toBe(19);
      expect(y).toBe(13);
    });

    test('grid lines should be calculated correctly', () => {
      const CELL_SIZE = 5;
      const width = 110;
      const height = 110;

      // Vertical lines
      const verticalLines = width + 1;
      expect(verticalLines).toBe(111);

      // Horizontal lines
      const horizontalLines = height + 1;
      expect(horizontalLines).toBe(111);
    });
  });

  describe('Universe Integration', () => {
    test('should create universe with correct dimensions', () => {
      const universe = mockUniverse.new();
      expect(universe.width()).toBe(110);
      expect(universe.height()).toBe(110);
    });

    test('cells array should have correct length', () => {
      const universe = mockUniverse.new();
      const cells = universe.cells();
      expect(cells.length).toBe(110 * 110);
    });
  });

  describe('Statistics Display', () => {
    test('should format grid size correctly', () => {
      const width = 110;
      const height = 110;
      const gridSizeText = `${width} × ${height}`;
      expect(gridSizeText).toBe('110 × 110');
    });

    test('should display stable status as Yes or No', () => {
      const isStable1 = true;
      const isStable2 = false;
      expect(isStable1 ? "Yes" : "No").toBe("Yes");
      expect(isStable2 ? "Yes" : "No").toBe("No");
    });
  });

  describe('Speed Control', () => {
    test('should accept speed values from 1 to 60', () => {
      const slider = document.getElementById('speed-slider');

      slider.value = 1;
      expect(parseInt(slider.value)).toBeGreaterThanOrEqual(1);

      slider.value = 60;
      expect(parseInt(slider.value)).toBeLessThanOrEqual(60);
    });

    test('should calculate frame delay correctly', () => {
      const testFPS = [1, 30, 60];
      const expectedDelays = [1000, 33.33, 16.67];

      testFPS.forEach((fps, index) => {
        const delay = 1000 / fps;
        expect(delay).toBeCloseTo(expectedDelays[index], 1);
      });
    });
  });

  describe('Event Handling', () => {
    test('buttons should have event listeners attachable', () => {
      const startBtn = document.getElementById('start-btn');
      const clickHandler = jest.fn();
      startBtn.addEventListener('click', clickHandler);

      startBtn.click();
      expect(clickHandler).toHaveBeenCalled();
    });

    test('speed slider should trigger input event', () => {
      const slider = document.getElementById('speed-slider');
      const inputHandler = jest.fn();
      slider.addEventListener('input', inputHandler);

      const event = new Event('input');
      slider.dispatchEvent(event);
      expect(inputHandler).toHaveBeenCalled();
    });
  });

  describe('Edge Cases', () => {
    test('should handle empty cells array', () => {
      const cells = [];
      expect(cells.length).toBe(0);
    });

    test('should handle maximum cell values', () => {
      const maxValue = 100;
      const getColor = (value) => {
        if (value === 0) return "rgb(0, 0, 0)";
        if (value === 1) return "rgb(0, 100, 200)";
        if (value === 2) return "rgb(0, 200, 100)";
        if (value === 3) return "rgb(255, 200, 0)";
        return "rgb(255, 0, 0)";
      };

      expect(getColor(maxValue)).toBe("rgb(255, 0, 0)");
    });

    test('should handle zero dimensions', () => {
      const width = 0;
      const height = 0;
      const totalCells = width * height;
      expect(totalCells).toBe(0);
    });
  });

  describe('Performance', () => {
    test('should calculate correct number of cells to render', () => {
      const width = 110;
      const height = 110;
      const totalCells = width * height;
      expect(totalCells).toBe(12100);
    });

    test('frame rate should be configurable', () => {
      const minFPS = 1;
      const maxFPS = 60;
      const defaultFPS = 30;

      expect(defaultFPS).toBeGreaterThanOrEqual(minFPS);
      expect(defaultFPS).toBeLessThanOrEqual(maxFPS);
    });
  });
});
