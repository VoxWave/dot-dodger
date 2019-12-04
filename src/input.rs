use std::collections::HashMap;
use ggez::event::{self, KeyCode, Button};

pub struct InputConfig {
    input_map: HashMap<RawInput, (u64, LogicalInput)>,
}

impl InputConfig {
    pub fn new() -> Self {
        let mut input_map = HashMap::new();
        input_map.insert(RawInput::KeyBoard(KeyCode::W), (0, LogicalInput::Axis(Axis::Y, AxisState::Positive)));
        input_map.insert(RawInput::KeyBoard(KeyCode::A), (0, LogicalInput::Axis(Axis::X, AxisState::Negative)));
        input_map.insert(RawInput::KeyBoard(KeyCode::S), (0, LogicalInput::Axis(Axis::Y, AxisState::Positive)));
        input_map.insert(RawInput::KeyBoard(KeyCode::D), (0, LogicalInput::Axis(Axis::X, AxisState::Negative)));

        InputConfig {
            input_map,
        }
    }

    // pub fn get_input(&self, raw_input: &RawInput) -> Option<(u64, LogicalInput)> {
    //     self.input_map.get(raw_input)
    // }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub enum RawInput {
    KeyBoard(KeyCode),
    Button(Button),
    Axis(event::Axis),
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub enum LogicalInput {
    Axis(Axis, AxisState),
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub enum Axis {
    X, Y,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub enum AxisState {
    //left and down
    Negative,
    Neutral,
    //right and up
    Positive,
}