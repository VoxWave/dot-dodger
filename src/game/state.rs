use ggez::Context;

use crate::input::RawInput;

trait GameState {
    fn update(&mut self) -> Option<dyn GameState>;
    fn draw(&mut self, ctx: &mut Context);
    fn handle_input(&mut self, input: RawInput);
}