use amethyst::{
    core::transform::Transform,
    prelude::*,
};

use crate::bullet::BulletComponent;
use crate::Tick;

use crate::rendering;

pub struct DotDodger;

impl SimpleState for DotDodger {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        rendering::initialise_camera(world);

        let sprite_sheet_handle = load_sprite_sheet(world);

        world.add_resource(Tick(0));
        world.register::<BulletComponent>();
        world.register::<Transform>();
        //TODO:
        // eli tänne nyt se spritesheetin lataus sillä renderissä olevalla funktiolla
        // relevantti osuus pong tutoriaalista on 02 loppupäässä.
    }
}
