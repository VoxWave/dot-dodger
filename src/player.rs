use std::collections::HashMap;

use crate::physics::Velocity;
use specs::{Entity, WriteStorage, System, WriteExpect};

// #[derive(Component, Debug, Default)]
// #[storage(NullStorage)]
// struct PlayerComponent;

#[derive(Eq, Hash, PartialEq,)]
enum Direction {
    Up, Down, Left, Right,
}

pub struct PlayerHandle(pub Entity);

pub struct PlayerControlSystem {
    button_states: HashMap<Direction, bool>,
    input_channel: Receiver<(Direction, bool)>,
}

impl PlayerControlSystem {
    pub fn new(input_channel: Receiver<Direction>) -> Self {
        let mut button_states = HashMap::new();
        button_states.insert(Direction::Up, false);
        button_states.insert(Direction::Down, false);
        button_states.insert(Direction::Left, false);
        button_states.insert(Direction::Right, false);
        PlayerControlSystem {
            button_states,
            input_channel,
        } 
    }

    fn handle_inputs(&mut self) {
        // inputs need to be mapped to one button only because this breaks if multiple buttons correspond
        // to the same input.
        for (direction, pressed) in self.input_channel.try_iter() {
            self.button_states.insert(direction, pressed);
        }
    }
}

impl<'a> System<'a> for PlayerControlSystem {

    type SystemData = (
        WriteExpect<'a, PlayerHandle>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (player, mut velocities): Self::SystemData) {
        self.handle_inputs();
        let player_vel = velocities.get_mut(player.0);
        let mut new_vel = Vector::zero();
        for (dir, pressed) in &self.button_states {
            if pressed {
                player_vel += match dir {
                    Direction::Up => Vector::new(0., 1.),
                    Direction::Down => Vector::new(0., -1.),
                    Direction::Left => Vector::new(-1., 0.),
                    Direction::Right => Vector::new(1., 0.),
                }
            }
        }
        new_vel.normalize();
        *player_vel.0 = new_vel.normalize();
    }
}
