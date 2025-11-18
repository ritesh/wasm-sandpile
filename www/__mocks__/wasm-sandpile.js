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
    // Simulate adding a grain to a random cell
    const randomIndex = Math.floor(Math.random() * this._cells.length);
    this._cells[randomIndex] = (this._cells[randomIndex] + 1) % 5;
  }

  static new() {
    return new MockUniverse();
  }
}

export const Universe = MockUniverse;
