use amethyst::assets::{AssetStorage, Loader};
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;

use crate::Tick;
use crate::physics::{Acceleration, Position, Velocity};

use crate::rendering;

pub struct DotDodger;

impl SimpleState for DotDodger {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        rendering::initialise_camera(world);
        world.add_resource(Tick(0));
    }
}