use crate::physics::Velocity;
use specs::{Entity, WriteStorage, System, WriteExpect};

// #[derive(Component, Debug, Default)]
// #[storage(NullStorage)]
// struct PlayerComponent;

pub struct PlayerHandle(pub Entity);

pub struct PlayerControlSystem;

impl<'a> System<'a> for PlayerControlSystem {
    type SystemData = (
        WriteExpect<'a, PlayerHandle>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (player, mut velocities): Self::SystemData) {
        let _player_vel = velocities.get_mut(player.0);
        
    }
}
