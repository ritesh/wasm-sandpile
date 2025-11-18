# Abelian Sandpile Visualization

A WebAssembly implementation of the Abelian Sandpile Model with interactive graphical visualization.

## About

The [Abelian Sandpile Model](https://en.wikipedia.org/wiki/Abelian_sandpile_model) is a cellular automaton that demonstrates self-organized criticality. Each cell in a grid contains a number of "grains" (0-3). When a cell accumulates 4 or more grains, it "topples", distributing one grain to each of its four neighbors. This simple rule creates beautiful fractal patterns and emergent complexity.

This project implements the sandpile algorithm in Rust, compiled to WebAssembly for fast browser-based execution, with an interactive canvas visualization.

## Features

- Real-time sandpile simulation in the browser
- Color-coded visualization (different colors for 0-4+ grains)
- Interactive controls: Start/Stop, Step-by-step execution, Reset
- Adjustable simulation speed (1-60 fps)
- Statistics display (tick count, grid size, stability status)
- 110x110 grid with automatic toppling

## Building and Running

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) with `cargo`
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [Node.js and npm](https://nodejs.org/)

### Build Steps

1. **Build the WebAssembly module:**
   ```bash
   wasm-pack build --target bundler
   ```

2. **Install web dependencies:**
   ```bash
   cd www
   npm install
   ```

3. **Build the web application:**
   ```bash
   npm run build
   ```

4. **Serve the application:**
   ```bash
   npm start
   ```

5. **Open your browser to:** `http://localhost:8080`

### Quick Start

If you have all prerequisites installed, you can run everything in one go:

```bash
wasm-pack build --target bundler && cd www && npm install && npm run build && npm start
```

## Project Structure

```
wasm-sandpile/
├── src/
│   ├── lib.rs          # Rust implementation of sandpile algorithm
│   └── utils.rs        # Utility functions
├── www/
│   ├── index.html      # Web UI
│   ├── index.js        # JavaScript visualization code
│   ├── bootstrap.js    # WASM loader
│   ├── package.json    # NPM dependencies
│   └── webpack.config.js # Webpack configuration
├── Cargo.toml          # Rust dependencies
└── README.md
```

## How It Works

1. **Rust Core**: The sandpile logic is implemented in Rust for performance
2. **WebAssembly**: Rust code is compiled to WASM for browser execution
3. **JavaScript Visualization**: Canvas API renders the grid with color coding:
   - Black: 0 grains
   - Blue: 1 grain
   - Green: 2 grains
   - Yellow: 3 grains
   - Red: 4+ grains (toppling state)

## Development

### Running Tests

```bash
wasm-pack test --headless --firefox
```

### Modifying the Grid Size

Edit the constants in `src/lib.rs`:

```rust
const WIDTH: usize = 110;
const HEIGHT: usize = 110;
```

Don't forget to rebuild after changes:

```bash
wasm-pack build --target bundler
cd www && npm run build
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
