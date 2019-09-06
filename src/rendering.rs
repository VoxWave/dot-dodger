use specs::{Component, DenseVecStorage, Join, ReadStorage};
use ggez::Context;

use crate::physics::Position;
use crate::rendering::Visual::*;
#[derive(Component)]
pub enum Visual {
    Circle([f32; 4], f64),
    Sprite(()),
}

pub fn render(
    c: &mut Context,
    (positions, visuals): (ReadStorage<Position>, ReadStorage<Visual>),
) {
    (&positions, &visuals).join().for_each(|(pos, vis)| {
        let x = pos.0.x as f64;
        let y = pos.0.y as f64;
        match vis {
            Circle(color, radius) => {},
            Sprite(img) => {},
        }
    });
}
