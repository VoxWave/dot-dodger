use ggez::Context;

use crate::input::RawInput;

pub mod ingame;

pub struct SharedData {
    back_to_main_menu: bool,
}

pub trait GameState {
    fn update(&mut self, shared_data: Option<&mut SharedData>) -> Transition;
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()>;
    fn handle_input(&mut self, input: RawInput);
}

enum Transition {
    Stay,
    Switch(Box<dyn GameState>),
    Push(Box<dyn GameState>),
    Pop,
    Quit
}



