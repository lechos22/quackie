use pancurses::{Attributes, Window};

use crate::geometry::vector::Vector2D;

use super::{pixel_data::PixelData, textured_triangle::TexturedTriangle2D};

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
    pub fn draw_triangles(&mut self, triangles: &[TexturedTriangle2D]) {
        for y in 0..self.height {
            let relative_y = y as f64 / self.min_wh as f64 - self.dy;
            for x in 0..self.width {
                let relative_x = (x as f64) * 0.5 / self.min_wh as f64 - self.dx;
                let point = Vector2D::new(relative_x, relative_y);
                for triangle in triangles.iter().rev() {
                    if triangle.is_point_inside(&point) {
                        self.set_pixel_at(x, y, triangle.get_pixel_at(&point));
                        break;
                    }
                }
            }
        }
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
                window.attrset(Attributes::new() | pixel.color_pair());
                let (x_i32, y_i32) = (x as i32, y as i32);
                window.mvaddch(y_i32, x_i32, pixel.character());
            }
        }
    }
}
