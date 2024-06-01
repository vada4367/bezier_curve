use point::Point;
use raylib::prelude::*;

pub mod curve;
pub mod mouse;
pub mod point;
pub mod school_geometry;
pub mod tangent;

use curve::{draw_curve, get_curve};
use mouse::change_tangent_mouse;
use tangent::Tangent;

pub const WIDTH: i32 = 800;
pub const HEIGHT: i32 = 600;

pub const ACCURACY: usize = 15;

pub const TANGENT_CIRCLE_R: f32 = ((WIDTH * WIDTH + HEIGHT * HEIGHT) / 100000) as f32;
pub const POINT_R: f32 = ((WIDTH * WIDTH + HEIGHT * HEIGHT) / 200000) as f32;

pub const TANGENT_COLOR: Color = Color::RED;
pub const POINT_COLOR: Color = Color::WHITE;

pub const NEW_TANGENT_SIZE: f32 = ((WIDTH * WIDTH + HEIGHT * HEIGHT) / 10000) as f32;
pub const NEW_TANGENT_SIZE_HALF: f32 = NEW_TANGENT_SIZE / 2f32;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Bezier Curves!")
        .build();

    let mut bezier_curve_tangents: Vec<Tangent> = vec![
        Tangent::new(
            0f32,
            HEIGHT as f32,
            HEIGHT as f32 * 0.1f32,
            HEIGHT as f32 * 0.8f32,
        ),
        Tangent::new(
            WIDTH as f32 * 0.5,
            HEIGHT as f32 * 0.1f32,
            WIDTH as f32,
            HEIGHT as f32,
        ),
    ];

    let mut curve = get_curve(&bezier_curve_tangents);

    let (mut mouse_x, mut mouse_y): (f32, f32);

    let mut change_point: Option<usize> = None;

    let mut new_tangent: Option<Tangent> = None;
    let mut min_dst = POINT_R + 1f32;
    let mut main_point_i: Option<usize> = None;

    while !rl.window_should_close() {
        (mouse_x, mouse_y) = unsafe {
            (
                raylib::ffi::GetMousePosition().x,
                raylib::ffi::GetMousePosition().y,
            )
        };

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(0x28, 0x28, 0x28, 0xff));

        draw_curve(&curve, &mut d);

        if unsafe { raylib::ffi::IsMouseButtonDown(0) } {
            for (i, tangent) in bezier_curve_tangents.iter().enumerate() {
                for point in 0..tangent.as_array().len() {
                    if (tangent.as_array()[point].x - mouse_x)
                        * (tangent.as_array()[point].x - mouse_x)
                        + (tangent.as_array()[point].y - mouse_y)
                            * (tangent.as_array()[point].y - mouse_y)
                        < TANGENT_CIRCLE_R * TANGENT_CIRCLE_R
                    {
                        change_point = Some(i * 3 + point);
                    }
                }
            }
        } else {
            change_point = None;
        }

        if unsafe { raylib::ffi::IsMouseButtonPressed(0) } && change_point.is_none() {
            for (i, point) in curve.iter().enumerate() {
                if (point.x - mouse_x) * (point.x - mouse_x)
                    + (point.y - mouse_y) * (point.y - mouse_y)
                    < min_dst * min_dst
                {
                    min_dst = (point.x - mouse_x) * (point.x - mouse_x)
                        + (point.y - mouse_y) * (point.y - mouse_y);
                    main_point_i = Some(i);
                }
            }

            if min_dst < POINT_R * POINT_R {
                let main_point = curve[main_point_i.unwrap()];
                let previous_point =
                    if main_point_i.unwrap() >= 1 && curve.len() > main_point_i.unwrap() - 1 {
                        curve[main_point_i.unwrap() - 1]
                    } else {
                        bezier_curve_tangents[0].p2
                    };

                let next_point = if curve.len() > main_point_i.unwrap() + 1 {
                    curve[main_point_i.unwrap() + 1]
                } else {
                    bezier_curve_tangents[bezier_curve_tangents.len() - 1].p2
                };

                let new_tangent_angle = (school_geometry::angle(previous_point, main_point)
                    + school_geometry::angle(main_point, next_point))
                    * 0.5f32;

                let p1 = Point::from_degree_system(
                    new_tangent_angle + std::f32::consts::PI,
                    NEW_TANGENT_SIZE_HALF,
                ) + main_point;
                let p2 = Point::from_degree_system(new_tangent_angle, NEW_TANGENT_SIZE_HALF)
                    + main_point;

                new_tangent = Some(Tangent::new(p1.x, p1.y, p2.x, p2.y));
            }
        } else {
            new_tangent = None;
            min_dst = POINT_R * POINT_R + 1f32;
            main_point_i = None;
        }
        if unsafe { raylib::ffi::IsMouseButtonPressed(1) } {
            for (i, tangent) in bezier_curve_tangents.clone().iter().enumerate() {
                if (tangent.p2.x - mouse_x).powf(2f32) + (tangent.p2.y - mouse_y).powf(2f32)
                    < TANGENT_CIRCLE_R.powf(2f32)
                {
                    bezier_curve_tangents.remove(i);
                    curve = get_curve(&bezier_curve_tangents);
                }
            }
        }

        if let Some(point) = change_point {
            change_tangent_mouse(point, mouse_x, mouse_y, &mut bezier_curve_tangents);
            curve = get_curve(&bezier_curve_tangents);
        }
        if let Some(tangent) = new_tangent {
            bezier_curve_tangents.insert((main_point_i.unwrap() - 1) / ACCURACY + 1, tangent);
            curve = get_curve(&bezier_curve_tangents);
        }

        for tangent in &bezier_curve_tangents {
            tangent.draw_to_window(&mut d);
        }
    }
}
