mod utils;
extern crate colored;
extern crate rand;

use crate::rand::distributions::Distribution;
use array2d::Array2D;
use rand::distributions::Uniform;
use rand::Rng;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Universe {
    width: usize,
    height: usize,
    cells: Array2D<usize>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        const width: usize = 110;
        const height: usize = 110;
        let mut rng = rand::thread_rng();
        let between: Uniform<usize> = Uniform::from(0..10);
        let cells = Array2D::filled_by_row_major(|| between.sample(&mut rng), width, height);
        println!("Cells {:?}", cells);
        Universe {
            width,
            height,
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
            self.topple()
        } else {
            return;
        }
        let mut next = self.cells.clone();
        //Pick a random cell and add 1
        let mut rng = rand::thread_rng();
        //TODO uniform distribution:
        //https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html
        let randomx: usize = rng.gen_range(0..self.width);
        let randomy: usize = rng.gen_range(0..self.height);
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
        const W: usize = 110;
        const H: usize = 110;
        for i in 0..W {
            for j in 0..H {
                if let Some(v) = next.get_mut(i, j) {
                    //Unstable
                    if *v >= 4 {
                        match (i, j) {
                            //corners
                            (0, 0) => {
                                //Top corner
                            }
                            (0, W) => {
                                //Top right corner
                            }
                            (0, H) => {
                                //Bottom left corner
                            }
                            (W, H) => {
                                //Bottom right corner
                            }
                            //Edges
                            (1..=W, 0) => {
                                //Top edge
                            }
                            (W, 1..=H) => {
                                //Right edge
                            }
                            (0, 1..=H) => {
                                //Left edge
                            }
                            (1..=W, H) => {
                                //Bottom edge
                            }
                            (_, _) => {}
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
    fn tick() {
        let mut u = Universe::new();
        for _i in 0..1000 {
            u.tick();
            println!("{:?}", u.cells());
        }
    }
    // #[test]
    // fn neigbour_test() {
    //     todo!()
    // }
}
