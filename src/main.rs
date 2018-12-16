
use std::sync::mpsc::channel;
use crate::collision::Hitbox;
use specs::world::Builder;
use nalgebra as na;


#[macro_use]
extern crate specs_derive;

use std::time::{Duration, Instant};

use crate::bullet::BulletComponent;
use crate::collision::CollisionSystem;
use crate::player::{PlayerControlSystem, PlayerHandle};
use crate::physics::{PhysicsSystem, Position, Velocity, Acceleration};
use piston_window::*;
use specs::{DispatcherBuilder, World};

use crate::na::{Point2, Vector2, zero};

mod bullet;
mod collision;
mod physics;
mod player;

const FRAME: Duration = Duration::from_millis(1000/60);

fn main() {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Hitbox>();
    world.register::<Acceleration>();
    world.register::<BulletComponent>();
    
    let player = world.create_entity()
        .with(Position(Point2::new(0., 0.)))
        .with(Velocity(zero()))
        .with(Acceleration(zero()))
        .with(Hitbox::Point(Point2::new(0., 0.)))
        .build();
    world.add_resource(PlayerHandle(player));

    let (send, recv) = channel();

    let mut dispatcher = DispatcherBuilder::new()
        .with(PlayerControlSystem::new(recv), "player_control_system", &[])
        .with(PhysicsSystem, "physics_system", &["player_control_system"])
        .with(CollisionSystem, "collision_system", &["physics_system"])
        .build();

    let mut window: PistonWindow = WindowSettings::new("dot-dodger", (640, 480))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
    let mut instant = Instant::now();
    while let Some(e) = window.next() {    
        match e {
            Event::Input(input) => send.send(input.clone()).unwrap(),
            _ => { 
                window.draw_2d(&e, |_c, g| {
                    clear([0.0, 0.0, 0.0, 1.0], g);
                });
            },
        }
        if instant.elapsed() >= FRAME {
            dispatcher.dispatch(&mut world.res);
            world.maintain();
            instant = Instant::now();
        }
    }
}

fn handle_death() {
    println!("dedness happen");
}