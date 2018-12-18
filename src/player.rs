use piston_window::keyboard::Key;
use piston_window::{Button, Input};
use std::collections::HashMap;
use std::sync::mpsc::Receiver;

use crate::na::{zero, Vector2};

use crate::physics::Velocity;
use specs::{Entity, System, WriteExpect, WriteStorage};

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct PlayerHandle(pub Entity);

pub struct PlayerControlSystem {
    button_map: HashMap<Button, Direction>,
    button_states: HashMap<Direction, bool>,
    input_channel: Receiver<Input>,
}

impl PlayerControlSystem {
    pub fn new(input_channel: Receiver<Input>) -> Self {
        let mut button_states = HashMap::new();
        button_states.insert(Direction::Up, false);
        button_states.insert(Direction::Down, false);
        button_states.insert(Direction::Left, false);
        button_states.insert(Direction::Right, false);
        let mut button_map = HashMap::new();
        //TODO: make this configurable and not hardcoded.
        button_map.insert(Button::Keyboard(Key::W), Direction::Up);
        button_map.insert(Button::Keyboard(Key::A), Direction::Left);
        button_map.insert(Button::Keyboard(Key::S), Direction::Down);
        button_map.insert(Button::Keyboard(Key::D), Direction::Right);
        button_map.insert(Button::Keyboard(Key::Up), Direction::Up);
        button_map.insert(Button::Keyboard(Key::Left), Direction::Left);
        button_map.insert(Button::Keyboard(Key::Down), Direction::Down);
        button_map.insert(Button::Keyboard(Key::Right), Direction::Right);

        PlayerControlSystem {
            button_map,
            button_states,
            input_channel,
        }
    }

    fn handle_inputs(&mut self) {
        for input in self.input_channel.try_iter() {
            match input {
                Input::Button(bs) => match self.button_map.get(&bs.button) {
                    Some(dir) => {
                        use piston_window::ButtonState::*;
                        let pressed = self.button_states.get_mut(dir).unwrap();
                        match bs.state {
                            Press => {
                                *pressed = true;
                            }
                            Release => {
                                *pressed = false;
                            }
                        }
                    }
                    None => {}
                },
                _ => {}
            }
        }
    }
}

impl<'a> System<'a> for PlayerControlSystem {
    type SystemData = (WriteExpect<'a, PlayerHandle>, WriteStorage<'a, Velocity>);

    fn run(&mut self, (player, mut velocities): Self::SystemData) {
        self.handle_inputs();
        let player_vel = velocities.get_mut(player.0).unwrap();
        let mut new_vel = zero::<Vector2<f32>>();
        for (dir, pressed) in &self.button_states {
            if *pressed {
                new_vel += match dir {
                    Direction::Up => Vector2::new(0., 1.),
                    Direction::Down => Vector2::new(0., -1.),
                    Direction::Left => Vector2::new(-1., 0.),
                    Direction::Right => Vector2::new(1., 0.),
                }
            }
        }
        if new_vel != zero() {
            player_vel.0 = new_vel.normalize()*2.7;
        } else {
            player_vel.0 = new_vel;
        }
    }
}
