use std::{
    ops::Mul,
    sync::{Arc, OnceLock},
    thread::sleep,
    time::{Duration, Instant},
};

use duck::{BASE_TRIANGLES, BEAK_TRIANGLE, EYE_TRIANGLE};
use pancurses::{
    initscr, Attributes, ColorPair, COLOR_BLACK, COLOR_RED, COLOR_WHITE, COLOR_YELLOW,
};
use quackie::{
    geometry::matrix::Matrix3D,
    graphics::{
        geometry_buffer::GeometryBuffer, pixel_data::PixelData, postprocess::antialias::Antialias,
        textured_triangle::TexturedTriangle2D, window_buffer::WindowBuffer,
    },
};

mod duck;

const ROTATION_SPEED: f64 = 2.0;

fn main() {
    let window = initscr();
    setup_pancurses();
    let time_reference = Instant::now();
    loop {
        let rotate_by = calculate_rotation(time_reference);
        draw_screen(rotate_by, &window);
        window.refresh();
        sleep(Duration::from_millis(10));
    }
}

fn setup_pancurses() {
    pancurses::start_color();
    pancurses::curs_set(0);
    pancurses::init_pair(0, COLOR_WHITE, COLOR_BLACK);
    pancurses::init_pair(1, COLOR_YELLOW, COLOR_BLACK);
    pancurses::init_pair(2, COLOR_RED, COLOR_BLACK);
}

fn calculate_rotation(time_reference: Instant) -> f64 {
    time_reference.elapsed().as_secs_f64().mul(-ROTATION_SPEED)
}

fn build_triangles() -> Arc<[TexturedTriangle2D]> {
    let mut triangles: Vec<TexturedTriangle2D> = BASE_TRIANGLES
        .iter()
        .map(|triangle| {
            TexturedTriangle2D::unipixeled(
                *triangle,
                PixelData::new('#', Attributes::new() | ColorPair(1)),
            )
        })
        .collect();
    let beak_triangle = TexturedTriangle2D::unipixeled(
        BEAK_TRIANGLE,
        PixelData::new('X', Attributes::new() | ColorPair(2)),
    );
    triangles.push(beak_triangle);
    let eye_triangle = TexturedTriangle2D::unipixeled(
        EYE_TRIANGLE,
        PixelData::new(' ', Attributes::new() | ColorPair(0)),
    );
    triangles.push(eye_triangle);
    Arc::from(triangles)
}

fn draw_screen(rotate_by: f64, window: &pancurses::Window) {
    let mut window_buffer = WindowBuffer::for_window(
        window,
        PixelData::new(' ', Attributes::new() | ColorPair(0)),
    );
    static TRIANGLES: OnceLock<Arc<[TexturedTriangle2D]>> = OnceLock::new();
    let triangles = TRIANGLES.get_or_init(build_triangles).clone();
    let mut geometry_buffer = GeometryBuffer::new(triangles.to_vec());
    geometry_buffer.transform(
        Matrix3D::transposition(0.5, 0.5)
            * Matrix3D::rotation(-rotate_by)
            * Matrix3D::transposition(-0.5, -0.5),
    );
    window_buffer.draw_geometry(&geometry_buffer);
    window_buffer.post_process(&Antialias);
    window_buffer.draw_screen(window);
}
