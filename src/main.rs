#[macro_use]
extern crate specs_derive;

use nalgebra as na;
use specs::Join;
use std::borrow::BorrowMut;

use std::path::Path;
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};

use specs::{DispatcherBuilder, World};

use clap::{App, Arg};

use ggez::{
    conf::{WindowMode, WindowSetup},
    event::{self, EventHandler},
    graphics, Context, ContextBuilder, GameResult,
};

use crate::bullet::{BulletComponent, BulletPatternSystem};
use crate::collision::CollisionSystem;
use crate::collision::Hitbox;
use crate::game::DotDodger;
use crate::physics::{Acceleration, PhysicsSystem, Position, Velocity};
use crate::player::PlayerControlSystem;
use crate::rendering::Visual;

mod bullet;
mod collision;
mod game;
mod input;
mod life;
mod physics;
mod player;
mod rendering;
mod sound;
mod utils;

const FRAME: Duration = Duration::from_millis(1000 / 60);

pub struct Tick(u64);

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("dot-dodger", "VoxWave")
        .window_setup(WindowSetup::default().title("dot-dodger").vsync(false))
        .window_mode(WindowMode::default().dimensions(480., 480.))
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
