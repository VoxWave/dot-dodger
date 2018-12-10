use crate::physics::Velocity;
use specs::{Entity, WriteStorage, System, WriteExpect};

// #[derive(Component, Debug, Default)]
// #[storage(NullStorage)]
// struct PlayerComponent;

pub struct PlayerHandle(pub Entity);

pub struct PlayerControlSystem {
    button_states: HashMap<Direction, bool>,
};

impl PlayerControlSystem {
    pub fn new() -> Self {
        let button_states = HashMap::new();
        button_states.insert(Direction::Up, false);
        button_states.insert(Direction::Down, false);
        button_states.insert(Direction::Left, false);
        button_states.insert(Direction::Right, false);
        PlayerControlSystem {
            button_states,
        } 
    }
}

enum Direction {
    Up, Down, Left, Right,
}

impl<'a> System<'a> for PlayerControlSystem {

    type SystemData = (
        WriteExpect<'a, PlayerHandle>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (player, mut velocities): Self::SystemData) {
        let _player_vel = velocities.get_mut(player.0);
        
    }
}
