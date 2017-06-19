use block::Block;
use graphics::Context;
use field::FieldDrawParams;
use opengl_graphics::GlGraphics;
use trans2d::Trans2D;
use shape::Shape;

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
    rot: Rot4,
}

fn block_offsets(shape: &Shape, rot: &Rot4) -> [Trans2D; 4] {
    fn b(positions: [(i32, i32); 4]) -> [Trans2D; 4] {
        [Trans2D::from_tup(positions[0]),
         Trans2D::from_tup(positions[1]),
         Trans2D::from_tup(positions[2]),
         Trans2D::from_tup(positions[3])]
    }

    match *shape {
        Shape::O => b([(0, 0), (0, -1), (1, 0), (1, -1)]),
        Shape::I => {
            match *rot {
                Rot4::Start => b([(-1, 0), (0, 0), (1, 0), (2, 0)]),
                Rot4::CW => b([(1, -1), (1, 0), (1, 1), (1, 2)]),
                Rot4::Reverse => b([(-1, 1), (0, 1), (1, 1), (2, 1)]),
                Rot4::CCW => b([(0, -1), (0, 0), (0, 1), (0, 2)]),
            }
        }
        Shape::Z => {
            match *rot {
                Rot4::Start => b([(-1, -1), (0, -1), (0, 0), (1, 0)]),
                Rot4::CW => b([(1, -1), (1, 0), (0, 0), (0, 1)]),
                Rot4::Reverse => b([(-1, 0), (0, 0), (0, 1), (1, 1)]),
                Rot4::CCW => b([(0, -1), (0, 0), (-1, 0), (-1, 1)]),
            }
        }
        Shape::S => {
            match *rot {
                Rot4::Start => b([(-1, 0), (0, 0), (0, -1), (1, -1)]),
                Rot4::CW => b([(0, -1), (0, 0), (1, 0), (1, 1)]),
                Rot4::Reverse => b([(-1, 1), (0, 1), (0, 0), (1, 0)]),
                Rot4::CCW => b([(-1, -1), (-1, 0), (0, 0), (0, 1)]),
            }
        }
        Shape::L => {
            match *rot {
                Rot4::Start => b([(-1, 0), (0, 0), (1, 0), (1, -1)]),
                Rot4::CW => b([(0, -1), (0, 0), (0, 1), (1, 1)]),
                Rot4::Reverse => b([(-1, 1), (-1, 0), (0, 0), (1, 0)]),
                Rot4::CCW => b([(-1, -1), (0, -1), (0, 0), (0, 1)]),
            }
        }
        Shape::J => {
            match *rot {
                Rot4::Start => b([(-1, -1), (-1, 0), (0, 0), (1, 0)]),
                Rot4::CW => b([(1, -1), (0, -1), (0, 0), (0, 1)]),
                Rot4::Reverse => b([(-1, 0), (0, 0), (1, 0), (1, 1)]),
                Rot4::CCW => b([(-1, 1), (0, 1), (0, 0), (0, -1)]),
            }
        }
        Shape::T => {
            match *rot {
                Rot4::Start => b([(-1, 0), (0, 0), (1, 0), (0, -1)]),
                Rot4::CW => b([(0, -1), (0, 0), (0, 1), (1, 0)]),
                Rot4::Reverse => b([(-1, 0), (0, 0), (1, 0), (0, 1)]),
                Rot4::CCW => b([(0, -1), (0, 0), (0, 1), (-1, 0)]),
            }
        }
    }
}

impl Tetromino {
    pub fn new(shape: Shape) -> Self {
        let rot = Rot4::Start;
        Self {
            blocks: new_blocks_for_offsets(&block_offsets(&shape, &rot)),
            shape: shape,
            rot: rot,
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
        let new_rot = self.rot.rotate(dir);
        let origin = &self.origin();
        self.new_rot_origin(new_rot, origin);
    }

    pub fn reset_pos_rot(&mut self, new_origin: &Trans2D) {
        self.new_rot_origin(Rot4::Start, new_origin);
    }

    pub fn blocks(&self) -> &[Block; 4] {
        &self.blocks
    }

    fn origin(&self) -> Trans2D {
        let cur_offsets = block_offsets(&self.shape, &self.rot);
        let b = self.blocks[0].pos();
        let c = &cur_offsets[0];
        b.trans(&c.invert())
    }

    fn new_rot_origin(&mut self, new_rot: Rot4, new_origin: &Trans2D) {
        let new_offsets = block_offsets(&self.shape, &new_rot);
        for i in 0..4 {
            self.blocks[i].jump_to(&new_offsets[i], new_origin);
        }
        self.rot = new_rot;
    }
}
