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
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        ReadExpect<'a, Tick>,
        Option<Write<'a, NewBullets>>,
    );

    fn run(&mut self, (entities, world, cur_tick, mut new_bullets): Self::SystemData) {
        let mut t = cur_tick.0 as f32;
        if t.rem_euclid(5.) == 0. {
            t /= 10.;
            let rotation = Rotation2::new(t);
            let bullet = create_bullet(
                world.create_entity(&entities),
                Point2::new(200., 200.),
                upcast_vector(rotation * Vector2::new(2., 2.)),
                zero(),
                5.,
            );
            match new_bullets {
                Some(mut new_bullets) => new_bullets.0.push(bullet),
                None => {}
            };
        }
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
        .with(Visual::Sprite("bullet".to_string()))
        .with(Hitbox::Circle(Circle::new(Point2::new(0., 0.), rad as f32)))
        .with(Position(pos))
        .with(Velocity(vel))
        .with(Acceleration(zero()))
        .with(BulletComponent)
        .build()
}
