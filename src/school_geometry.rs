use crate::point::Point;

pub fn distance(p1: Point, p2: Point) -> f32 {
    ((p1.x - p2.x).powf(2f32) + (p1.y - p2.y).powf(2f32)).powf(0.5f32)
}

pub fn angle(p1: Point, p2: Point) -> f32 {
    (p2.y - p1.y).atan2(p2.x - p1.x)
}
