use crate::na::{Point2, Vector2};
use kolli_desu::gjk::collides;
use kolli_desu::shapes::{Circle, ConvexPolygon, Shape};
use specs::prelude::ParallelIterator;
use specs::{Component, DenseVecStorage, ParJoin, ReadExpect, ReadStorage, System};

use crate::bullet::BulletComponent;
use crate::handle_death;
use crate::physics::Position;
use crate::player::PlayerHandle;

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
            Circle(c) => (c as Vector2<f32>).farthest_in_dir(dir),
            ConvexPolygon(cp) => cp as Vector2<f32>.farthest_in_dir(dir),
            Point(p) => (p as Vector2<f32>).farthest_in_dir(dir),
        }
    }
}

pub struct CollisionSystem;

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (
        ReadExpect<'a, PlayerHandle>,
        ReadStorage<'a, Hitbox>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BulletComponent>,
    );

    fn run(&mut self, (player, hitboxes, positions, bullets): Self::SystemData) {
        let (p_hitbox, p_pos) = (
            hitboxes.get(player.0).unwrap(),
            positions.get(player.0).unwrap(),
        );
        let collision = (&hitboxes, &positions, &bullets)
            .par_join()
            .find_any(|(hitbox, position, _)| collides((*hitbox, (position.0 as Vector2<f32>)), (p_hitbox, (p_pos.0 as Vector2<f32>))));
        if let Some(_) = collision {
            handle_death();
        }
    }
}
