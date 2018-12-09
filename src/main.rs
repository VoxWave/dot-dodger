extern crate kolli_desu;
extern crate nalgebra as na;
extern crate piston_window;
extern crate specs;
#[macro_use]
extern crate specs_derive;

use piston_window::*;

mod bullet;
mod collision;
mod physics;
mod player;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", (640, 480))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
    while let Some(e) = window.next() {
        window.draw_2d(&e, |_c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
        });
    }
}

fn handle_death() {
    
}