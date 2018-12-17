use specs::{Component, DenseVecStorage, ReadExpect, ReadStorage};

use crate::physics::Position;
use crate::player::PlayerHandle;
#[derive(Component)]
pub enum Visual {
    Sprite,
}

pub fn render(
    (player, positions, visuals): (
        ReadExpect<PlayerHandle>,
        ReadStorage<Position>,
        ReadStorage<Visual>,
    ),
) {

}
