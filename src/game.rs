use ggez::{
    Context, GameResult, graphics,
    event::{self, EventHandler},
};

use specs::{World, Dispatcher};

use crate::bullet::{BulletComponent, BulletPatternSystem};
use crate::collision::{CollisionSystem, Hitbox};
use crate::physics::{Acceleration, PhysicsSystem, Position, Velocity};
use crate::rendering::{render, Visual};

pub struct DotDodger<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> DotDodger<'a, 'b> {
    pub fn new(_ctx: &mut Context) -> Self {
        let mut world = World::new();
        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<Hitbox>();
        world.register::<Acceleration>();
        world.register::<BulletComponent>();
        world.register::<Visual>();

        let (send, recv) = channel();

        let mut dispatcher = DispatcherBuilder::new()
            .with(PlayerControlSystem::new(recv), "player_control_system", &[])
            .with(BulletPatternSystem, "bullet_pattern_system", &[])
            .with(PhysicsSystem, "physics_system", &["player_control_system"])
            .with(CollisionSystem, "collision_system", &["physics_system"])
            .build();
        DotDodger {
            world,
            dispatcher
        }
    }
}

impl EventHandler for DotDodger {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        self.world.exec(|s| {
            render(ctx, s);
        });
        graphics::present(ctx)
    }
}
