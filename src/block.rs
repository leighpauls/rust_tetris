
use graphics::context::Context;
use opengl_graphics::GlGraphics;
use graphics::{Transformed, rectangle};
use graphics::types::Color;

use field::FieldDrawParams;

use RED;
use Trans2D;

#[derive(Clone)]
pub struct Block {
    color: Color,
    x: i32,
    y: i32,
}

impl Block {
    pub fn from_trans(location: Trans2D, origin: Trans2D) -> Block {
        Self::new(location.0 + origin.0, location.1 + origin.1)
    }

    pub fn new(x: i32, y: i32) -> Block {
        Block {
            color: RED,
            x: x,
            y: y,
        }
    }

    pub fn draw(&self, c: &Context, params: &FieldDrawParams, gl: &mut GlGraphics) {

        rectangle(self.color,
                  rectangle::square(0.0, 0.0, params.block_size),
                  c.transform
                      .trans(params.x, params.y)
                      .trans(self.x as f64 * params.block_size,
                             self.y as f64 * params.block_size),
                  gl);

    }

    pub fn move_block(&mut self, trans: &Trans2D) {
        self.x += trans.0;
        self.y += trans.1;
    }

    pub fn jump_to(&mut self, location: Trans2D, origin: Trans2D) {
        self.x = location.0 + origin.0;
        self.y = location.1 + origin.1;
    }

    pub fn pos(&self) -> Trans2D {
        (self.x, self.y)
    }
}
