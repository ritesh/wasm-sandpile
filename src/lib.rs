use core::panic;
use rand::distr::Uniform;
use rand::prelude::Distribution;
use std::sync::{Arc, Mutex};

use wasm_bindgen::prelude::*;

const MAX_ITERATIONS: u32 = 10_000;
const WIDTH: usize = 110;
const HEIGHT: usize = 110;

#[wasm_bindgen]
pub struct Universe {
    width: usize,
    height: usize,
    cells: Arc<Mutex<Vec<usize>>>,
    //contains indices of unstable cells i.e. greater than or eq 4
    unstable: Arc<Mutex<Vec<usize>>>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let mut rng = rand::rng();
        let cells = Vec::<usize>::with_capacity(WIDTH * HEIGHT)
            .iter()
            .map(|_| Uniform::<usize>::new(0, 3).unwrap().sample(&mut rng))
            .collect();

        //  println!("Cells {:?}", cells);
        Universe {
            width: WIDTH,
            height: HEIGHT,
            cells: Arc::new(Mutex::new(cells)),
            unstable: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn tick(&mut self) {}

    pub fn stable(&self) -> bool {
        //If any of the cells have 4 or more, this configuration is unstable
        let cells = self.cells.lock().unwrap();
        let mut unstable = self.unstable.lock().unwrap();
        for (idx, elem) in cells.iter().enumerate() {
            if *elem >= 4 {
                unstable.push(idx)
            }
        }
        unstable.is_empty()
    }
    pub fn topple(&mut self) {
        let mut rng = rand::rng();
        while !self.stable() {
            let mut cells = self.cells.lock().unwrap();
            let mut unstable = self.unstable.lock().unwrap();
            //random number picker
            let unstable_len = self.unstable.lock().unwrap().len();
            let pick_unstable_idx = Uniform::new(0, unstable_len).unwrap().sample(&mut rng);
            let cell_idx = unstable[pick_unstable_idx];

            //Last step
            //probably not efficient
            unstable.remove(pick_unstable_idx);
        }
    }
}

// #[cfg(test)]
// mod tests {

//    }
