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
use crate::game::{client::Client, local::DotDodger, server::Server};
use crate::physics::{Acceleration, PhysicsSystem, Position, Velocity};
use crate::player::PlayerControlSystem;
use crate::rendering::Visual;

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
    let options = App::new("Dot-dodger")
        .author("VoxWave")
        .about("A Dot dodging game for mature dot-dodgers such as myself.")
        .arg(
            Arg::with_name("server")
                .short("s")
                .long("server")
                .value_name("PORT")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("client")
                .short("c")
                .long("client")
                .value_name("ADDRESS:PORT")
                .takes_value(true),
        )
        .get_matches();

    match (options.is_present("server"), options.is_present("client")) {
        (true, false) => {
            let port = options.value_of("server").unwrap();
            run_server(port);
        }
        (false, true) => {
            let address = options.value_of("client").unwrap();
            run_client(address);
        }
        (false, false) => {
            run_local();
        }
        (true, true) => {
            println!("Both the server and client flags were used. Use one or the other. Alternatively use neither for local game.");
            return;
        }
    }
}

fn run_server(port: &str) {
    let (mut ctx, mut event_loop) = ContextBuilder::new("dot-dodger server", "VoxWave")
        .window_setup(
            WindowSetup::default()
                .title("dot-dodger server")
                .vsync(false),
        )
        .window_mode(WindowMode::default().dimensions(1280., 720.))
        .build()
        .expect("Failed to create a ggez context!");

    let mut dot_dodger = Server::new(&mut ctx, port);

    match event::run(&mut ctx, &mut event_loop, &mut dot_dodger) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("An error occured: {}", e),
    }
}

fn run_client(address: &str) {
    let (mut ctx, mut event_loop) = ContextBuilder::new("dot-dodger client", "VoxWave")
        .window_setup(
            WindowSetup::default()
                .title("dot-dodger client")
                .vsync(false),
        )
        .window_mode(WindowMode::default().dimensions(1280., 720.))
        .build()
        .expect("Failed to create a ggez context!");

    let mut dot_dodger = Client::new(&mut ctx, address);

    match event::run(&mut ctx, &mut event_loop, &mut dot_dodger) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("An error occured: {}", e),
    }
}

fn run_local() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("dot-dodger", "VoxWave")
        .window_setup(WindowSetup::default().title("dot-dodger").vsync(false))
        .window_mode(WindowMode::default().dimensions(1280., 720.))
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
