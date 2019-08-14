use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::{
        math::{zero, Point2, Rotation2, Vector2},
        transform::Transform,
    },
    ecs::{
        Builder, Component, Entities, Entity, LazyUpdate, NullStorage, Read, ReadExpect, System,
    },
    renderer::{SpriteSheet},
};
use kolli_desu::shapes::Circle;

use crate::Tick;
// use crate::collision::Hitbox;
use crate::physics::{Acceleration, Position, Velocity};
use crate::rendering;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct BulletComponent;

pub struct BulletPatternSystem;

impl<'a> System<'a> for BulletPatternSystem {
    type SystemData = (Entities<'a>, Read<'a, LazyUpdate>, ReadExpect<'a, Tick>);

    fn run(&mut self, (entities, world, cur_tick): Self::SystemData) {
        let mut t = cur_tick.0 as f64;
        t /= 100.;
        let rotation = Rotation2::new(t);
        let mut transform = Transform::default();
        transform.set_translation_xyz(200., 200., 0.);

        create_bullet(
            world.create_entity(&entities),
            transform,
            Point2::new(200., 200.),
            rotation * Vector2::new(2., 2.),
            zero(),
            5.,
        );
        println!("bullet created")
    }
}

fn create_bullet(
    builder: impl Builder,
    trans: Transform,
    pos: Point2<f64>,
    vel: Vector2<f64>,
    acc: Vector2<f64>,
    rad: f64,
) -> Entity {
    builder
        // .with(Visual::Circle([1., 0., 0., 1.], rad))
        // .with(Hitbox::Circle(Circle::new(Point2::new(0., 0.), rad as f32)))
        .with(trans)
        .with(Position(pos))
        .with(Velocity(vel))
        .with(Acceleration(acc))
        .with(BulletComponent)
        .build()
}
