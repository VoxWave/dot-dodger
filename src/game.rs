use std::{
    sync::mpsc::{channel, Sender},
    time::Instant,
};

use kolli_desu::shapes::Circle;

use ggez::{
    event::{self, EventHandler},
    graphics,
    input::keyboard::{KeyCode, KeyMods},
    Context, GameResult,
};
use specs::{prelude::WorldExt, Builder, Dispatcher, DispatcherBuilder, World};

use crate::{
    bullet::{BulletComponent, BulletPatternSystem},
    collision::{CollisionSystem, Hitbox},
    game::state::{GameState, ingame::InGame},
    input::{Axis, AxisState, InputHandler, RawInput},
    na::{Point2, Vector2},
    physics::{Acceleration, PhysicsSystem, Position, Velocity},
    player::{PCSMessage, PlayerControlSystem, PlayerInputState},
    rendering::{Renderer, Visual},
    Tick, FRAME,
};

mod state;

pub struct DotDodger {
    current_state: Box<dyn GameState>,
}

impl DotDodger {
    pub fn new(ctx: &mut Context) -> Self {
        DotDodger {
            current_state: Box::new(InGame::new(ctx)),
        }
    }

    fn handle_input(&mut self) {
        for (player_id, input) in self.input_handler.get_inputs() {
            self.input_channel.send(PCSMessage::Input(player_id, input)).unwrap();
        }
    }
}

impl EventHandler for DotDodger {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.current_state.update(None);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.current_state.draw(ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        repeat: bool,
    ) {
        if !repeat {
            self.input_handler
                .handle_input(RawInput::KeyBoard(keycode, true));
        }
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        self.input_handler
            .handle_input(RawInput::KeyBoard(keycode, false));
    }
}
