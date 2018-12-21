use crate::rendering::Visual::*;
use piston_window::context::Context;
use piston_window::{ellipse, G2d};
use specs::{Component, DenseVecStorage, Join, ReadStorage};

use crate::physics::Position;
#[derive(Component)]
pub enum Visual {
    Circle([f32; 4], f64),
    Sprite,
}

pub fn render(
    c: Context,
    g: &mut G2d,
    (positions, visuals): (ReadStorage<Position>, ReadStorage<Visual>),
) {
    (&positions, &visuals).join().for_each(|(pos, vis)| {
        let x = pos.0.x as f64;
        let y = pos.0.y as f64;
        match vis {
            Circle(color, radius) => {
                //Figure out where to fetch the resolution of the window so that I don't have to hardcore 480 for the height here.
                ellipse(
                    *color,
                    [x - radius, 480. - (y - radius), 2. * radius, 2. * radius],
                    c.transform,
                    g,
                );
            }
            _ => {}
        }
    });
}
