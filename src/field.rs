
use piston::input::RenderArgs;

use block::Block;
use tetronimo::Tetromino;
use graphics::Context;
use opengl_graphics::GlGraphics;
use tetronimo::RotDir;

use Trans2D;
use BLACK;

pub struct FieldDrawParams {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub line_width: f64,
    pub block_size: f64,
}

impl FieldDrawParams {
    fn new(args: &RenderArgs) -> Self {
        let height = args.height as f64 - 20.0;
        let block_size = height / 22.0;
        FieldDrawParams {
            x: args.width as f64 / 2.0 - block_size * 5.0,
            y: 10.0,
            width: block_size * 10.0,
            height: height,
            block_size: block_size,
            line_width: height / 440.0,
        }
    }
}

pub struct Field {
    blocks: Vec<Block>,
    cur_tetromino: Option<Tetromino>,
}

impl Field {
    pub fn new() -> Field {
        Field {
            blocks: vec![],
            cur_tetromino: None,
        }
    }

    pub fn new_cur_tetromino(&mut self) {
        if let Some(ref t) = self.cur_tetromino {
            self.blocks.extend_from_slice(t.blocks());
        }
        self.cur_tetromino = Some(Tetromino::new_t());
    }

    pub fn move_block(&mut self, trans: &Trans2D) {
        if let Some(ref mut t) = self.cur_tetromino {
            t.move_blocks(trans);
        }
    }

    pub fn rotate_block(&mut self, dir: RotDir) {
        if let Some(ref mut t) = self.cur_tetromino {
            t.rotate_blocks(dir);
        }
    }

    pub fn draw(&self, render_args: &RenderArgs, c: &Context, gl: &mut GlGraphics) {

        let params = FieldDrawParams::new(render_args);

        self.draw_field_lines(c, gl, &params);

        for block in &self.blocks {
            block.draw(c, &params, gl);
        }

        if let &Some(ref t) = &self.cur_tetromino {
            t.draw(c, &params, gl);
        }

    }

    fn draw_field_lines(&self, c: &Context, gl: &mut GlGraphics, params: &FieldDrawParams) {
        use graphics::{Line, Transformed};
        let line = Line::new_round(BLACK, params.line_width);
        for y_idx in 2..23 {
            line.draw([0.0, 0.0, params.width, 0.0],
                      &c.draw_state,
                      c.transform
                          .trans(params.x, params.y)
                          .trans(0.0, y_idx as f64 * params.block_size),
                      gl);
        }
        for x_idx in 0..11 {
            line.draw([0.0, 0.0, 0.0, params.height - params.block_size * 2.0],
                      &c.draw_state,
                      c.transform
                          .trans(params.x, params.y + params.block_size * 2.0)
                          .trans(x_idx as f64 * params.block_size, 0.0),
                      gl);

        }
    }
}
