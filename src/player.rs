use specs::{Component, Entity, NullStorage};

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
struct PlayerComponent;

pub fn get_player_handle() -> Entity {
    
}