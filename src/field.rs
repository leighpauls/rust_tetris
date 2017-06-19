
use piston::input::RenderArgs;

use block::Block;
use tetronimo::Tetromino;
use graphics::Context;
use opengl_graphics::GlGraphics;
use tetronimo::RotDir;

use trans2d::Trans2D;
use BLACK;

pub struct FieldDrawParams {
    pub x: f64,
    pub y: f64,
    pub line_width: f64,
    pub block_size: f64,
    pub cols: i32,
    pub over_rows: i32,
    pub body_rows: i32,
}

impl FieldDrawParams {
    fn new(args: &RenderArgs,
           cols: i32,
           over_rows: i32,
           body_rows: i32,
           center_x_blocks: f64,
           center_y_blocks: f64)
           -> Self {
        let block_size = args.height as f64 / 25.0;
        let field_width = cols as f64 * block_size;
        let field_height = (over_rows + body_rows) as f64 * block_size;

        Self {
            x: args.width as f64 / 2.0 - field_width / 2.0 + center_x_blocks * block_size,
            y: args.height as f64 / 2.0 - field_height / 2.0 + center_y_blocks * block_size,
            line_width: block_size / 20.0,
            block_size: block_size,
            cols: cols,
            over_rows: over_rows,
            body_rows: body_rows,
        }
    }

    fn new_main(args: &RenderArgs) -> Self {
        Self::new(args, 10, 2, 20, 0.0, 0.0)
    }

    fn new_stash(args: &RenderArgs) -> Self {
        Self::new(args, 5, 0, 4, -8.0, -8.5)
    }

    fn new_preview(args: &RenderArgs, idx: i32) -> Self {
        Self::new(args, 4, 0, 2, 7.5, -9.5 + idx as f64 * 3.0)
    }
}

pub struct Field {
    blocks: Vec<Block>,
    cur_tet: Option<Tetromino>,
    stash_tet: Option<Tetromino>,
}

fn main_block_start_pos() -> Trans2D {
    Trans2D::new(4, 1)
}

fn stash_block_start_pos() -> Trans2D {
    Trans2D::new(2, 2)
}

fn preview_block_start_pos() -> Trans2D {
    Trans2D::new(1, 1)
}

impl Field {
    pub fn new() -> Field {
        Field {
            blocks: vec![],
            cur_tet: None,
            stash_tet: None,
        }
    }

    pub fn new_cur_tetromino(&mut self) {
        if let Some(ref t) = self.cur_tet {
            self.blocks.extend_from_slice(t.blocks());
        }
        let mut new_block = Tetromino::new_t();
        new_block.jump_to(&main_block_start_pos());
        self.cur_tet = Some(new_block);
    }

    pub fn stash_tetromino(&mut self) {
        match self.cur_tet.take() {
            None => return,
            Some(mut cur) => {
                cur.reset_pos_rot(&stash_block_start_pos());
                match self.stash_tet.take() {
                    None => {
                        self.stash_tet = Some(cur);
                        self.new_cur_tetromino();
                    }
                    Some(mut stash) => {
                        stash.jump_to(&main_block_start_pos());
                        self.cur_tet = Some(stash);
                        self.stash_tet = Some(cur);
                    }
                }
            }
        }
    }

    pub fn move_tetromino(&mut self, trans: &Trans2D) {
        if let Some(ref mut t) = self.cur_tet {
            t.move_blocks(trans);
        }
    }

    pub fn rotate_tetromino(&mut self, dir: RotDir) {
        if let Some(ref mut t) = self.cur_tet {
            t.rotate_blocks(dir);
        }
    }
}

impl Field {
    pub fn draw(&self, render_args: &RenderArgs, c: &Context, gl: &mut GlGraphics) {
        let main_params = FieldDrawParams::new_main(render_args);

        self.draw_field_lines(c, gl, &main_params);

        for block in &self.blocks {
            block.draw(c, &main_params, gl);
        }

        if let &Some(ref t) = &self.cur_tet {
            t.draw(c, &main_params, gl);
        }

        let stash_params = FieldDrawParams::new_stash(render_args);
        self.draw_field_lines(c, gl, &stash_params);
        if let &Some(ref t) = &self.stash_tet {
            t.draw(c, &stash_params, gl);
        }

        for preview_idx in 0..5 {
            let preview_params = FieldDrawParams::new_preview(render_args, preview_idx);
            self.draw_field_lines(c, gl, &preview_params);
        }
    }

    fn draw_field_lines(&self, c: &Context, gl: &mut GlGraphics, params: &FieldDrawParams) {
        use graphics::{Line, Transformed};
        let line = Line::new_round(BLACK, params.line_width);

        let width = params.block_size * params.cols as f64;
        for y_idx in params.over_rows..params.over_rows + params.body_rows + 1 {
            line.draw([0.0, 0.0, width, 0.0],
                      &c.draw_state,
                      c.transform
                          .trans(params.x, params.y)
                          .trans(0.0, y_idx as f64 * params.block_size),
                      gl);
        }

        let height = params.block_size * params.body_rows as f64;
        let start_y = params.y + params.block_size * params.over_rows as f64;
        for x_idx in 0..params.cols + 1 {
            line.draw([0.0, 0.0, 0.0, height],
                      &c.draw_state,
                      c.transform
                          .trans(params.x, start_y)
                          .trans(x_idx as f64 * params.block_size, 0.0),
                      gl);

        }
    }
}
