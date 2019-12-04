use std::{sync::mpsc::{channel, Sender}, time::Instant};

use ggez::{
    event::{self, EventHandler},
    graphics, Context, GameResult,
    input::keyboard::{KeyCode, KeyMods},
};
use specs::{Builder, Dispatcher, DispatcherBuilder, World, prelude::WorldExt};

use crate::{
    bullet::{BulletComponent, BulletPatternSystem},
    collision::{CollisionSystem, Hitbox},
    physics::{Acceleration, PhysicsSystem, Position, Velocity},
    player::{PlayerControlSystem, PlayerInputState, PCSMessage},
    rendering::{Renderer, Visual},
    input::{AxisState, Axis, InputConfig, RawInput},
    Tick, FRAME, na::{Vector2, Point2},
};

pub struct DotDodger<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
    renderer: Renderer,
    last_tick: Instant,
    input_channel: Sender<PCSMessage>,
    raw_inputs: Vec<RawInput>,
    input_config: InputConfig,
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

        let player1 = world
            .create_entity()
            .with(PlayerInputState(AxisState::Neutral, AxisState::Neutral))
            .with(Position(Point2::new(0., 0.)))
            .with(Velocity(Vector2::new(0., 0.)))
            .with(Acceleration(Vector2::new(0., 0.)))
            .with(Visual::Sprite("player".to_string()))
            .build();

        send.send(PCSMessage::NewPlayer(player1));

        let renderer = Renderer::new(ctx);

        DotDodger {
            world,
            dispatcher,
            renderer,
            last_tick: Instant::now(),
            input_channel: send,
            raw_inputs: Vec::new(),
            input_config: InputConfig::new(),
        }
    }

    fn handle_input(&mut self) {
        for raw_input in self.raw_inputs.drain(..) {
            // if let Some(input) = self.input_config.get_input(raw_input) {
            //     self.input_channel.send(input);
            // }
        }
    }
}

impl<'a, 'b> EventHandler for DotDodger<'a, 'b> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.last_tick.elapsed() >= FRAME {
            self.handle_input();
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

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, repeat: bool) {
        if !repeat {
            
        }
    }
}
