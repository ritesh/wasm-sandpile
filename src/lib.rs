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
        const WIDTH: usize = 110;
        const HEIGHT: usize = 110;
        let mut rng = rand::thread_rng();
        let between: Uniform<usize> = Uniform::from(0..10);
        let cells = Array2D::filled_by_row_major(|| between.sample(&mut rng), WIDTH, HEIGHT);
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
            // print!("Unstable!");
            self.topple()
        } else {
            print!("Stable!.......");
            println!("{:?}", self.cells());
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
        // let h: usize = self.height();
        // let w: usize = self.width();
        const h: usize = 110;
        const w: usize = 110;
        //TODO: pick cells at random, rather than from 0,0 to W, H
        for i in 0..=h {
            for j in 0..=w {
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
                            (0, w) => {
                                //Top right corner
                                let n1 = self.cells.get(0, w - 1);
                                let n2 = self.cells.get(1, w);
                                let _ = next.set(0, w - 1, n1.unwrap_or(&0) + 1);
                                let _ = next.set(1, w, n2.unwrap_or(&0) + 1);
                                continue;
                            }
                            (h, 0) => {
                                //Bottom left corner
                                let n1 = self.cells.get(h - 1, 0);
                                let n2 = self.cells.get(h, 1);
                                let _ = next.set(h - 1, 0, n1.unwrap_or(&0) + 1);
                                let _ = next.set(h, 1, n2.unwrap_or(&0) + 1);
                                continue;
                            }
                            (h, w) => {
                                //Bottom right corner
                                let n1 = self.cells.get(h - 1, w);
                                let n2 = self.cells.get(h, w - 1);
                                let _ = next.set(h - 1, w, n1.unwrap_or(&0) + 1);
                                let _ = next.set(h, w - 1, n2.unwrap_or(&0) + 1);
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
                                let _ = next.set(1, j + 1, n3.unwrap_or(&0) + 1);
                                continue;
                            }
                            (h, j) => {
                                //Bottom edge
                                let n1 = self.cells.get(h, j - 1);
                                let n2 = self.cells.get(h, j + 1);
                                let n3 = self.cells.get(h - 1, j);
                                let _ = next.set(h, j - 1, n1.unwrap_or(&0) + 1);
                                let _ = next.set(h, j + 1, n2.unwrap_or(&0) + 1);
                                let _ = next.set(h - 1, j, n3.unwrap_or(&0) + 1);
                                continue;
                            }
                            (i, w) => {
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
    fn tick() {
        let mut u = Universe::new();
        loop {
            u.tick();
        }
    }
    // #[test]
    // fn neigbour_test() {
    //     todo!()
    // }
}
