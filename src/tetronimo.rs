use block::Block;
use graphics::Context;
use field::FieldDrawParams;
use opengl_graphics::GlGraphics;
use trans2d::Trans2D;
use std::ops::Add;

pub enum RotDir {
    CW,
    CCW,
}

enum Rot4 {
    Start,
    CW,
    Reverse,
    CCW,
}

enum Shape {
    O,
    I(Rot4),
    Z(Rot4),
    S(Rot4),
    L(Rot4),
    J(Rot4),
    T(Rot4),
}

impl Shape {
    fn rotate(&self, dir: RotDir) -> Shape {
        match self {
            &Shape::O => Shape::O,
            &Shape::I(ref r) => Shape::I(r.rotate(dir)),
            &Shape::Z(ref r) => Shape::Z(r.rotate(dir)),
            &Shape::S(ref r) => Shape::S(r.rotate(dir)),
            &Shape::L(ref r) => Shape::L(r.rotate(dir)),
            &Shape::J(ref r) => Shape::J(r.rotate(dir)),
            &Shape::T(ref r) => Shape::T(r.rotate(dir)),
        }
    }

    fn block_offsets(&self) -> [Trans2D; 4] {

        fn b(positions: [(i32, i32); 4]) -> [Trans2D; 4] {
            [Trans2D::from_tup(positions[0]),
             Trans2D::from_tup(positions[1]),
             Trans2D::from_tup(positions[2]),
             Trans2D::from_tup(positions[3])]
        }

        match self {
            &Shape::O => b([(0, 0), (0, -1), (1, 0), (1, -1)]),
            &Shape::I(ref r) => {
                match r {
                    &Rot4::Start => b([(-1, 0), (0, 0), (1, 0), (2, 0)]),
                    &Rot4::CW => b([(1, -1), (1, 0), (1, 1), (1, 2)]),
                    &Rot4::Reverse => b([(-1, 1), (0, 1), (1, 1), (2, 1)]),
                    &Rot4::CCW => b([(0, -1), (0, 0), (0, 1), (0, 2)]),
                }
            },
            &Shape::Z(ref r) => {
                match r {
                    &Rot4::Start => b([(-1, -1), (0, -1), (0, 0), (1, 0)]),
                    &Rot4::CW => b([(1, -1), (1, 0), (0, 0), (0, 1)]),
                    &Rot4::Reverse => b([(-1, 0), (0, 0), (0, 1), (1, 1)]),
                    &Rot4::CCW => b([(0, -1), (0, 0), (-1, 0), (-1, 1)]),
                }
            },
            &Shape::S(ref r) => {
                match r {
                    &Rot4::Start => b([(-1, 0), (0, 0), (0, -1), (1, -1)]),
                    &Rot4::CW => b([(0, -1), (0, 0), (1, 0), (1, 1)]),
                    &Rot4::Reverse => b([(-1, 1), (0, 1), (0, 0), (1, 0)]),
                    &Rot4::CCW => b([(-1, -1), (-1, 0), (0, 0), (0, 1)]),
                }
            },
            &Shape::L(ref r) => {
                match r {
                    &Rot4::Start => b([(-1, 0), (0, 0), (1, 0), (1, -1)]),
                    &Rot4::CW => b([(0, -1), (0, 0), (0, 1), (1, 1)]),
                    &Rot4::Reverse => b([(-1, 1), (-1, 0), (0, 0), (1, 0)]),
                    &Rot4::CCW => b([(-1, -1), (0, -1), (0, 0), (0, 1)]),
                }
            },
            &Shape::J(ref r) => {
                match r {
                    &Rot4::Start => b([(-1, -1), (-1, 0), (0, 0), (1, 0)]),
                    &Rot4::CW => b([(1, -1), (0, -1), (0, 0), (0, 1)]),
                    &Rot4::Reverse => b([(-1, 0), (0, 0), (1, 0), (1, 1)]),
                    &Rot4::CCW => b([(-1, 1), (0, 1), (0, 0), (0, -1)]),
                }
            },
            &Shape::T(ref r) => {
                match r {
                    &Rot4::Start => b([(-1, 0), (0, 0), (1, 0), (0, -1)]),
                    &Rot4::CW => b([(0, -1), (0, 0), (0, 1), (1, 0)]),
                    &Rot4::Reverse => b([(-1, 0), (0, 0), (1, 0), (0, 1)]),
                    &Rot4::CCW => b([(0, -1), (0, 0), (0, 1), (-1, 0)]),
                }
            }
        }
    }

