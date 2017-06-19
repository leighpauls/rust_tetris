
mod app;
mod block;
mod field;
mod tetronimo;
mod trans2d;
mod shape;
mod shape_bag;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::{Events, EventSettings};
use piston::input::{RenderEvent, UpdateEvent, PressEvent};
use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use graphics::types::Color;

use app::App;

const GREEN: Color = [0.0, 1.0, 0.0, 1.0];
const RED: Color = [1.0, 0.0, 0.0, 1.0];
const BLACK: Color = [0.0, 0.0, 0.0, 1.0];


pub fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("Tetris", [640, 480])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App::new(opengl);

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(p) = e.press_args() {
            app.key_press(&p);
        }
    }
}
