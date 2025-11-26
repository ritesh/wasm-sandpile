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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_universe_creation() {
        let universe = Universe::new();
        assert_eq!(universe.width(), WIDTH);
        assert_eq!(universe.height(), HEIGHT);
        assert_eq!(universe.cells().len(), WIDTH * HEIGHT);
    }

    #[test]
    fn test_cells_initialized_with_valid_values() {
        let universe = Universe::new();
        let cells = universe.cells();
        // All cells should be initialized with values 0-3
        for &cell in cells.iter() {
            assert!(cell < 4, "Cell value {} should be less than 4", cell);
        }
    }

    #[test]
    fn test_stable_with_all_zeros() {
        let mut universe = Universe::new();
        universe.cells = vec![0; WIDTH * HEIGHT];
        assert!(universe.stable(), "Universe with all zeros should be stable");
    }

    #[test]
    fn test_stable_with_all_threes() {
        let mut universe = Universe::new();
        universe.cells = vec![3; WIDTH * HEIGHT];
        assert!(universe.stable(), "Universe with all threes should be stable");
    }

    #[test]
    fn test_unstable_with_single_four() {
        let mut universe = Universe::new();
        universe.cells = vec![0; WIDTH * HEIGHT];
        universe.cells[0] = 4;
        assert!(!universe.stable(), "Universe with a cell having 4 grains should be unstable");
    }

    #[test]
    fn test_tick_topples_single_cell() {
        let mut universe = Universe::new();
        universe.cells = vec![0; WIDTH * HEIGHT];

        // Place 4 grains in the center
        let center_row = HEIGHT / 2;
        let center_col = WIDTH / 2;
        let center_idx = center_row * WIDTH + center_col;
        universe.cells[center_idx] = 4;

        universe.tick();

        // After toppling, center should have 0 and neighbors should each have 1
        assert_eq!(universe.cells[center_idx], 0);
        assert_eq!(universe.cells[center_idx - WIDTH], 1); // top
        assert_eq!(universe.cells[center_idx + WIDTH], 1); // bottom
        assert_eq!(universe.cells[center_idx - 1], 1);     // left
        assert_eq!(universe.cells[center_idx + 1], 1);     // right
    }

    #[test]
    fn test_tick_top_left_corner() {
        let mut universe = Universe::new();
        universe.cells = vec![0; WIDTH * HEIGHT];

        // Place 4 grains in top-left corner (0, 0)
        universe.cells[0] = 4;

        universe.tick();

        // After toppling, corner should have 0
        assert_eq!(universe.cells[0], 0);
        // Only right and bottom neighbors should get grains
        assert_eq!(universe.cells[1], 1);        // right
        assert_eq!(universe.cells[WIDTH], 1);    // bottom
    }

    #[test]
    fn test_tick_top_right_corner() {
        let mut universe = Universe::new();
        universe.cells = vec![0; WIDTH * HEIGHT];

        // Place 4 grains in top-right corner (0, WIDTH-1)
        let idx = WIDTH - 1;
        universe.cells[idx] = 4;

        universe.tick();

        // After toppling, corner should have 0
        assert_eq!(universe.cells[idx], 0);
        // Only left and bottom neighbors should get grains
        assert_eq!(universe.cells[idx - 1], 1);        // left
        assert_eq!(universe.cells[idx + WIDTH], 1);    // bottom
    }

    #[test]
    fn test_tick_bottom_left_corner() {
        let mut universe = Universe::new();
        universe.cells = vec![0; WIDTH * HEIGHT];

        // Place 4 grains in bottom-left corner
        let idx = (HEIGHT - 1) * WIDTH;
        universe.cells[idx] = 4;

        universe.tick();

        // After toppling, corner should have 0
        assert_eq!(universe.cells[idx], 0);
        // Only right and top neighbors should get grains
        assert_eq!(universe.cells[idx + 1], 1);        // right
        assert_eq!(universe.cells[idx - WIDTH], 1);    // top
    }

    #[test]
    fn test_tick_bottom_right_corner() {
        let mut universe = Universe::new();
        universe.cells = vec![0; WIDTH * HEIGHT];

        // Place 4 grains in bottom-right corner
        let idx = WIDTH * HEIGHT - 1;
        universe.cells[idx] = 4;

        universe.tick();

        // After toppling, corner should have 0
        assert_eq!(universe.cells[idx], 0);
        // Only left and top neighbors should get grains
        assert_eq!(universe.cells[idx - 1], 1);        // left
        assert_eq!(universe.cells[idx - WIDTH], 1);    // top
    }

    #[test]
    fn test_tick_multiple_unstable_cells() {
        let mut universe = Universe::new();
        universe.cells = vec![0; WIDTH * HEIGHT];

        // Place 4 grains in two adjacent cells
        let idx1 = 100;
        let idx2 = 101;
        universe.cells[idx1] = 4;
        universe.cells[idx2] = 4;

        universe.tick();

        // Both cells should be toppled
        assert!(universe.cells[idx1] < 4);
        assert!(universe.cells[idx2] < 4);
    }

    #[test]
    fn test_tick_with_value_greater_than_four() {
        let mut universe = Universe::new();
        universe.cells = vec![0; WIDTH * HEIGHT];

        // Place 8 grains in center
        let center_idx = (HEIGHT / 2) * WIDTH + (WIDTH / 2);
        universe.cells[center_idx] = 8;

        universe.tick();

        // After one tick, center should have 4 (8 - 4)
        assert_eq!(universe.cells[center_idx], 4);
        // Neighbors should each have 1
        assert_eq!(universe.cells[center_idx - WIDTH], 1);
        assert_eq!(universe.cells[center_idx + WIDTH], 1);
        assert_eq!(universe.cells[center_idx - 1], 1);
        assert_eq!(universe.cells[center_idx + 1], 1);
    }

    #[test]
    fn test_cells_returns_copy() {
        let universe = Universe::new();
        let cells1 = universe.cells();
        let cells2 = universe.cells();

        // Should return a new Vec each time, not the same reference
        assert_eq!(cells1.len(), cells2.len());
        for i in 0..cells1.len() {
            assert_eq!(cells1[i], cells2[i]);
        }
    }

    #[test]
    fn test_width_and_height_getters() {
        let universe = Universe::new();
        assert_eq!(universe.width(), WIDTH);
        assert_eq!(universe.height(), HEIGHT);
    }

    #[test]
    fn test_tick_preserves_total_grains_in_interior() {
        let mut universe = Universe::new();
        universe.cells = vec![3; WIDTH * HEIGHT];

        // Set a single cell to 4 in the interior (not on edge)
        let center_idx = (HEIGHT / 2) * WIDTH + (WIDTH / 2);
        universe.cells[center_idx] = 4;

        // Count total grains before
        let total_before: usize = universe.cells.iter().sum();

        universe.tick();

        // Count total grains after
        let total_after: usize = universe.cells.iter().sum();

        // Total should be preserved for interior cells
        assert_eq!(total_before, total_after);
    }
}
