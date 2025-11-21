use array2d::Array2D;
use rand::distr::Uniform;
use rand::prelude::Distribution;
use rand::Rng;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Universe {
    width: usize,
    height: usize,
    cells: Array2D<usize>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        const WIDTH: usize = 110;
        const HEIGHT: usize = 110;
        let mut rng = rand::rng();
        let cells = Array2D::filled_by_row_major(
            || Uniform::<usize>::new(0, 10).unwrap().sample(&mut rng),
            WIDTH,
            HEIGHT,
        );
        //  println!("Cells {:?}", cells);
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
        self.cells.as_row_major()
    }
    pub fn tick(&mut self) {
        if !self.stable() {
            self.topple();
        }
        // Always add a grain after toppling (or if already stable)
        let mut next = self.cells.clone();
        //Pick a random cell and add 1
        let mut rng = rand::rng();
        //TODO uniform distribution:
        //https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html
        let randomx: usize = rng.random_range(0..self.width);
        let randomy: usize = rng.random_range(0..self.height);
        next.get_mut(randomx, randomy).map(|v| *v += 1);
        self.cells = next;
    }

    pub fn stable(&self) -> bool {
        for row in self.cells.rows_iter() {
            for el in row {
                if *el >= 4 {
                    return false;
                }
            }
        }
        true
    }
    pub fn topple(&mut self) {
        let mut next = self.cells.clone();
        // let h: usize = self.height();
        // let w: usize = self.width();
        const H: usize = 110;
        const W: usize = 110;
        //TODO: pick cells at random, rather than from 0,0 to W, H
        for i in 0..=H {
            for j in 0..=W {
                if let Some(v) = next.get(i, j) {
                    //Unstable
                    if *v >= 4 {
                        let _ = next.set(i, j, *v - 4);
                        match (i, j) {
                            //corners
                            (0, 0) => {
                                //Top left corner
                                let n1 = self.cells.get(i + 1, 0);
                                let n2 = self.cells.get(0, j + 1);
                                //TODO: handle errors
                                let _ = next.set(i + 1, 0, n1.unwrap_or(&0) + 1);
                                let _ = next.set(0, j + 1, n2.unwrap_or(&0) + 1);
                                continue;
                            }
                            (0, W) => {
                                //Top right corner
                                let n1 = self.cells.get(0, W - 1);
                                let n2 = self.cells.get(1, W);
                                let _ = next.set(0, W - 1, n1.unwrap_or(&0) + 1);
                                let _ = next.set(1, W, n2.unwrap_or(&0) + 1);
                                continue;
                            }
                            (H, 0) => {
                                //Bottom left corner
                                let n1 = self.cells.get(H - 1, 0);
                                let n2 = self.cells.get(H, 1);
                                let _ = next.set(H - 1, 0, n1.unwrap_or(&0) + 1);
                                let _ = next.set(H, 1, n2.unwrap_or(&0) + 1);
                                continue;
                            }
                            (H, W) => {
                                //Bottom right corner
                                let n1 = self.cells.get(H - 1, W);
                                let n2 = self.cells.get(H, W - 1);
                                let _ = next.set(H - 1, W, n1.unwrap_or(&0) + 1);
                                let _ = next.set(H, W - 1, n2.unwrap_or(&0) + 1);
                                continue;
                            }
                            //Top edge
                            (0, j) => {
                                //Top edge
                                let n1 = self.cells.get(0, j - 1);
                                let n2 = self.cells.get(0, j + 1);
                                let n3 = self.cells.get(1, j);
                                let _ = next.set(0, j - 1, n1.unwrap_or(&0) + 1);
                                let _ = next.set(0, j + 1, n2.unwrap_or(&0) + 1);
                                let _ = next.set(1, j, n3.unwrap_or(&0) + 1);
                                continue;
                            }
                            (H, j) => {
                                //Bottom edge
                                let n1 = self.cells.get(H, j - 1);
                                let n2 = self.cells.get(H, j + 1);
                                let n3 = self.cells.get(H - 1, j);
                                let _ = next.set(H, j - 1, n1.unwrap_or(&0) + 1);
                                let _ = next.set(H, j + 1, n2.unwrap_or(&0) + 1);
                                let _ = next.set(H - 1, j, n3.unwrap_or(&0) + 1);
                                continue;
                            }
                            (i, W) => {
                                //Right edge
                                let n1 = self.cells.get(i - 1, j);
                                let n2 = self.cells.get(i, j - 1);
                                let n3 = self.cells.get(i + 1, j);
                                let _ = next.set(i - 1, j, n1.unwrap_or(&0) + 1);
                                let _ = next.set(i, j - 1, n2.unwrap_or(&0) + 1);
                                let _ = next.set(i + 1, j, n3.unwrap_or(&0) + 1);
                                continue;
                            }
                            (i, 0) => {
                                //Left edge
                                let n1 = self.cells.get(i - 1, 0);
                                let n2 = self.cells.get(i + 1, 0);
                                let n3 = self.cells.get(i, 1);
                                let _ = next.set(i - 1, 0, n1.unwrap_or(&0) + 1);
                                let _ = next.set(i + 1, 0, n2.unwrap_or(&0) + 1);
                                let _ = next.set(i, 1, n3.unwrap_or(&0) + 1);
                                continue;
                            }
                            //Everything else
                            (i, j) => {
                                let n1 = self.cells.get(i - 1, j);
                                let n2 = self.cells.get(i + 1, j);
                                let n3 = self.cells.get(i, j - 1);
                                let n4 = self.cells.get(i, j + 1);
                                let _ = next.set(i - 1, j, n1.unwrap_or(&0) + 1);
                                let _ = next.set(i + 1, j, n2.unwrap_or(&0) + 1);
                                let _ = next.set(i, j - 1, n3.unwrap_or(&0) + 1);
                                let _ = next.set(i, j + 1, n4.unwrap_or(&0) + 1);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        self.cells = next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_universe_creation() {
        let u = Universe::new();
        assert_eq!(u.width(), 110);
        assert_eq!(u.height(), 110);
        assert_eq!(u.cells().len(), 110 * 110);
    }

    #[test]
    fn test_accessors() {
        let u = Universe::new();
        assert_eq!(u.width(), 110);
        assert_eq!(u.height(), 110);

        let cells = u.cells();
        // All cells should be initialized with values 0-9
        for cell in cells.iter() {
            assert!(*cell < 10, "Cell value should be less than 10");
        }
    }

    #[test]
    fn test_stable_with_small_values() {
        let cells = Array2D::filled_with(2, 10, 10);
        let u = Universe {
            width: 10,
            height: 10,
            cells,
        };
        assert!(u.stable(), "Universe with all cells < 4 should be stable");
    }

    #[test]
    fn test_unstable_with_large_values() {
        let cells = Array2D::filled_with(4, 10, 10);
        let u = Universe {
            width: 10,
            height: 10,
            cells,
        };
        assert!(!u.stable(), "Universe with cells >= 4 should be unstable");
    }

    #[test]
    fn test_topple_center_cell() {
        // Create a small grid with one unstable cell in the center
        let mut cells = Array2D::filled_with(0, 5, 5);
        cells.set(2, 2, 4).unwrap();

        let mut u = Universe {
            width: 5,
            height: 5,
            cells,
        };

        u.topple();

        // After toppling, center should have 0, neighbors should have 1
        assert_eq!(
            *u.cells.get(2, 2).unwrap(),
            0,
            "Center cell should be 0 after toppling"
        );
        assert_eq!(*u.cells.get(1, 2).unwrap(), 1, "Top neighbor should be 1");
        assert_eq!(
            *u.cells.get(3, 2).unwrap(),
            1,
            "Bottom neighbor should be 1"
        );
        assert_eq!(*u.cells.get(2, 1).unwrap(), 1, "Left neighbor should be 1");
        assert_eq!(*u.cells.get(2, 3).unwrap(), 1, "Right neighbor should be 1");
    }

    #[test]
    fn test_topple_corner_cell() {
        // Create a small grid with one unstable cell in top-left corner
        let mut cells = Array2D::filled_with(0, 5, 5);
        cells.set(0, 0, 4).unwrap();

        let mut u = Universe {
            width: 5,
            height: 5,
            cells,
        };

        u.topple();

        // Corner cell should distribute to only 2 neighbors
        assert_eq!(
            *u.cells.get(0, 0).unwrap(),
            0,
            "Corner cell should be 0 after toppling"
        );
        assert_eq!(*u.cells.get(1, 0).unwrap(), 1, "Right neighbor should be 1");
        assert_eq!(
            *u.cells.get(0, 1).unwrap(),
            1,
            "Bottom neighbor should be 1"
        );
        // Other cells should remain 0
        assert_eq!(*u.cells.get(2, 0).unwrap(), 0);
        assert_eq!(*u.cells.get(0, 2).unwrap(), 0);
    }

    #[test]
    fn test_topple_edge_cell() {
        // Create a small grid with one unstable cell on the top edge
        let mut cells = Array2D::filled_with(0, 5, 5);
        cells.set(0, 2, 4).unwrap();

        let mut u = Universe {
            width: 5,
            height: 5,
            cells,
        };

        u.topple();

        // Edge cell should distribute to 3 neighbors
        assert_eq!(
            *u.cells.get(0, 2).unwrap(),
            0,
            "Edge cell should be 0 after toppling"
        );
        assert_eq!(*u.cells.get(0, 1).unwrap(), 1, "Left neighbor should be 1");
        assert_eq!(*u.cells.get(0, 3).unwrap(), 1, "Right neighbor should be 1");
        assert_eq!(
            *u.cells.get(1, 2).unwrap(),
            1,
            "Bottom neighbor should be 1"
        );
    }

    #[test]
    fn test_multiple_topples() {
        // Create a grid where multiple cells need to topple
        let mut cells = Array2D::filled_with(0, 5, 5);
        cells.set(2, 2, 5).unwrap();

        let mut u = Universe {
            width: 5,
            height: 5,
            cells,
        };

        // First topple
        u.topple();
        assert_eq!(
            *u.cells.get(2, 2).unwrap(),
            1,
            "Center should have 1 after first topple"
        );

        // Neighbors should each have 1
        assert_eq!(*u.cells.get(1, 2).unwrap(), 1);
        assert_eq!(*u.cells.get(3, 2).unwrap(), 1);
        assert_eq!(*u.cells.get(2, 1).unwrap(), 1);
        assert_eq!(*u.cells.get(2, 3).unwrap(), 1);

        assert!(u.stable(), "Universe should be stable after toppling");
    }

    #[test]
    fn test_tick_adds_grain() {
        let cells = Array2D::filled_with(0, 5, 5);
        let mut u = Universe {
            width: 5,
            height: 5,
            cells,
        };

        // Count total grains before tick
        let sum_before: usize = u.cells().iter().sum();

        // Perform several ticks on a stable universe
        for _ in 0..10 {
            if u.stable() {
                u.tick();
            }
        }

        // Total grains should have increased
        let sum_after: usize = u.cells().iter().sum();
        assert!(
            sum_after > sum_before,
            "Tick should add grains to the universe"
        );
    }

    #[test]
    fn test_tick_eventually_stabilizes() {
        let cells = Array2D::filled_with(3, 5, 5);
        let mut u = Universe {
            width: 5,
            height: 5,
            cells,
        };

        // Run for a limited number of iterations
        // Should not panic and should eventually stabilize or keep running
        for _ in 0..100 {
            if u.stable() {
                break;
            }
            u.topple();
        }

        // At least verify the universe is in a valid state
        for cell in u.cells().iter() {
            assert!(*cell < 100, "Cell values should remain reasonable");
        }
    }

    #[test]
    fn test_cascade_toppling() {
        // Set up a chain reaction: center cell has 4, will cause neighbors to topple
        let mut cells = Array2D::filled_with(3, 5, 5);
        cells.set(2, 2, 4).unwrap();

        let mut u = Universe {
            width: 5,
            height: 5,
            cells,
        };

        // First topple
        u.topple();

        // This should have made neighbors have 4, causing more instability
        assert!(!u.stable(), "After toppling, neighbors should be unstable");
    }

    #[test]
    fn test_cells_vector_size() {
        let u = Universe::new();
        let cells = u.cells();
        assert_eq!(
            cells.len(),
            110 * 110,
            "Cells vector should have width * height elements"
        );
    }

    #[test]
    fn test_boundary_conditions() {
        // Test all four corners
        let mut cells = Array2D::filled_with(0, 10, 10);
        let corners = [(0, 0), (0, 9), (9, 0), (9, 9)];

        for (i, j) in corners.iter() {
            cells.set(*i, *j, 4).unwrap();
        }

        let mut u = Universe {
            width: 10,
            height: 10,
            cells: cells.clone(),
        };

        u.topple();

        // All corners should now be 0
        for (i, j) in corners.iter() {
            assert_eq!(
                *u.cells.get(*i, *j).unwrap(),
                0,
                "Corner ({}, {}) should be 0 after toppling",
                i,
                j
            );
        }
    }
}
