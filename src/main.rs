
use nalgebra as na;


#[macro_use]
extern crate specs_derive;

use std::time::{Duration, Instant};

use crate::bullet::BulletComponent;
use crate::collision::CollisionSystem;
use crate::physics::{PhysicsSystem, Position, Velocity, Acceleration};
use piston_window::*;
use specs::{DispatcherBuilder, World};

mod bullet;
mod collision;
mod physics;
mod player;

const FRAME: Duration = Duration::from_millis(1000/60);

fn main() {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Acceleration>();
    world.register::<BulletComponent>();
    
    let mut dispatcher = DispatcherBuilder::new()
        .with(PhysicsSystem, "physics_system", &[])
        .with(CollisionSystem, "collision_system", &["physics_system"])
        .build();

    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", (640, 480))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
    let mut instant = Instant::now();
    while let Some(e) = window.next() {
        if instant.elapsed() >= FRAME {
            dispatcher.dispatch(&mut world.res);
            world.maintain();
            instant = Instant::now();
        }
        window.draw_2d(&e, |_c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
        });
    }
}

fn handle_death() {
    println!("dedness happen");
}