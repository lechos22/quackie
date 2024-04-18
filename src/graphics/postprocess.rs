pub mod antialias;
pub mod outline;

use super::{pixel_data::PixelData, window_buffer::WindowBuffer};

pub trait PostProcessShader: Sync {
    fn execute(&self, window_buffer: &WindowBuffer, x: usize, y: usize) -> PixelData;
}
