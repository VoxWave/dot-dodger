use kolli_desu::shapes::Circle;
use specs::{Builder, Component, Entities, LazyUpdate, NullStorage, Read, ReadStorage, System};

use crate::collision::Hitbox;
use crate::na::{zero, Point2, Vector2};
use crate::physics::{Acceleration, Position, Velocity};
use crate::rendering::Visual;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct BulletComponent;

pub struct BulletPatternSystem;

impl<'a> System<'a> for BulletPatternSystem {
    type SystemData = (Entities<'a>, Read<'a, LazyUpdate>);

    fn run(&mut self, (entities, world): Self::SystemData) {
        world
            .create_entity(&entities)
            .with(Visual::Circle([1., 0., 0., 1.], 5.))
            .with(Hitbox::Circle(Circle::new(Point2::new(0., 0.), 5.)))
            .with(Position(Point2::new(0., 0.)))
            .with(Velocity(Vector2::new(1., 1.)))
            .with(Acceleration(zero()))
            .with(BulletComponent)
            .build();
    }
}

fn create_bullet() {}
