use std::{ops::Range, thread};

use pancurses::Window;

use crate::geometry::vector::Vector2D;

use super::{
    geometry_buffer::GeometryBuffer, pixel_data::PixelData, postprocess::PostProcessShader,
    textured_triangle::TexturedTriangle2D,
};

pub struct WindowBuffer {
    width: usize,
    height: usize,
    min_wh: f64,
    dx: f64,
    dy: f64,
    pixels: Vec<PixelData>,
    background: PixelData,
}

impl WindowBuffer {
    pub fn new(width: usize, height: usize, background: PixelData) -> Self {
        let pixels = vec![background; width * height];
        let min_wh = height.min(width / 2) as f64;
        let dx = ((width / 2) as f64 - min_wh) * 0.5 / min_wh;
        let dy = (height as f64 - min_wh) * 0.5 / min_wh;
        Self {
            width,
            height,
            min_wh,
            dx,
            dy,
            pixels,
            background,
        }
    }
    pub fn for_window(window: &Window, background: PixelData) -> Self {
        Self::new(
            (window.get_max_x() - window.get_beg_x()) as usize,
            (window.get_max_y() - window.get_beg_y()) as usize,
            background,
        )
    }
    pub fn set_pixel_at(&mut self, x: usize, y: usize, pixel: PixelData) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = pixel
        }
    }
    pub fn draw_geometry(&mut self, geometry: &GeometryBuffer) {
        const THREADS: usize = 8;
        self.pixels = thread::scope(|scope| {
            let mut handles = Vec::with_capacity(THREADS);
            let triangles = geometry.triangles();
            for thread_n in 0..THREADS {
                let size = self.height * self.width;
                let chunk = size / THREADS;
                let self_ref = &self;
                handles.push(scope.spawn(move || {
                    self_ref.render_range(
                        triangles,
                        (thread_n * chunk)..((thread_n + 1) * chunk).max(size),
                    )
                }));
            }
            let mut outputs = Vec::with_capacity(THREADS);
            for handle in handles {
                let result = handle.join().unwrap();
                outputs.push(result);
            }
            outputs.concat()
        });
    }
    fn render_range(
        &self,
        triangles: &[TexturedTriangle2D],
        range: Range<usize>,
    ) -> Vec<PixelData> {
        let mut output = Vec::with_capacity(range.len());
        for n in range {
            let y = n / self.width;
            let x = n % self.width;
            let relative_y = y as f64 / self.min_wh - self.dy;
            let relative_x = (x as f64) * 0.5 / self.min_wh - self.dx;
            let point = Vector2D::new(relative_x, relative_y);
            let mut broke = false;
            for triangle in triangles.iter().rev() {
                if triangle.is_point_inside(&point) {
                    output.push(triangle.get_pixel_at(&point));
                    broke = true;
                    break;
                }
            }
            if !broke {
                output.push(self.background);
            }
        }
        output
    }
    pub fn post_process(&mut self, shader: &dyn PostProcessShader) {
        const THREADS: usize = 8;
        self.pixels = thread::scope(|scope| {
            let mut handles = Vec::with_capacity(THREADS);
            for thread_n in 0..THREADS {
                let size = self.height * self.width;
                let chunk = size / THREADS;
                let self_ref = &self;
                handles.push(scope.spawn(move || {
                    self_ref.post_process_range(
                        shader,
                        (thread_n * chunk)..((thread_n + 1) * chunk).max(size),
                    )
                }));
            }
            let mut outputs = Vec::with_capacity(THREADS);
            for handle in handles {
                let result = handle.join().unwrap();
                outputs.push(result);
            }
            outputs.concat()
        });
    }
    pub fn post_process_range(
        &self,
        shader: &dyn PostProcessShader,
        range: Range<usize>,
    ) -> Vec<PixelData> {
        let mut output = Vec::with_capacity(range.len());
        for n in range {
            let y = n / self.width;
            let x = n % self.width;
            output.push(shader.execute(self, x, y));
        }
        output
    }
    pub fn get_pixel_at(&self, x: usize, y: usize) -> PixelData {
        if x >= self.width || y >= self.height {
            self.background
        } else {
            self.pixels[y * self.width + x]
        }
    }
    pub fn draw_screen(&self, window: &Window) {
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let pixel = self.get_pixel_at(x, y);
                window.attrset(pixel.attributes());
                let (x_i32, y_i32) = (x as i32, y as i32);
                window.mvaddch(y_i32, x_i32, pixel.character());
            }
        }
    }
}
