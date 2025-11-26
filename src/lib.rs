use rand::distr::Uniform;
use rand::prelude::Distribution;

use wasm_bindgen::prelude::*;

const WIDTH: usize = 110;
const HEIGHT: usize = 110;

#[wasm_bindgen]
pub struct Universe {
    width: usize,
    height: usize,
    cells: Vec<usize>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let mut rng = rand::rng();
        let dist = Uniform::<usize>::new(0, 4).unwrap();
        let cells: Vec<usize> = (0..WIDTH * HEIGHT)
            .map(|_| dist.sample(&mut rng))
            .collect();

        Universe {
            width: WIDTH,
            height: HEIGHT,
            cells,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn cells(&self) -> Vec<usize> {
        self.cells.clone()
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        // Find all cells that need to topple (>= 4 grains)
        let mut to_topple = Vec::new();
        for (idx, &value) in self.cells.iter().enumerate() {
            if value >= 4 {
                to_topple.push(idx);
            }
        }

        // Topple all unstable cells
        for idx in to_topple {
            let row = idx / self.width;
            let col = idx % self.width;

            // Remove 4 grains from current cell
            next[idx] = next[idx].saturating_sub(4);

            // Add 1 grain to each neighbor (if it exists)
            // Top neighbor
            if row > 0 {
                next[idx - self.width] += 1;
            }
            // Bottom neighbor
            if row < self.height - 1 {
                next[idx + self.width] += 1;
            }
            // Left neighbor
            if col > 0 {
                next[idx - 1] += 1;
            }
            // Right neighbor
            if col < self.width - 1 {
                next[idx + 1] += 1;
            }
        }

        self.cells = next;
    }

    pub fn stable(&self) -> bool {
        self.cells.iter().all(|&cell| cell < 4)
    }
}

// #[cfg(test)]
// mod tests {

//    }
