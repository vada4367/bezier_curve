use crate::{point::Point, tangent::Tangent, ACCURACY, POINT_COLOR, POINT_R};
use raylib::drawing::RaylibDrawHandle;
use raylib::prelude::RaylibDraw;

pub fn dots_to_curve(points: &Vec<Point>) -> Option<Vec<Point>> {
    let mut result = Vec::new();

    for i in 0..ACCURACY {
        let some_part_curve = dots_to_curve_at_time(points, (1f32 / ACCURACY as f32) * i as f32)?;

        result.extend(some_part_curve);
    }

    Some(result)
}

pub fn dots_to_curve_at_time(points: &Vec<Point>, time: f32) -> Option<Vec<Point>> {
    if points.is_empty() {
        return None;
    }

    if points.len() == 1 {
        return Some(vec![points[0]]);
    }

    let mut next_points = Vec::new();

    for p in 0..points.len() - 1 {
        next_points.push(Point::from_to_time(&points[p], &points[p + 1], time));
    }

    dots_to_curve_at_time(&next_points, time)
}

pub fn draw_dots_as_curve(points: &Vec<Point>, d: &mut RaylibDrawHandle<'_>) {
    let curve = dots_to_curve(points).unwrap();
    for dot in 0..curve.len() - 1 {
        //dot.draw_to_window(POINT_R, POINT_COLOR, d);

        d.draw_line_ex(
            raylib::ffi::Vector2 {
                x: curve[dot].x,
                y: curve[dot].y,
            },
            raylib::ffi::Vector2 {
                x: curve[dot + 1].x,
                y: curve[dot + 1].y,
            },
            POINT_R,
            POINT_COLOR,
        );
    }
}

pub fn draw_curve(curve: &Vec<Point>, d: &mut RaylibDrawHandle<'_>) {
    for dot in 0..curve.len() - 1 {
        //dot.draw_to_window(POINT_R, POINT_COLOR, d);

        d.draw_line_ex(
            raylib::ffi::Vector2 {
                x: curve[dot].x,
                y: curve[dot].y,
            },
            raylib::ffi::Vector2 {
                x: curve[dot + 1].x,
                y: curve[dot + 1].y,
            },
            POINT_R,
            POINT_COLOR,
        );
    }
}

pub fn get_curve(bezier_curve_tangents: &[Tangent]) -> Vec<Point> {
    let mut curve = Vec::new();
    for part in 0..bezier_curve_tangents.len() - 1 {
        curve.extend(
            dots_to_curve(&vec![
                bezier_curve_tangents[part].p2,
                bezier_curve_tangents[part].p3,
                bezier_curve_tangents[part + 1].p1,
                bezier_curve_tangents[part + 1].p2,
            ])
            .unwrap(),
        );
    }
    curve.push(bezier_curve_tangents[bezier_curve_tangents.len() - 1].p2);
    curve
}
