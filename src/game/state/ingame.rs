use std::{
    collections::HashMap,
    sync::mpsc::{channel, Sender},
    time::Instant,
};

use specs::{
    prelude::WorldExt,
    Builder,
    Dispatcher,
    DispatcherBuilder,
    World,
};

use ggez::{Context, GameResult, graphics};

use crate::{
    game::state::{GameState, Transition, SharedData},
    bullet::BulletPatternSystem,
    collision::{Hitbox, CollisionSystem},
    input::{AxisState, InputHandler, RawInput},
    na::{Point2, Vector2},
    physics::{Position, Velocity, Acceleration, PhysicsSystem},
    player::{PCSMessage, PlayerInputState, PlayerControlSystem},
    rendering::{Renderer, Visual},
    Tick, FRAME,
};

pub struct InGame<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
    renderer: Renderer,
    last_tick: Instant,
    input_channel: Sender<PCSMessage>,
    input_handler: InputHandler,
}

impl<'a, 'b> InGame<'a, 'b> {
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
            .with(Visual::Sprite("player".to_string(), 1.))
            .with(Hitbox::Point(Point2::new(0., 0.)))
            .build();

        send.send(PCSMessage::NewPlayer(player1)).unwrap();

        let renderer = Renderer::new(ctx);

        InGame {
            world,
            dispatcher,
            renderer,
            last_tick: Instant::now(),
            input_channel: send,
            input_handler: InputHandler::new(),
        }
    }
}

impl <'a, 'b> GameState for InGame<'a, 'b> {
    fn update(&mut self, ctx: &mut Context, shared_data: &mut SharedData) -> Transition {
        if self.last_tick.elapsed() >= FRAME {
            for (player_id, input) in self.input_handler.get_inputs() {
                self.input_channel.send(PCSMessage::Input(player_id, input)).unwrap();
            }
            self.dispatcher.dispatch(&self.world);
            self.world.maintain();
            self.world.write_resource::<Tick>().0 += 1;
            self.last_tick = Instant::now();
        }
        Transition::Stay
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        let renderer = &self.renderer;
        self.world.exec(|s| {
            renderer.render(ctx, s);
        });
        graphics::present(ctx)
    }

    fn handle_input(&mut self, input: RawInput) {
        self.input_handler.handle_input(input);
    }

}