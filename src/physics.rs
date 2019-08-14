use amethyst::core::math::{Point2, Vector2};
use amethyst::ecs::prelude::ParallelIterator;
use amethyst::ecs::{
    Component, DenseVecStorage, ParJoin, ReadStorage, System, VecStorage, WriteStorage,
};

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Position(pub Point2<f64>);

#[derive(Debug, Component)]
pub struct Velocity(pub Vector2<f64>);

#[derive(Debug, Component)]
pub struct Acceleration(pub Vector2<f64>);

pub struct PhysicsSystem;

impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Acceleration>,
    );

    fn run(&mut self, (mut positions, mut velocities, accelerations): Self::SystemData) {
        (&mut positions, &mut velocities, &accelerations)
            .par_join()
            .for_each(|(pos, vel, acc)| {
                pos.0 += vel.0;
                vel.0 += acc.0;
            });
    }
}
