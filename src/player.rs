use crate::na::{zero, Vector2};

use crate::physics::Velocity;
use amethyst::{
    ecs::{Entity, Read, System, WriteExpect, WriteStorage},
    input::InputHandler,
};

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct PlayerHandle(pub Entity);

pub struct PlayerControlSystem;

impl<'s> System<'s> for PlayerControlSystem {
    type SystemData = (Read<'s, InputHandler<String, String>>, WriteExpect<'s, PlayerHandle>, WriteStorage<'s, Velocity>);

    fn run(&mut self, (inputs, player, mut velocities): Self::SystemData) {
        let player_vel = velocities.get_mut(player.0).unwrap();

        let x = inputs.axis_value("left_right").unwrap();
        let y = inputs.axis_value("up_down").unwrap();
        let mut new_vel = Vector2::new(x, y);
        
        if new_vel != zero() {
            player_vel.0 = new_vel.normalize() * 2.7;
        } else {
            player_vel.0 = new_vel;
        }
    }
}
