use kolli_desu::shapes::Circle;
use specs::prelude::ParallelIterator;
use specs::{
    Builder, Component, Entities, Entity, LazyUpdate, NullStorage, Read, ReadExpect, System, Write, ReadStorage, ParJoin, Join,
};

use crate::collision::Hitbox;
use crate::na::{zero, Point2, Rotation2, Vector2};
use crate::physics::{Acceleration, Position, Velocity};
use crate::rendering::Visual;
use crate::Tick;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct NormalBullet;

pub struct BulletPatternSystem;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct SpiralSprayer;

pub struct NewBullets(pub Vec<Entity>);

impl<'a> System<'a> for BulletPatternSystem {
    type SystemData = (Entities<'a>, ReadStorage<'a, SpiralSprayer>, ReadStorage<'a, Position>, Read<'a, LazyUpdate>, ReadExpect<'a, Tick>);

    fn run(&mut self, (entities, spirals, positions, world, cur_tick): Self::SystemData) {

        let mut t = cur_tick.0 as f64;
        t /= 10.;
        let rotation = Rotation2::new(t);
        (&spirals, &positions).par_join().for_each(|(_, pos)| {
            create_bullet(
                world.create_entity(&entities),
                pos.0,
                rotation * Vector2::new(2., 2.),
                zero(),
                20./2.,
            );
        });
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
        .with(Visual::Sprite("bullet".to_string(), 1.))
        .with(Hitbox::Circle(Circle::new(Point2::new(0., 0.), rad as f32)))
        .with(Position(pos))
        .with(Velocity(vel))
        .with(Acceleration(acc))
        .with(NormalBullet)
        .build()
}
