// Mock implementation of the WASM Universe module for testing

class MockUniverse {
  constructor() {
    this._width = 110;
    this._height = 110;
    this._cells = new Array(this._width * this._height).fill(0);
    this._tickCount = 0;
  }

  width() {
    return this._width;
  }

  height() {
    return this._height;
  }

  cells() {
    return this._cells;
  }

  stable() {
    // Check if all cells are less than 4
    return this._cells.every(cell => cell < 4);
  }

  tick() {
    this._tickCount++;
    // Simple mock: just mark that tick was called
    // Find cells with 4+ grains and topple them
    const newCells = [...this._cells];
    for (let i = 0; i < this._cells.length; i++) {
      if (this._cells[i] >= 4) {
        newCells[i] = this._cells[i] - 4;
        const row = Math.floor(i / this._width);
        const col = i % this._width;

        // Add to neighbors
        if (row > 0) newCells[i - this._width] += 1;
        if (row < this._height - 1) newCells[i + this._width] += 1;
        if (col > 0) newCells[i - 1] += 1;
        if (col < this._width - 1) newCells[i + 1] += 1;
      }
    }
    this._cells = newCells;
  }

  static new() {
    return new MockUniverse();
  }
}

export const Universe = MockUniverse;
