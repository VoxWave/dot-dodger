use ggez::{
    event::{self, Button, KeyCode},
    input::gamepad::GamepadId,
};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub struct InputHandler {
    input_map: HashMap<InputType, (u64, LogicalInput)>,
    axis_states: HashMap<(GamepadId, event::Axis), f32>,
    button_states: HashSet<Button>,
    key_states: HashSet<KeyCode>,
}

impl InputHandler {
    pub fn new() -> Self {
        let mut input_map = HashMap::new();
        input_map.insert(
            InputType::KeyBoard(KeyCode::W),
            (0, LogicalInput::Axis(Axis::Y, AxisState::Positive)),
        );
        input_map.insert(
            InputType::KeyBoard(KeyCode::A),
            (0, LogicalInput::Axis(Axis::X, AxisState::Negative)),
        );
        input_map.insert(
            InputType::KeyBoard(KeyCode::S),
            (0, LogicalInput::Axis(Axis::Y, AxisState::Negative)),
        );
        input_map.insert(
            InputType::KeyBoard(KeyCode::D),
            (0, LogicalInput::Axis(Axis::X, AxisState::Positive)),
        );
        input_map.insert(
            InputType::KeyBoard(KeyCode::Up),
            (0, LogicalInput::Axis(Axis::Y, AxisState::Positive)),
        );
        input_map.insert(
            InputType::KeyBoard(KeyCode::Left),
            (0, LogicalInput::Axis(Axis::X, AxisState::Negative)),
        );
        input_map.insert(
            InputType::KeyBoard(KeyCode::Down),
            (0, LogicalInput::Axis(Axis::Y, AxisState::Negative)),
        );
        input_map.insert(
            InputType::KeyBoard(KeyCode::Right),
            (0, LogicalInput::Axis(Axis::X, AxisState::Positive)),
        );

        InputHandler {
            input_map,
            axis_states: HashMap::new(),
            button_states: HashSet::new(),
            key_states: HashSet::new(),
        }
    }

    fn update_set<T>(set: &mut HashSet<T>, value: T, pressed: bool)
    where
        T: Eq + Hash,
    {
        if pressed {
            set.insert(value);
        } else {
            set.remove(&value);
        }
    }

    pub fn handle_input(&mut self, raw_input: RawInput) {
        use RawInput::*;
        match raw_input {
            KeyBoard(keycode, pressed) => {
                Self::update_set(&mut self.key_states, keycode, pressed);
            }
            Button(button, pressed) => {
                Self::update_set(&mut self.button_states, button, pressed);
            }
            Axis(id, axis, value) => {
                self.axis_states.insert((id, axis), value);
            }
        }
    }

    pub fn get_inputs(&self) -> Vec<(u64, LogicalInput)> {
        use InputType::*;
        self.key_states
            .iter()
            .flat_map(|key| self.input_map.get(&KeyBoard(*key)).cloned())
            .chain(
                self.button_states
                    .iter()
                    .flat_map(|button| self.input_map.get(&Button(*button)).cloned()),
            )
            .chain(self.axis_states.iter().flat_map(|((id, axis), value)| {
                if let Some((player_id, l_input)) = self.input_map.get(&Axis(*id, *axis)) {
                    match l_input {
                        LogicalInput::Axis(axis, _) => {
                            if *value > 0.5 {
                                Some((*player_id, LogicalInput::Axis(*axis, AxisState::Positive)))
                            } else if *value < -0.5 {
                                Some((*player_id, LogicalInput::Axis(*axis, AxisState::Negative)))
                            } else {
                                Some((*player_id, LogicalInput::Axis(*axis, AxisState::Neutral)))
                            }
                        }
                    }
                } else {
                    None
                }
            }))
            .collect()
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub enum InputType {
    KeyBoard(KeyCode),
    Button(Button),
    Axis(GamepadId, event::Axis),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum RawInput {
    KeyBoard(KeyCode, bool),
    Button(Button, bool),
    Axis(GamepadId, event::Axis, f32),
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub enum LogicalInput {
    Axis(Axis, AxisState),
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub enum Axis {
    X,
    Y,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub enum AxisState {
    //left and down
    Negative,
    Neutral,
    //right and up
    Positive,
}
