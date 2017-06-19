
use shape::{Shape, ALL_SHAPES};
use std::vec::Vec;
use rand::{thread_rng, Rng};

pub struct ShapeBag {
    remaining: Vec<Shape>,
}

impl ShapeBag {
    pub fn new() -> Self {
        ShapeBag { remaining: vec![] }
    }

    pub fn next_shape(&mut self) -> Shape {
        if self.remaining.is_empty() {
            self.remaining.extend_from_slice(&ALL_SHAPES);
        }
        let idx = thread_rng().gen::<usize>() % self.remaining.len();
        self.remaining.remove(idx)
    }
}
