#[macro_use]
extern crate specs_derive;

use amethyst::{
    assets::Processor,
    prelude::*,
    renderer::{
        types::DefaultBackend, Factory, Format, GraphBuilder, GraphCreator,
        Kind, RenderGroupDesc, RenderingSystem, SpriteSheet, SubpassBuilder,
    },
    utils::application_root_dir,
    window::{ScreenDimensions, Window, WindowBundle},
};

use nalgebra as na;

use rendering::ExampleGraph;
use bullet::BulletPatternSystem;
use game::DotDodger;
use physics::PhysicsSystem;
use amethyst::{GameDataBuilder, Application};

mod bullet;
mod game;
mod physics;
mod player;
mod rendering;

pub struct Tick(u64);

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let assets = app_root.join("assets");
    let display_config_path = assets.join("configs").join("display_config.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(WindowBundle::from_config_path(display_config_path))?
        .with(
            Processor::<SpriteSheet>::new(),
            "sprite_sheet_processor",
            &[],
        )
        .with_thread_local(RenderingSystem::<DefaultBackend, _>::new(
            ExampleGraph::default(),
        ));

    let mut game = Application::new(assets, DotDodger, game_data)?;

    Ok(())
}