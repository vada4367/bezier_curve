use crate::{point::Point, TANGENT_CIRCLE_R, TANGENT_COLOR};
use raylib::drawing::RaylibDrawHandle;
use raylib::prelude::RaylibDraw;

#[derive(Clone, Copy)]
pub struct Tangent {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
}

impl Tangent {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Tangent {
        Tangent {
            p1: Point::new(x1, y1),
            p2: Point::new((x1 + x2) * 0.5f32, (y1 + y2) * 0.5f32),
            p3: Point::new(x2, y2),
        }
    }

    pub fn as_array(&self) -> Vec<Point> {
        vec![self.p1, self.p2, self.p3]
    }

    pub fn change_point(&mut self, new_point: Point, index: usize) {
        match index {
            0 => self.p1 = new_point,
            1 => self.p2 = new_point,
            2 => self.p3 = new_point,
            _ => (),
        }
    }

    pub fn change_point_delta(&mut self, delta_point: Point, index: usize) {
        match index {
            0 => self.p1 = self.p1 + delta_point,
            1 => self.p2 = self.p2 + delta_point,
            2 => self.p3 = self.p3 + delta_point,
            _ => (),
        }
    }

    pub fn draw_to_window(&self, d: &mut RaylibDrawHandle<'_>) {
        for point in self.as_array().iter() {
            point.draw_to_window(TANGENT_CIRCLE_R, TANGENT_COLOR, d);
        }

        d.draw_line(
            self.p1.x as i32,
            self.p1.y as i32,
            self.p3.x as i32,
            self.p3.y as i32,
            TANGENT_COLOR,
        );
    }
}
