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

    let mut bezier_curve_tangents: Vec<Tangent> = vec![Tangent { p1: Point { x: -1.1081944, y: 350.8687 }, p2: Point { x: 28.0, y: 327.0 }, p3: Point { x: 78.0, y: 286.0 } }, Tangent { p1: Point { x: 159.85805, y: 183.04123 }, p2: Point { x: 146.0, y: 135.0 }, p3: Point { x: 131.0, y: 83.0 } }, Tangent { p1: Point { x: 78.30553, y: 273.0171 }, p2: Point { x: 77.0, y: 323.0 }, p3: Point { x: 70.0, y: 591.0 } }, Tangent { p1: Point { x: 57.37322, y: 566.86816 }, p2: Point { x: 61.0, y: 517.0 }, p3: Point { x: 69.0, y: 407.0 } }, Tangent { p1: Point { x: 91.20135, y: 390.64404 }, p2: Point { x: 117.0, y: 383.0 }, p3: Point { x: 144.0, y: 375.0 } }, Tangent { p1: Point { x: 172.0, y: 408.0 }, p2: Point { x: 173.0, y: 431.0 }, p3: Point { x: 175.81839, y: 495.82318 } }, Tangent { p1: Point { x: 162.19492, y: 557.83997 }, p2: Point { x: 207.0, y: 547.0 }, p3: Point { x: 269.0, y: 532.0 } }, Tangent { p1: Point { x: 364.07703, y: 453.62918 }, p2: Point { x: 358.0, y: 404.0 }, p3: Point { x: 352.0, y: 355.0 } }, Tangent { p1: Point { x: 265.0, y: 383.0 }, p2: Point { x: 257.0, y: 449.0 }, p3: Point { x: 248.05573, y: 522.7902 } }, Tangent { p1: Point { x: 295.1053, y: 568.94257 }, p2: Point { x: 339.0, y: 545.0 }, p3: Point { x: 405.0, y: 509.0 } }, Tangent { p1: Point { x: 481.028, y: 302.6421 }, p2: Point { x: 487.0, y: 253.0 }, p3: Point { x: 503.0, y: 120.0 } }, Tangent { p1: Point { x: 472.0, y: 103.0 }, p2: Point { x: 456.0, y: 153.0 }, p3: Point { x: 434.58527, y: 219.92105 } }, Tangent { p1: Point { x: 415.85663, y: 574.5271 }, p2: Point { x: 450.0, y: 538.0 }, p3: Point { x: 484.14337, y: 501.4729 } }, Tangent { p1: Point { x: 579.7147, y: 272.81604 }, p2: Point { x: 584.0, y: 223.0 }, p3: Point { x: 592.0, y: 130.0 } }, Tangent { p1: Point { x: 556.77295, y: 101.46069 }, p2: Point { x: 550.0, y: 151.0 }, p3: Point { x: 515.0, y: 407.0 } }, Tangent { p1: Point { x: 538.156, y: 575.8029 }, p2: Point { x: 564.0, y: 533.0 }, p3: Point { x: 589.844, y: 490.19708 } }, Tangent { p1: Point { x: 619.3622, y: 382.4076 }, p2: Point { x: 663.0, y: 358.0 }, p3: Point { x: 722.0, y: 325.0 } }, Tangent { p1: Point { x: 754.8991, y: 411.34912 }, p2: Point { x: 749.0, y: 461.0 }, p3: Point { x: 737.0, y: 562.0 } }, Tangent { p1: Point { x: 667.0, y: 556.0 }, p2: Point { x: 648.0, y: 533.0 }, p3: Point { x: 608.2065, y: 484.82886 } }, Tangent { p1: Point { x: 611.33813, y: 413.69785 }, p2: Point { x: 643.0, y: 375.0 }, p3: Point { x: 679.0, y: 331.0 } }, Tangent { p1: Point { x: 682.4137, y: 350.96548 }, p2: Point { x: 738.0, y: 368.0 }, p3: Point { x: 800.0, y: 387.0 } }, Tangent { p1: Point { x: 793.625, y: 327.34955 }, p2: Point { x: 775.0, y: 307.0 }, p3: Point { x: 721.0, y: 248.0 } }];

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

        println!("{:?}", bezier_curve_tangents);
    }
}
