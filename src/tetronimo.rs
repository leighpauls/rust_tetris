use block::Block;
use graphics::Context;
use field::FieldDrawParams;
use opengl_graphics::GlGraphics;
use Trans2D;
use std::ops::Add;

pub enum RotDir {
    CW,
    CCW,
}

struct Rot0;

enum Rot2 {
    Start,
    Reverse,
}

enum Rot4 {
    Start,
    CW,
    Reverse,
    CCW,
}

enum Shape {
    O,
    I(Rot2),
    Z(Rot2),
    S(Rot2),
    L(Rot4),
    J(Rot4),
    T(Rot4),
}

impl Shape {
    fn rotate(&self, dir: RotDir) -> Shape {
        match self {
            &Shape::O => Shape::O,
            &Shape::I(ref r) => Shape::I(r.rotate()),
            &Shape::Z(ref r) => Shape::Z(r.rotate()),
            &Shape::S(ref r) => Shape::S(r.rotate()),
            &Shape::L(ref r) => Shape::L(r.rotate(dir)),
            &Shape::J(ref r) => Shape::J(r.rotate(dir)),
            &Shape::T(ref r) => Shape::T(r.rotate(dir)),
        }
    }
}

impl Rot2 {
    fn rotate(&self) -> Self {
        match self {
            &Rot2::Start => Rot2::Reverse,
            &Rot2::Reverse => Rot2::Start,
        }
    }
}

impl Rot4 {
    fn rotate(&self, dir: RotDir) -> Self {
        match dir {
            RotDir::CW => {
                match self {
                    &Rot4::Start => Rot4::CW,
                    &Rot4::CW => Rot4::Reverse,
                    &Rot4::Reverse => Rot4::CCW,
                    &Rot4::CCW => Rot4::Start,
                }
            }
            RotDir::CCW => {
                match self {
                    &Rot4::Start => Rot4::CCW,
                    &Rot4::CCW => Rot4::Reverse,
                    &Rot4::Reverse => Rot4::CW,
                    &Rot4::CW => Rot4::Start,
                }
            }

        }
    }
}

fn block_offsets(shape: &Shape) -> [Trans2D; 4] {
    match shape {
        &Shape::O => [(0, 0), (0, -1), (1, 0), (1, -1)],
        &Shape::I(ref r) => {
            match r {
                &Rot2::Start => [(-1, 0), (0, 0), (1, 0), (2, 0)],
                &Rot2::Reverse => [(0, 1), (0, 0), (0, -2), (0, -2)],
            }
        }
        &Shape::Z(_) => [(0, 0), (0, -1), (1, 0), (1, -1)],
        &Shape::S(_) => [(0, 0), (0, -1), (1, 0), (1, -1)],
        &Shape::L(_) => [(0, 0), (0, -1), (1, 0), (1, -1)],
        &Shape::J(_) => [(0, 0), (0, -1), (1, 0), (1, -1)],
        &Shape::T(ref r) => {
            match r {
                &Rot4::Start => [(-1, 0), (0, 0), (1, 0), (0, -1)],
                &Rot4::CW => [(0, -1), (0, 0), (0, 1), (1, 0)],
                &Rot4::Reverse => [(-1, 0), (0, 0), (1, 0), (0, 1)],
                &Rot4::CCW => [(0, -1), (0, 0), (0, 1), (-1, 0)],
            }
        }
    }
}

fn new_blocks_for_offsets(offsets: [Trans2D; 4]) -> [Block; 4] {
    let origin = (4, 1);
    [Block::from_trans(offsets[0], origin),
     Block::from_trans(offsets[1], origin),
     Block::from_trans(offsets[2], origin),
     Block::from_trans(offsets[3], origin)]
}

pub struct Tetromino {
    blocks: [Block; 4],
    shape: Shape,
}

impl Tetromino {
    pub fn new_t() -> Tetromino {
        let shape = Shape::T(Rot4::Start);
        Tetromino {
            blocks: new_blocks_for_offsets(block_offsets(&shape)),
            shape: shape,
        }
    }

    pub fn draw(&self, c: &Context, params: &FieldDrawParams, gl: &mut GlGraphics) {
        for block in &self.blocks {
            block.draw(&c, &params, gl);
        }
    }

    pub fn move_blocks(&mut self, trans: &Trans2D) {
        for block in &mut self.blocks {
            block.move_block(trans);
        }
    }

    pub fn rotate_blocks(&mut self, dir: RotDir) {
        let cur_offsets = block_offsets(&self.shape);
        let b = self.blocks[0].pos();
        let c = cur_offsets[0];
        let origin = (b.0 - c.0, b.1 - c.1);

        let new_shape = self.shape.rotate(dir);
        let new_offsets = block_offsets(&new_shape);
        for i in 0..4 {
            self.blocks[i].jump_to(new_offsets[i], origin);
        }
        self.shape = new_shape;
    }

    pub fn blocks(&self) -> &[Block; 4] {
        &self.blocks
    }
}
