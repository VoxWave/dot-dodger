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
mod utils;

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
}

fn handle_death() {
    println!("dedness happen");
}
