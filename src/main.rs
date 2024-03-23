use std::{
    ops::Mul,
    thread::sleep,
    time::{Duration, Instant},
};

use duck::{BASE_TRIANGLES, BEAK_TRIANGLE, EYE_TRIANGLE};
use geometry::Vector2D;
use pancurses::{initscr, COLOR_BLACK, COLOR_RED, COLOR_YELLOW};

mod duck;
mod geometry;

#[repr(i16)]
enum ColorPair {
    Body = 1,
    Beak = 2,
}

const MIDDLE: Vector2D = Vector2D::new(0.5, 0.5);
const ROTATION_SPEED: f64 = 2.0;

fn main() {
    let window = initscr();
    setup_pancurses();
    let time_reference = Instant::now();
    loop {
        let (max_y, max_x) = window.get_max_yx();
        let rotate_by = calculate_rotation(time_reference);
        draw_screen(max_y, max_x, rotate_by, &window);
        window.refresh();
        sleep(Duration::from_millis(10));
    }
}

fn setup_pancurses() {
    pancurses::start_color();
    pancurses::curs_set(0);
    pancurses::init_pair(ColorPair::Body as i16, COLOR_YELLOW, COLOR_BLACK);
    pancurses::init_pair(ColorPair::Beak as i16, COLOR_RED, COLOR_BLACK);
}

fn calculate_rotation(time_reference: Instant) -> f64 {
    time_reference.elapsed().as_secs_f64().mul(-ROTATION_SPEED)
}

fn draw_screen(max_y: i32, max_x: i32, rotate_by: f64, window: &pancurses::Window) {
    for y in (0..max_y).rev() {
        for x in 0..max_x {
            draw_pixel(x, max_x, y, max_y, rotate_by, window);
        }
    }
}

fn draw_pixel(x: i32, max_x: i32, y: i32, max_y: i32, rotate_by: f64, window: &pancurses::Window) {
    let current_point = Vector2D::new((x as f64) / (max_x as f64), (y as f64) / (max_y as f64));
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
