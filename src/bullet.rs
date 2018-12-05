use specs::{Component, NullStorage};

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct BulletComponent;