use ggez::Context;

use crate::input::RawInput;

trait GameState {
    fn update(self) -> dyn GameState;
    fn draw(&mut self, ctx: &mut Context);
    fn handle_input(&mut self, input: RawInput);
}