use amethyst::prelude::*;

use crate::bullet::BulletComponent;
use crate::Tick;

use crate::rendering;

pub struct DotDodger;

impl SimpleState for DotDodger {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        // rendering::initialise_camera(world);
        // world.add_resource(Tick(0));
        // world.register::<BulletComponent>();
        //TODO:
        // eli tänne nyt se spritesheetin lataus sillä renderissä olevalla funktiolla
        // relevantti osuus pong tutoriaalista on 02 loppupäässä.
    }
}
