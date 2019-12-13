use ggez::Context;

use crate::input::RawInput;

mod ingame_state;

pub trait GameState<T> {
    fn update(self, shared_data: T) -> (Trans, T);
    fn draw(&mut self, ctx: &mut Context);
    fn handle_input(&mut self, input: RawInput);
}

enum Trans {

}

