use kolli_desu::shapes::{Circle, ConvexPolygon, Shape};
use na::{Point2, Vector2};
use specs::{Component, DenseVecStorage, ReadStorage, System};

use physics::Position;

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
    type SystemData = (ReadStorage<'a, Hitbox>, ReadStorage<'a, Position>);

    fn run(&mut self, (hitboxes, positions): Self::SystemData) {
        //(&hitboxes, &positions)
    }
}
