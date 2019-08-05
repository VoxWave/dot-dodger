#[macro_use]
extern crate specs_derive;

use amethyst::{
    core::transform::TransformBundle,
    assets::Processor,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend, RenderingBundle, SpriteSheet,
    },
    utils::application_root_dir,
    window::WindowBundle,
};

use nalgebra as na;
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
    let assets = app_root.join("assets/");
    let display_config_path = assets.join("configs").join("display_config.ron");

    let game_data = GameDataBuilder::default()
        .with(PhysicsSystem, "physics_system", &[])
        .with(BulletPatternSystem, "bullet_pattern_system", &["physics_system"])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?;

    let mut game = Application::new(assets, DotDodger, game_data)?;
    game.run();

    Ok(())
}