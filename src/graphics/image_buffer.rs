use std::fmt::Debug;

use super::pixel_data::PixelData;

pub struct ImageBuffer {
    width: usize,
    height: usize,
    pixels: Vec<PixelData>,
    background: PixelData,
}

impl ImageBuffer {
    pub fn new(width: usize, height: usize, background: PixelData) -> Self {
        let pixels = vec![background; width * height];
        Self {
            width,
            height,
            pixels,
            background,
        }
    }
    pub fn set_pixel_at(&mut self, x: usize, y: usize, pixel: PixelData) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = pixel
        }
    }
    pub fn get_pixel_at(&self, x: usize, y: usize) -> PixelData {
        if x >= self.width || y >= self.height {
            self.background
        } else {
            self.pixels[y * self.width + x]
        }
    }
}

impl Debug for ImageBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImageBuffer")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("background", &self.background)
            .finish()
    }
}
