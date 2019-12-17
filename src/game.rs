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
    game::state::{GameState, ingame::InGame, main_menu::MainMenu, SharedData},
    input::{Axis, AxisState, InputHandler, RawInput},
    na::{Point2, Vector2},
    physics::{Acceleration, PhysicsSystem, Position, Velocity},
    player::{PCSMessage, PlayerControlSystem, PlayerInputState},
    rendering::{Renderer, Visual},
    Tick, FRAME,
};

mod state;

pub struct DotDodger {
    current_states: Vec<Box<dyn GameState>>,
    shared_data: SharedData,
}

impl DotDodger {
    pub fn new(ctx: &mut Context) -> Self {
        DotDodger {
            current_states: vec![Box::new(MainMenu::new())],
            shared_data: SharedData::new(),
        }
    }

    fn top_state(&self) -> usize {
        self.current_states.len() - 1
    }
}

impl EventHandler for DotDodger {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        use crate::game::state::Transition::*;
        let top = self.top_state();
        let transition = self.current_states[top].update(ctx, &mut self.shared_data);
        match transition {
            Stay => {},
            Switch(new_state) => self.current_states[top] = new_state,
            Push(new_state) => self.current_states.push(new_state),
            Pop => {
                self.current_states.pop().unwrap();
                if self.current_states.is_empty() {
                    ggez::event::quit(ctx);    
                }
            },
            Quit => {
                ggez::event::quit(ctx);
            },
        };
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let top = self.top_state();
        self.current_states[top].draw(ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        repeat: bool,
    ) {
        let top = self.top_state();
        if !repeat {
            self.current_states[top].handle_input(RawInput::KeyBoard(keycode, true));
        }
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        let top = self.top_state();
        self.current_states[top].handle_input(RawInput::KeyBoard(keycode, false));
    }
}
