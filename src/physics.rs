use na::{Point2, Vector2};
use specs::prelude::ParallelIterator;
use specs::{Component, DenseVecStorage, ParJoin, ReadStorage, System, VecStorage, WriteStorage};

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Position(Point2<f32>);

#[derive(Debug, Component)]
struct Velocity(Vector2<f32>);

#[derive(Debug, Component)]
struct Acceleration(Vector2<f32>);

struct PhysicsSystem;

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
