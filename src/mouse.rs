use crate::school_geometry::{angle, distance};
use crate::{point::Point, tangent::Tangent};

pub fn change_tangent_mouse(
    point: usize,
    mouse_x: f32,
    mouse_y: f32,
    bezier_curve_tangents: &mut [Tangent],
) {
    match point % 3 {
        1 => {
            let delta_point = Point::new(
                mouse_x - bezier_curve_tangents[point / 3].as_array()[1].x,
                mouse_y - bezier_curve_tangents[point / 3].as_array()[1].y,
            );
            bezier_curve_tangents[point / 3].change_point(Point::new(mouse_x, mouse_y), 1);
            bezier_curve_tangents[point / 3].change_point_delta(delta_point, 0);
            bezier_curve_tangents[point / 3].change_point_delta(delta_point, 2);
        }
        _ => {
            let main = point % 3;
            let other = 2 - (point % 3);

            let other_distance = distance(
                bezier_curve_tangents[point / 3].as_array()[other],
                bezier_curve_tangents[point / 3].as_array()[1],
            );
            bezier_curve_tangents[point / 3].change_point(Point::new(mouse_x, mouse_y), main);
            let main_angle = angle(
                bezier_curve_tangents[point / 3].as_array()[1],
                bezier_curve_tangents[point / 3].as_array()[main],
            );
            let other_angle = main_angle + std::f32::consts::PI; // + 180 degrees
            let center_clone = bezier_curve_tangents[point / 3].as_array()[1];
            bezier_curve_tangents[point / 3].change_point(
                Point::from_degree_system(other_angle, other_distance) + center_clone,
                other,
            );
        }
    }
}
