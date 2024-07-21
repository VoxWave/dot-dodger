use ggez::{
    event::EventHandler, input::keyboard::{KeyCode, KeyMods}, Context, GameError, GameResult
};

use crate::{
    game::state::{GameState, main_menu::MainMenu, SharedData},
    input::RawInput,

};

mod state;

pub struct DotDodger {
    current_states: Vec<Box<dyn GameState>>,
    shared_data: SharedData,
}

impl DotDodger {
    pub fn new() -> Self {
        DotDodger {
            current_states: vec![Box::new(MainMenu::new())],
            shared_data: SharedData::new(),
        }
    }

    fn top_state(&self) -> usize {
        self.current_states.len() - 1
    }
}

impl EventHandler<GameError> for DotDodger {
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
        if keycode == KeyCode::Escape {
            ggez::event::quit(ctx);
        }
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
