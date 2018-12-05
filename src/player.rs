use specs::{Component, NullStorage};

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
struct PlayerComponent;

pub fn get_player() -> u32 {

}