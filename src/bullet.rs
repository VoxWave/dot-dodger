use kolli_desu::shapes::Circle;
use specs::{Builder, Component, Entity, Entities, LazyUpdate, NullStorage, Read, System};

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
        create_bullet(world.create_entity(&entities), Point2::new(0., 0.), Vector2::new(10., 10.), zero(), 5.);
    }
}

fn create_bullet(builder: impl Builder, pos: Point2<f32>, vel: Vector2<f32>, acc: Vector2<f32>, rad: f64) -> Entity {
        builder
            .with(Visual::Circle([1., 0., 0., 1.], rad))
            .with(Hitbox::Circle(Circle::new(Point2::new(0., 0.), rad as f32)))
            .with(Position(pos))
            .with(Velocity(vel))
            .with(Acceleration(zero()))
            .with(BulletComponent)
            .build()
}
