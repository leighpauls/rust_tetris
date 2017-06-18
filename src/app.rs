
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::input::{RenderArgs, UpdateArgs};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::Button;
use piston::input::Button::Keyboard;
use piston::input::keyboard::Key;

use field::Field;
use tetronimo::RotDir;

use GREEN;
use Trans2D;

pub struct App {
    gl: GlGraphics,
    field: Field,
}

impl App {
    pub fn new(opengl: OpenGL) -> App {
        App {
            gl: GlGraphics::new(opengl),
            field: Field::new(),
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let field = &self.field;
        self.gl
            .draw(args.viewport(), |c, gl| {
                use graphics::clear;
                clear(GREEN, gl);
                field.draw(args, &c, gl);
            });

    }

    pub fn key_press(&mut self, button: &Button) {
        if let &Keyboard(ref key) = button {

            match key {
                &Key::Space => self.field.new_cur_tetromino(),
                &Key::Up => self.field.move_block(&(0, -1)),
                &Key::Down => self.field.move_block(&(0, 1)),
                &Key::Left => self.field.move_block(&(-1, 0)),
                &Key::Right => self.field.move_block(&(1, 0)),
                &Key::Z => self.field.rotate_block(RotDir::CCW),
                &Key::X => self.field.rotate_block(RotDir::CW),
                _ => (),
            }
        }
    }

    pub fn update(&mut self, _: &UpdateArgs) {
        // 2 radians / second
        // self.rotation += 2.0 * args.dt;
    }
}
