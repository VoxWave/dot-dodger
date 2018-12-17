use piston_window::context::Context;
use piston_window::G2d;
use specs::{Component, DenseVecStorage, ReadExpect, ReadStorage};

use crate::physics::Position;
use crate::player::PlayerHandle;
#[derive(Component)]
pub enum Visual {
    Circle(f32),
    Sprite,
}

pub fn render(
    c: Context,
    g: &mut G2d,
    (player, positions, visuals): (
        ReadExpect<PlayerHandle>,
        ReadStorage<Position>,
        ReadStorage<Visual>,
    ),
) {
    
}
