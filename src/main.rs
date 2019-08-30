#[macro_use]
extern crate specs_derive;

use std::borrow::BorrowMut;
use nalgebra as na;
use specs::world::Builder;
use specs::Join;

use std::sync::mpsc::channel;
use std::time::{Duration, Instant};
use std::path::Path;

use specs::{DispatcherBuilder, World};

use ggez::{
    graphics,
    Context,
    ContextBuilder,
    GameResult,
    event::{
        self,
        EventHandler
    }
};

use crate::bullet::{BulletComponent, BulletPatternSystem};
use crate::collision::CollisionSystem;
use crate::collision::Hitbox;
use crate::game::DotDodger;
use crate::physics::{Acceleration, PhysicsSystem, Position, Velocity};
use crate::player::{PlayerControlSystem, PlayerHandle};
use crate::rendering::{render, Visual};

use crate::na::{zero, Point2};

mod bullet;
mod collision;
mod game;
mod physics;
mod player;
mod rendering;

const FRAME: Duration = Duration::from_millis(1000 / 60);

pub struct Tick(u64);

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("dot-dodger", "VoxWave")
        .build()
        .expect("Failed to create a ggez context!");

    let mut dot_dodger = DotDodger::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut dot_dodger) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("An error occured: {}", e),
    }

    let player = world
        .create_entity()
        .with(Position(Point2::new(200., 200.)))
        .with(Velocity(zero()))
        .with(Acceleration(zero()))
        .with(Hitbox::Point(Point2::new(0., 0.)))
        .with(Visual::Circle([1., 0., 0., 1.],10.))
        // .with(Visual::Sprite(player_texture))
        .build();
    world.add_resource(PlayerHandle(player));
    world.add_resource(Tick(0));

    let mut instant = Instant::now();
    while let Some(e) = window.next() {
        println!("entities: {}", world.entities().join().count());
        match e {
            Event::Input(input) => send.send(input.clone()).unwrap(),
            _ => {
                window.draw_2d(&e, |c, g| {
                    clear([0.0, 0.0, 0.0, 1.0], g);
                });
            }
        }
        if instant.elapsed() >= FRAME {
            dispatcher.dispatch(&mut world.res);
            world.maintain();
            world.write_resource::<Tick>().0 += 1;
            instant = Instant::now();
        }
    }
}

fn handle_death() {
    println!("dedness happen");
}
