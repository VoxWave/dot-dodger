use kolli_desu::shapes::{Circle, ConvexPolygon, Shape};
use na::{Point2, Vector2};
use specs::prelude::ParallelIterator;
use specs::{Component, DenseVecStorage, Join, ParJoin, ReadStorage, System};

use bullet::BulletComponent;
use physics::Position;
use player::get_player;

#[derive(Component, Debug)]
enum Hitbox {
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

struct CollisionSystem;

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (
        ReadStorage<'a, Hitbox>, 
        ReadStorage<'a, Position>,
        ReadStorage<'a, BulletComponent>,
    );

    fn run(&mut self, (hitboxes, positions, bullets): Self::SystemData) {
        let player = get_player_handle();
        let (player_hitbox, player_pos) = (&hitboxes, &positions).join().get(player);
        (&hitboxes, &positions, &bullets)
            .par_join()
            .for_each(|(hbx, pos, _)|{

            });
    }
}
