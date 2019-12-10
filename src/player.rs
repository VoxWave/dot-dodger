use std::collections::{HashMap, HashSet};
use std::sync::mpsc::Receiver;

use crate::{
    na::{zero, Vector2},
    physics::Velocity,
    input::{LogicalInput, Axis, AxisState},
};

use specs::{Component, Entity, Join, ReadStorage, System, VecStorage, WriteStorage};

const PLAYER_SPEED: f64 = 3.;

//x and y input values
#[derive(Component)]
#[storage(VecStorage)]
pub struct PlayerInputState(pub AxisState, pub AxisState);

pub enum PCSMessage {
    NewPlayer(Entity),
    Input(u64, LogicalInput),
}

pub struct PlayerControlSystem {
    players: Vec<Entity>,
    message_channel: Receiver<PCSMessage>,
}

impl PlayerControlSystem {
    pub fn new(input_channel: Receiver<PCSMessage>) -> Self {
        // let button_states = HashSet::new();
        // let mut button_map = HashMap::new();
        // //TODO: make this configurable and not hardcoded.
        // // button_map.insert(Button::Keyboard(Key::W), Direction::Up);
        // // button_map.insert(Button::Keyboard(Key::A), Direction::Left);
        // // button_map.insert(Button::Keyboard(Key::S), Direction::Down);
        // // button_map.insert(Button::Keyboard(Key::D), Direction::Right);
        // // button_map.insert(Button::Keyboard(Key::Up), Direction::Up);
        // // button_map.insert(Button::Keyboard(Key::Left), Direction::Left);
        // // button_map.insert(Button::Keyboard(Key::Down), Direction::Down);
        // // button_map.insert(Button::Keyboard(Key::Right), Direction::Right);

        PlayerControlSystem {
            players: Vec::with_capacity(4),
            message_channel: input_channel,
        }
    }

    pub fn drain_messages<'a>(&mut self, player_inputs: &mut WriteStorage<'a, PlayerInputState>) {
        use PCSMessage::*;
        for message in self.message_channel.try_iter() {
            match message {
                NewPlayer(entity) => self.players.push(entity),
                Input(player_id, LogicalInput::Axis(axis, state)) => {
                    if let Some(player_input) = self.players.get(player_id as usize).and_then(|entity| player_inputs.get_mut(*entity)) {
                        match axis {
                            Axis::X => player_input.0 = state,
                            Axis::Y => player_input.1 = state,
                        }
                    }
                },
                _ => {},
            }
        }
    }
}

impl<'a> System<'a> for PlayerControlSystem {
    type SystemData = (
        WriteStorage<'a, PlayerInputState>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (mut players, mut velocities): Self::SystemData) {
        (&mut players).join().for_each(|mut player_input| {
            player_input.0 = AxisState::Neutral;
            player_input.1 = AxisState::Neutral;
        });
        self.drain_messages(&mut players);
        (&players, &mut velocities).join().for_each(|(player_input, velocity)| {
            let new_velocity = match player_input {
                PlayerInputState(x, y) => {
                    let state_to_float = |x| match x {
                        &AxisState::Positive => 1.,
                        &AxisState::Neutral => 0.,
                        &AxisState::Negative => -1.,
                    };
                    Vector2::new(state_to_float(x), state_to_float(y)) 
                },
            };
            if new_velocity == zero() {
                velocity.0 = new_velocity;
            } else {
                velocity.0 = new_velocity.normalize() * PLAYER_SPEED;
            }
        });
        // let mut new_vel = zero::<Vector2<f64>>();
        // let pressed_buttons: HashSet<&Direction> = self
        //     .button_states
        //     .iter()
        //     .flat_map(|b| self.button_map.get(b))
        //     .collect();
        // for dir in pressed_buttons {
        //     new_vel += match dir {
        //         Direction::Up => Vector2::new(0., 1.),
        //         Direction::Down => Vector2::new(0., -1.),
        //         Direction::Left => Vector2::new(-1., 0.),
        //         Direction::Right => Vector2::new(1., 0.),
        //     }
        // }
        // if new_vel != zero() {
        //     player_vel.0 = new_vel.normalize() * 2.7;
        // } else {
        //     player_vel.0 = new_vel;
        // }
    }
}
