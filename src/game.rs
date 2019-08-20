use ggez::{
    Context, GameResult, graphics,
    event::{self, EventHandler},
};

use specs::{World, Dispatcher};

pub struct DotDodger {
    world: World,
    dispatcher: Dispatcher,
}

impl DotDodger {
    pub fn new(_ctx: &mut Context) -> Self {
        DotDodger {}
    }
}

impl EventHandler for DotDodger {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        graphics::present(ctx)
    }
}