    fn default_rotation(&self) -> Shape {
        match self {
            &Shape::O => Shape::O,
            &Shape::I(_) => Shape::I(Rot4::Start),
            &Shape::Z(_) => Shape::Z(Rot4::Start),
            &Shape::S(_) => Shape::S(Rot4::Start),
            &Shape::L(_) => Shape::L(Rot4::Start),
            &Shape::J(_) => Shape::J(Rot4::Start),
            &Shape::T(_) => Shape::T(Rot4::Start),
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


fn new_blocks_for_offsets(offsets: &[Trans2D; 4]) -> [Block; 4] {
    [Block::new(offsets[0].clone()),
     Block::new(offsets[1].clone()),
     Block::new(offsets[2].clone()),
     Block::new(offsets[3].clone())]
}

pub struct Tetromino {
    blocks: [Block; 4],
    shape: Shape,
}

impl Tetromino {
    pub fn new_o() -> Self {
        let shape = Shape::O;
        Self {
            blocks: new_blocks_for_offsets(&shape.block_offsets()),
            shape: shape,
        }
    }
    pub fn new_i() -> Self {
        let shape = Shape::I(Rot4::Start);
        Self {
            blocks: new_blocks_for_offsets(&shape.block_offsets()),
            shape: shape,
        }
    }
    pub fn new_z() -> Self {
        let shape = Shape::Z(Rot4::Start);
        Self {
            blocks: new_blocks_for_offsets(&shape.block_offsets()),
            shape: shape,
        }
    }
    pub fn new_s() -> Self {
        let shape = Shape::S(Rot4::Start);
        Self {
            blocks: new_blocks_for_offsets(&shape.block_offsets()),
            shape: shape,
        }
    }
    pub fn new_l() -> Self {
        let shape = Shape::L(Rot4::Start);
        Self {
            blocks: new_blocks_for_offsets(&shape.block_offsets()),
            shape: shape,
        }
    }
    pub fn new_j() -> Self {
        let shape = Shape::J(Rot4::Start);
        Self {
            blocks: new_blocks_for_offsets(&shape.block_offsets()),
            shape: shape,
        }
    }
    
    pub fn new_t() -> Self {
        let shape = Shape::T(Rot4::Start);
        Self {
            blocks: new_blocks_for_offsets(&shape.block_offsets()),
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

    pub fn jump_to(&mut self, new_origin: &Trans2D) {
        let old_origin = self.origin();
        self.move_blocks(&new_origin.trans(&old_origin.invert()));
    }

    pub fn rotate_blocks(&mut self, dir: RotDir) {
        let mut new_shape = self.shape.rotate(dir);
        let origin = &self.origin();
        self.new_shape_origin(new_shape, origin);
    }

    pub fn reset_pos_rot(&mut self, new_origin: &Trans2D) {
        let mut new_shape = self.shape.default_rotation();
        self.new_shape_origin(new_shape, new_origin);
    }

    pub fn blocks(&self) -> &[Block; 4] {
        &self.blocks
    }

    fn origin(&self) -> Trans2D {
        let cur_offsets = self.shape.block_offsets();
        let b = self.blocks[0].pos();
        let c = &cur_offsets[0];
        b.trans(&c.invert())
    }

    fn new_shape_origin(&mut self, new_shape: Shape, new_origin: &Trans2D) {
        let old_origin = self.origin();
        let new_offsets = new_shape.block_offsets();
        for i in 0..4 {
            self.blocks[i].jump_to(&new_offsets[i], new_origin);
        }
        self.shape = new_shape;
    }
}
