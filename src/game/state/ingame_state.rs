use std::{
    sync::mpsc::{channel, Sender},
    time::Instant,
};

use specs::{
    prelude::WorldExt,
    Builder,
    Dispatcher, 
    DispatcherBuilder,
    World,
};

use ggez::Context;

use crate::{
    game::state::GameState,
    input::InputHandler,
    player::PCSMessage,
    rendering::Renderer,
};

struct InGame<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
    renderer: Renderer,
    last_tick: Instant,
    input_channel: Sender<PCSMessage>,
    input_handler: InputHandler,
}

impl<'a, 'b> InGame<'a, 'b> {
    pub fn new(ctx: &mut Context) -> Self {

    }
}

impl <'a, 'b> GameState for InGame<'a, 'b> {
    fn update(self) -> Box<dyn GameState> {
        Box::new(self)
    }

    fn draw(&mut self, ctx: &mut Context) {

    }

    fn handle_input(&mut self, input: RawInput) {

    }

}