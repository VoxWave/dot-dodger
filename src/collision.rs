use crate::player::PlayerInputState;
use std::convert::Into;

use crate::na::{Point2, Vector2};
use kolli_desu::gjk::collides;
use kolli_desu::shapes::{Circle, ConvexPolygon, Shape};
use specs::prelude::ParallelIterator;
use specs::{Component, DenseVecStorage, ParJoin, ReadStorage, System, Join};

use crate::bullet::BulletComponent;
use crate::handle_death;
use crate::physics::Position;
use crate::utils::{downcast_point, downcast_vector};

#[derive(Component, Debug)]
pub enum Hitbox {
    Circle(Circle),
    ConvexPolygon(ConvexPolygon),
    Point(Point2<f32>),
}

impl Shape for Hitbox {
    fn start(&self) -> Vector2<f32> {
        use self::Hitbox::*;
        match self {
            Circle(c) => c.start(),
            ConvexPolygon(cp) => cp.start(),
            Point(p) => p.start(),
        }
    }

    fn farthest_in_dir(&self, dir: Vector2<f32>) -> Vector2<f32> {
        use self::Hitbox::*;
        match self {
            Circle(c) => c.farthest_in_dir(dir),
            ConvexPolygon(cp) => cp.farthest_in_dir(dir),
            Point(p) => p.farthest_in_dir(dir),
        }
    }
}

pub struct CollisionSystem;

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (
        ReadStorage<'a, PlayerInputState>,
        ReadStorage<'a, Hitbox>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BulletComponent>,
    );

    fn run(&mut self, (player, hitboxes, positions, bullets): Self::SystemData) {
        for (_, p_hitbox, p_pos) in (&player, &hitboxes, &positions).join() {
            let collision =
            (&hitboxes, &positions, &bullets)
                .par_join()
                .find_any(|(hitbox, position, _)| {
                    collides(
                        (*hitbox, downcast_point(position.0)),
                        (p_hitbox, downcast_point(p_pos.0)),
                    )
                });
            if let Some(_) = collision {
                handle_death();
            } else {
                println!("dedness not happend");
            }
        }
    }
}
