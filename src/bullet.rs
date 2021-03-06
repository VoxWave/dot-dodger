use kolli_desu::shapes::Circle;
use specs::{
    Builder, Component, Entities, Entity, LazyUpdate, NullStorage, Read, ReadExpect, System, Write,
};

use crate::collision::Hitbox;
use crate::na::{zero, Point2, Rotation2, Vector2};
use crate::physics::{Acceleration, Position, Velocity};
use crate::rendering::Visual;
use crate::utils::upcast_vector;
use crate::Tick;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct BulletComponent;

pub struct BulletPatternSystem;

pub struct NewBullets(pub Vec<Entity>);

impl<'a> System<'a> for BulletPatternSystem {
    type SystemData = (Entities<'a>, Read<'a, LazyUpdate>, ReadExpect<'a, Tick>);

    fn run(&mut self, (entities, world, cur_tick): Self::SystemData) {
        let mut t = cur_tick.0 as f64;
        t /= 10.;
        let rotation = Rotation2::new(t);
        create_bullet(
            world.create_entity(&entities),
            Point2::new(200., 200.),
            rotation * Vector2::new(2., 2.),
            zero(),
            20./2.,
        );
    }
}

pub fn create_bullet(
    builder: impl Builder,
    pos: Point2<f64>,
    vel: Vector2<f64>,
    acc: Vector2<f64>,
    rad: f64,
) -> Entity {
    builder
        .with(Visual::Sprite("bullet".to_string(), 20./64.))
        .with(Hitbox::Circle(Circle::new(Point2::new(0., 0.), rad as f32)))
        .with(Position(pos))
        .with(Velocity(vel))
        .with(Acceleration(acc))
        .with(BulletComponent)
        .build()
}
