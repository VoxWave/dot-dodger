use ggez::Context;

use crate::input::RawInput;

mod ingame_state;

pub trait GameState {
    fn update(self) -> Box<dyn GameState>;
    fn draw(&mut self, ctx: &mut Context);
    fn handle_input(&mut self, input: RawInput);
}

