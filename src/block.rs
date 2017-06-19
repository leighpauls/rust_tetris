
use graphics::context::Context;
use opengl_graphics::GlGraphics;
use graphics::{Transformed, rectangle};
use graphics::types::Color;

use field::FieldDrawParams;

use RED;
use trans2d::Trans2D;

#[derive(Clone)]
pub struct Block {
    color: Color,
    pos : Trans2D,
}

impl Block {
    pub fn new(pos: Trans2D) -> Block {
        Block {
            color: RED,
            pos : pos,
        }
    }

    pub fn draw(&self, c: &Context, params: &FieldDrawParams, gl: &mut GlGraphics) {

        rectangle(self.color,
                  rectangle::square(0.0, 0.0, params.block_size),
                  c.transform
                      .trans(params.x, params.y)
                      .trans(self.pos.x as f64 * params.block_size,
                             self.pos.y as f64 * params.block_size),
                  gl);

    }

    pub fn move_block(&mut self, trans: &Trans2D) {
        self.pos = self.pos.trans(trans);
    }

    pub fn jump_to(&mut self, location: &Trans2D, origin: &Trans2D) {
        self.pos = origin.trans(location);
    }

    pub fn pos(&self) -> &Trans2D {
        &self.pos
    }
}
