use raylib::color::Color;
use raylib::drawing::RaylibDrawHandle;
use raylib::prelude::RaylibDraw;

#[derive(Clone, Copy, PartialEq, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl Point {
    pub fn new(_x: f32, _y: f32) -> Point {
        Point { x: _x, y: _y }
    }

    pub fn from_degree_system(angle: f32, dst: f32) -> Point {
        Point::new(dst * angle.cos(), dst * angle.sin())
    }

    pub fn draw_to_window(&self, r: f32, color: Color, d: &mut RaylibDrawHandle<'_>) {
        d.draw_circle(self.x as i32, self.y as i32, r, color);
    }

    pub fn from_to_time(p1: &Point, p2: &Point, time: f32) -> Point {
        Point {
            x: p1.x + (p2.x - p1.x) * time,
            y: p1.y + (p2.y - p1.y) * time,
        }
    }
}
