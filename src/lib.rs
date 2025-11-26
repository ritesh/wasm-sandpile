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
        // Step 1: Add one grain of sand to a uniformly random cell
        let mut rng = rand::rng();
        let dist = Uniform::<usize>::new(0, self.cells.len()).unwrap();
        let random_idx = dist.sample(&mut rng);
        self.cells[random_idx] += 1;

        // Step 2: Topple repeatedly until the configuration is stable
        while !self.stable() {
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
    fn test_tick_always_produces_stable_state() {
        let mut universe = Universe::new();
        universe.cells = vec![0; WIDTH * HEIGHT];

        // Test with all zeros - should add 1 grain and remain stable
        universe.tick();
        assert!(universe.stable(), "Universe should be stable after tick");

        // Test with some threes - should still be stable after tick
        universe.cells = vec![3; WIDTH * HEIGHT];
        universe.tick();
        assert!(universe.stable(), "Universe should be stable after tick");
    }

    #[test]
    fn test_tick_with_unstable_initial_state() {
        let mut universe = Universe::new();
        universe.cells = vec![0; WIDTH * HEIGHT];

        // Place 4 grains in the center
        let center_idx = (HEIGHT / 2) * WIDTH + (WIDTH / 2);
        universe.cells[center_idx] = 4;

        // Universe is initially unstable
        assert!(!universe.stable());

        universe.tick();

        // After tick, should be stable
        assert!(universe.stable(), "Universe should be stable after tick");
    }

    #[test]
    fn test_tick_with_multiple_unstable_cells() {
        let mut universe = Universe::new();
        universe.cells = vec![0; WIDTH * HEIGHT];

        // Place 4 grains in multiple cells
        universe.cells[100] = 4;
        universe.cells[101] = 4;
        universe.cells[200] = 5;

        // Universe is initially unstable
        assert!(!universe.stable());

        universe.tick();

        // After tick, should be stable
        assert!(universe.stable(), "Universe should be stable after tick");
        // All cells should have less than 4 grains
        for &cell in universe.cells.iter() {
            assert!(cell < 4, "Cell has {} grains, should be < 4", cell);
        }
    }

    #[test]
    fn test_tick_cascading_topple() {
        let mut universe = Universe::new();
        universe.cells = vec![0; WIDTH * HEIGHT];

        // Create a configuration that will cause cascading topples
        // Set up cells that will trigger each other
        let center_idx = (HEIGHT / 2) * WIDTH + (WIDTH / 2);
        universe.cells[center_idx] = 3;
        universe.cells[center_idx - WIDTH] = 3;
        universe.cells[center_idx + WIDTH] = 3;
        universe.cells[center_idx - 1] = 3;
        universe.cells[center_idx + 1] = 3;

        universe.tick();

        // After tick with cascading topples, should still be stable
        assert!(universe.stable(), "Universe should be stable after cascading topples");
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
    fn test_tick_increases_total_grains() {
        let mut universe = Universe::new();
        universe.cells = vec![0; WIDTH * HEIGHT];

        // Count total grains before
        let total_before: usize = universe.cells.iter().sum();

        universe.tick();

        // Count total grains after
        let total_after: usize = universe.cells.iter().sum();

        // Total should increase by 1 (we add 1 grain)
        // Note: grains can be lost at boundaries during toppling
        assert!(
            total_after >= total_before,
            "Total grains should not decrease"
        );
        assert!(
            total_after <= total_before + 1,
            "Total grains should increase by at most 1"
        );
    }
}
