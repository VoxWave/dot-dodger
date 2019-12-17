use ggez::{Context, GameResult};

use crate::input::RawInput;

pub mod ingame;

pub struct SharedData {
    back_to_main_menu: bool,
}

impl SharedData {
    pub fn new() -> Self {
        SharedData {
            back_to_main_menu: false,
        }
    }
}

pub trait GameState {
    fn update(&mut self, shared_data: &mut SharedData) -> Transition;
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()>;
    fn handle_input(&mut self, input: RawInput);
}

pub enum Transition {
    Stay,
    Switch(Box<dyn GameState>),
    Push(Box<dyn GameState>),
    Pop,
    Quit
}



