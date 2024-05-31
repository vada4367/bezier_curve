use raylib::prelude::*;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

const ACCURACY: i32 = 100;

const TANGENT_CIRCLE_R: f32 = 10f32;
const POINT_R: f32 = 5f32;

const TANGENT_COLOR: Color = Color::RED;
const POINT_COLOR: Color = Color::WHITE;

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

    let (mut mouse_x, mut mouse_y) = (0f32, 0f32);

    let mut change_point: Option<usize> = None;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(0x28, 0x28, 0x28, 0xff));

        draw_curve(&curve, &mut d);

        if unsafe { raylib::ffi::IsMouseButtonDown(0) } {
            (mouse_x, mouse_y) = unsafe {
                (
                    raylib::ffi::GetMousePosition().x,
                    raylib::ffi::GetMousePosition().y,
                )
            };
            for (i, tangent) in bezier_curve_tangents.iter().enumerate() {
                for point in 0..tangent.as_array().len() {
                    if (tangent.as_array()[point].x - mouse_x).powf(2f32)
                        + (tangent.as_array()[point].y - mouse_y).powf(2f32)
                        < TANGENT_CIRCLE_R * TANGENT_CIRCLE_R
                    {
                        change_point = Some(i * 3 + point);
                    }
                }
            }
        } else {
            change_point = None;
        }

        if let Some(point) = change_point {
            change_tangent_mouse(point, mouse_x, mouse_y, &mut bezier_curve_tangents);
            curve = get_curve(&bezier_curve_tangents);
        }

        for tangent in &bezier_curve_tangents {
            tangent.draw_to_window(&mut d);
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default)]
struct Point {
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

#[derive(Clone, Copy)]
struct Tangent {
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
    for dot in dots_to_curve(points).unwrap() {
        dot.draw_to_window(POINT_R, POINT_COLOR, d);
    }
}

pub fn draw_curve(curve: &Vec<Point>, d: &mut RaylibDrawHandle<'_>) {
    for dot in curve {
        dot.draw_to_window(POINT_R, POINT_COLOR, d);
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
    curve
}

pub fn distance(p1: Point, p2: Point) -> f32 {
    ((p1.x - p2.x).powf(2f32) + (p1.y - p2.y).powf(2f32)).powf(0.5f32)
}

pub fn angle(p1: Point, p2: Point) -> f32 {
    (p2.y - p1.y).atan2(p2.x - p1.x)
}

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
