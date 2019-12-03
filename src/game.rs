use std::{sync::mpsc::channel, time::Instant};

use ggez::{
    event::{self, EventHandler},
    graphics, Context, GameResult,
};
use specs::{Builder, Dispatcher, DispatcherBuilder, World, prelude::WorldExt};

use crate::{
    bullet::{BulletComponent, BulletPatternSystem},
    collision::{CollisionSystem, Hitbox},
    physics::{Acceleration, PhysicsSystem, Position, Velocity},
    player::{AxisState, PlayerControlSystem, PlayerInputState},
    rendering::Renderer,
    rendering::Visual,
    Tick, FRAME, na::{Vector2, Point2},
};

pub struct DotDodger<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
    renderer: Renderer,
    last_tick: Instant,
}

impl<'a, 'b> DotDodger<'a, 'b> {
    pub fn new(ctx: &mut Context) -> Self {
        let mut world = World::new();
        world.register::<Visual>();

        let (send, recv) = channel();

        let mut dispatcher = DispatcherBuilder::new()
            .with(PlayerControlSystem::new(recv), "player_control_system", &[])
            .with(BulletPatternSystem, "bullet_pattern_system", &[])
            .with(
                PhysicsSystem,
                "physics_system",
                &[/*"player_control_system"*/],
            )
            .with(CollisionSystem, "collision_system", &["physics_system"])
            .build();
        dispatcher.setup(&mut world);
        world.insert(Tick(0));

        world
            .create_entity()
            .with(PlayerInputState(AxisState::Neutral, AxisState::Neutral))
            .with(Position(Point2::new(0., 0.)))
            .with(Velocity(Vector2::new(0., 0.)))
            .with(Acceleration(Vector2::new(0., 0.)))
            .with(Visual::Sprite("player".to_string()))
            .build();

        let renderer = Renderer::new(ctx);

        DotDodger {
            world,
            dispatcher,
            renderer,
            last_tick: Instant::now(),
        }
    }
}

impl<'a, 'b> EventHandler for DotDodger<'a, 'b> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.last_tick.elapsed() >= FRAME {
            self.dispatcher.dispatch(&self.world);
            self.world.maintain();
            self.world.write_resource::<Tick>().0 += 1;
            self.last_tick = Instant::now();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        let renderer = &self.renderer;
        self.world.exec(|s| {
            renderer.render(ctx, s);
        });
        graphics::present(ctx)
    }
}
