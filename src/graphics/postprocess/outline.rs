use crate::graphics::pixel_data::PixelData;

use super::PostProcessShader;

pub struct Outline;
impl PostProcessShader for Outline {
    fn execute(
        &self,
        window_buffer: &crate::graphics::window_buffer::WindowBuffer,
        x: usize,
        y: usize,
    ) -> PixelData {
        let old = window_buffer.get_pixel_at(x, y);
        if x == 0 || y == 0 {
            return old;
        }
        let neighbours = [
            window_buffer.get_pixel_at(x - 1, y),
            window_buffer.get_pixel_at(x + 1, y),
            window_buffer.get_pixel_at(x, y - 1),
            window_buffer.get_pixel_at(x, y + 1),
        ];
        if old.character().is_ascii_graphic() {
            let neighbours = neighbours
                .iter()
                .filter(|pixel| !pixel.character().is_whitespace())
                .count();
            match neighbours {
                0 => PixelData::new('+', old.attributes()),
                1 => PixelData::new('+', old.attributes()),
                2 => PixelData::new('+', old.attributes()),
                3 => PixelData::new('+', old.attributes()),
                4 => PixelData::new(' ', old.attributes()),
                _ => unreachable!(),
            }
        } else {
            old
        }
    }
}
