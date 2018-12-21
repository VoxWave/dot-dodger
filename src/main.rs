#[macro_use]
extern crate specs_derive;

use nalgebra as na;
use specs::world::Builder;
use specs::Join;

use std::sync::mpsc::channel;
use std::time::{Duration, Instant};

use crate::bullet::{BulletComponent, BulletPatternSystem};
use crate::collision::CollisionSystem;
use crate::collision::Hitbox;
use crate::physics::{Acceleration, PhysicsSystem, Position, Velocity};
use crate::player::{PlayerControlSystem, PlayerHandle};
use crate::rendering::{render, Visual};
use piston_window::*;
use specs::{DispatcherBuilder, World};

use crate::na::{zero, Point2};

mod bullet;
mod collision;
mod physics;
mod player;
mod rendering;

const FRAME: Duration = Duration::from_millis(1000 / 60);

fn main() {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Hitbox>();
    world.register::<Acceleration>();
    world.register::<BulletComponent>();
    world.register::<Visual>();

    let player = world
        .create_entity()
        .with(Position(Point2::new(200., 200.)))
        .with(Velocity(zero()))
        .with(Acceleration(zero()))
        .with(Hitbox::Point(Point2::new(0., 0.)))
        .with(Visual::Circle([0., 0.7, 0., 1.], 10.))
        .build();
    world.add_resource(PlayerHandle(player));

    let (send, recv) = channel();

    let mut dispatcher = DispatcherBuilder::new()
        .with(PlayerControlSystem::new(recv), "player_control_system", &[])
        .with(BulletPatternSystem, "bullet_pattern_system", &[])
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
                window.draw_2d(&e, |c, g| {
                    clear([0.0, 0.0, 0.0, 1.0], g);
                    &mut world.exec(|s| {
                        render(c, g, s);
                    });
                });
            }
        }
        if instant.elapsed() >= FRAME {
            let count = world.entities().join().count();
            println!("{}, {:?}", count, instant.elapsed());
            dispatcher.dispatch(&mut world.res);
            world.maintain();
            instant = Instant::now();
        }
    }
}

fn handle_death() {
    println!("dedness happen");
}
