use std::collections::{HashMap, HashSet};
use std::sync::mpsc::Receiver;

use crate::na::{zero, Vector2};

use crate::physics::Velocity;
use specs::{Component, Entity, Join, ReadStorage, System, VecStorage, WriteStorage};

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum AxisState {
    //left and down
    Negative,
    Neutral,
    //right and up
    Positive,
}
//x and y input values
#[derive(Component)]
#[storage(VecStorage)]
pub struct PlayerInputState(pub AxisState, pub AxisState);

pub enum PCSMessage {
    NewPlayer(Entity),
    Input(u64, AxisState, AxisState)
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

    pub fn drain_messages(&mut self) {

    }
}

impl<'a> System<'a> for PlayerControlSystem {
    type SystemData = (
        ReadStorage<'a, PlayerInputState>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (players, mut velocities): Self::SystemData) {
        self.drain_messages();
        (&players, &mut velocities).join().for_each(|(_, velocity)| {
            if velocity.0 == zero() {
                velocity.0 += Vector2::new(1., 1.);
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
