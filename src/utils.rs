use crate::na::{Point2, Vector2};

pub fn upcast_vector(vector: Vector2<f32>) -> Vector2<f64> {
    Vector2::new(vector.x as f64, vector.y as f64)
}

pub fn downcast_point(point: Point2<f64>) -> Point2<f32> {
    Point2::new(point.x as f32, point.y as f32)
}

pub fn downcast_vector(vector: Vector2<f64>) -> Vector2<f32> {
    Vector2::new(vector.x as f32, vector.y as f32)
}