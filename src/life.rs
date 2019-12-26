use specs::{Component, DenseVecStorage, ReadStorage, Join};

#[derive(Component, Debug, Default)]
pub struct Lives(pub u8);

pub fn everyone_dead(lives: ReadStorage<Lives>) -> bool {
    !lives.join().any(|life| life.0 > 0 )
}