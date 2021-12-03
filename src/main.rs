#[macro_use]
extern crate specs_derive;

use ggez::conf::FullscreenType;
use nalgebra as na;

use std::time::Duration;

use ggez::{
    conf::{WindowMode, WindowSetup},
    event::{self}, ContextBuilder,
};

use crate::game::DotDodger;

mod bullet;
mod collision;
mod enemy;
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
        .window_mode(WindowMode::default().dimensions(500., 500.).fullscreen_type(FullscreenType::Windowed))
        .build()
        .expect("Failed to create a ggez context!");

    let mut dot_dodger = DotDodger::new(&mut ctx);

    event::run(ctx, event_loop, dot_dodger)
}
