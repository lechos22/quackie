use std::{
    ops::Mul,
    thread::sleep,
    time::{self, Duration},
};

use duck::{BASE_TRIANGLES, BEAK_TRIANGLE, EYE_TRIANGLE};
use geometry::Point2D;
use pancurses::{initscr, COLOR_BLACK, COLOR_RED, COLOR_YELLOW};

mod duck;
mod geometry;

#[repr(i16)]
enum ColorPair {
    Body = 1,
    Beak = 2,
}

const MIDDLE: Point2D = Point2D::new(0.5, 0.5);
const ROTATION_SPEED: f64 = 2.0;

fn main() {
    let window = initscr();
    pancurses::start_color();
    pancurses::curs_set(0);
    pancurses::init_pair(ColorPair::Body as i16, COLOR_YELLOW, COLOR_BLACK);
    pancurses::init_pair(ColorPair::Beak as i16, COLOR_RED, COLOR_BLACK);
    let time_reference = time::SystemTime::now();
    loop {
        let (max_y, max_x) = window.get_max_yx();
        let rotate_by = time_reference
            .elapsed()
            .unwrap_or_default()
            .as_secs_f64()
            .mul(-ROTATION_SPEED);
        for y in (0..max_y).rev() {
            for x in 0..max_x {
                let current_point =
                    Point2D::new((x as f64) / (max_x as f64), (y as f64) / (max_y as f64));
                let current_point_rotated = current_point.rotate_around(MIDDLE, rotate_by);
                let inside_base = BASE_TRIANGLES
                    .into_iter()
                    .any(|triangle| triangle.is_point_inside(&current_point_rotated));
                let inside_eye = EYE_TRIANGLE.is_point_inside(&current_point_rotated);
                let inside_beak = BEAK_TRIANGLE.is_point_inside(&current_point_rotated);
                if inside_beak {
                    window.color_set(ColorPair::Beak as i16);
                    window.mvaddch(y, x, '#');
                } else if inside_base && !inside_eye {
                    window.color_set(ColorPair::Body as i16);
                    window.mvaddch(y, x, '#');
                } else {
                    window.mvaddch(y, x, ' ');
                }
            }
        }
        window.refresh();
        sleep(Duration::from_millis(10));
    }
}
