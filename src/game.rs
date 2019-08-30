use std::time::Instant;

use ggez::{
    Context, GameResult, graphics,
    event::{self, EventHandler},
};

use specs::{
        world::{
            Builder,
        },
        World, 
        Dispatcher, 
        DispatcherBuilder,
    };

use crate::na::{zero, Point2};

use crate::bullet::{BulletComponent, BulletPatternSystem};
use crate::collision::{CollisionSystem, Hitbox};
use crate::FRAME;
use crate::physics::{Acceleration, PhysicsSystem, Position, Velocity};
use crate::rendering::{render, Visual};

pub struct DotDodger<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
    player: Entity,
    last_tick: Instant,
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

        let player = world
        .create_entity()
        .with(Position(Point2::new(200., 200.)))
        .with(Velocity(zero()))
        .with(Acceleration(zero()))
        .with(Hitbox::Point(Point2::new(0., 0.)))
        .with(Visual::Circle([1., 0., 0., 1.],10.))
        // .with(Visual::Sprite(player_texture))
        .build();
        world.add_resource(PlayerHandle(player));
        world.add_resource(Tick(0));
        DotDodger {
            world,
            dispatcher,
            player,
            last_tick: Instant::now(),
        }
    }
}

impl<'a, 'b> EventHandler for DotDodger<'a, 'b> {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if instant.elapsed() >= FRAME {
            self.dispatcher.dispatch(&mut world.res);
            self.world.maintain();
            self.world.write_resource::<Tick>().0 += 1;
            self.last_tick = Instant::now();
        }
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
