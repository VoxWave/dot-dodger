use std::{
    sync::{
        mpsc::{channel, Sender},
        Mutex,
    },
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
    game::state::{GameState, Transition, SharedData, main_menu::MainMenu},
    bullet::BulletPatternSystem,
    collision::{Hitbox, CollisionSystem, Invul},
    life::{self, Lives},
    input::{AxisState, InputHandler, RawInput},
    na::{Point2, Vector2},
    physics::{Position, Velocity, Acceleration, PhysicsSystem},
    player::{PCSMessage, PlayerInputState, PlayerControlSystem},
    rendering::{Renderer, Visual},
    sound::{SoundChannel, SoundMessage, SoundPlayer},
    Tick, FRAME,
};

pub struct InGame<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
    renderer: Renderer,
    sound_player: SoundPlayer,
    last_tick: Instant,
    input_channel: Sender<PCSMessage>,
    input_handler: InputHandler,
}

impl<'a, 'b> InGame<'a, 'b> {
    pub fn new(ctx: &mut Context) -> Self {
        let mut world = World::new();
        world.register::<Visual>();
        world.register::<Lives>();

        let (input_sender, input_recv) = channel();
        let (snd_msg_sender, snd_msg_receiver) = channel();

        let mut dispatcher = DispatcherBuilder::new()
            .with(PlayerControlSystem::new(input_recv), "player_control_system", &[])
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
            .with(Lives(3))
            .with(Invul(0))
            .with(Position(Point2::new(0., 0.)))
            .with(Velocity(Vector2::new(0., 0.)))
            .with(Acceleration(Vector2::new(0., 0.)))
            .with(Visual::Sprite("player".to_string(), 1.))
            .with(Hitbox::Point(Point2::new(0., 0.)))
            .build();

        input_sender.send(PCSMessage::NewPlayer(player1)).unwrap();

        world.insert(SoundChannel(Mutex::new(snd_msg_sender)));

        let renderer = Renderer::new(ctx);

        InGame {
            world,
            dispatcher,
            renderer,
            sound_player: SoundPlayer::new(ctx, snd_msg_receiver),
            last_tick: Instant::now(),
            input_channel: input_sender,
            input_handler: InputHandler::new(),
        }
    }
}

impl <'a, 'b> GameState for InGame<'a, 'b> {
    fn update(&mut self, ctx: &mut Context, shared_data: &mut SharedData) -> Transition {
        let mut elapsed = self.last_tick.elapsed();
        while elapsed >= FRAME {
            for (player_id, input) in self.input_handler.get_inputs() {
                self.input_channel.send(PCSMessage::Input(player_id, input)).unwrap();
            }
            self.dispatcher.dispatch(&self.world);
            self.world.maintain();
            if self.world.exec(|s| life::everyone_dead(s) ) {
                return Transition::Switch(Box::new(MainMenu::new()))
            }
            self.sound_player.update(ctx);
            self.world.write_resource::<Tick>().0 += 1;
            elapsed -= FRAME;
        }
        Transition::Stay
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::WHITE);
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