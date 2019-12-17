use ggez::{Context, GameResult, graphics, event::KeyCode};

use crate::{
    game::state::{GameState, SharedData, Transition, ingame::InGame},
    input::RawInput,
};

pub struct MainMenu {
    start_game: bool,
}

impl MainMenu {
    pub fn new() -> Self {
        MainMenu {
            start_game: false,
        }
    }
}

impl GameState for MainMenu {
    fn update(&mut self, ctx: &mut Context, shared_data: &mut SharedData) -> Transition {
        use Transition::*;
        if self.start_game {
            Switch(Box::new(InGame::new(ctx)))
        } else {
            Stay
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        graphics::present(ctx)
    }

    fn handle_input(&mut self, input: RawInput) {
        match input {
            RawInput::KeyBoard(KeyCode::A, true) => self.start_game = true,
            _ => {},
        }
    }
}