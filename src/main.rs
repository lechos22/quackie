use std::{thread::sleep, time::Duration};

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

fn main() {
    let window = initscr();
    pancurses::start_color();
    pancurses::curs_set(0);
    pancurses::init_pair(ColorPair::Body as i16, COLOR_YELLOW, COLOR_BLACK);
    pancurses::init_pair(ColorPair::Beak as i16, COLOR_RED, COLOR_BLACK);
    loop {
        let (max_y, max_x) = window.get_max_yx();
        for y in (0..max_y).rev() {
            for x in 0..max_x {
                let current_point =
                    Point2D::new((x as f64) / (max_x as f64), (y as f64) / (max_y as f64));
                let inside_base = BASE_TRIANGLES
                    .into_iter()
                    .any(|triangle| triangle.is_point_inside(&current_point));
                let inside_eye = EYE_TRIANGLE.is_point_inside(&current_point);
                let inside_beak = BEAK_TRIANGLE.is_point_inside(&current_point);
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
