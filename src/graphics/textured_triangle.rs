use std::sync::Arc;

use crate::geometry::{matrix::Matrix3D, triangle::Triangle2D, vector::Vector2D};

use super::{image_buffer::ImageBuffer, pixel_data::PixelData};

#[derive(Clone, Debug)]
pub struct TexturedTriangle2D {
    geometry: Triangle2D,
    uv_geometry: Triangle2D,
    texture: Arc<ImageBuffer>,
}

impl TexturedTriangle2D {
    pub fn new(geometry: Triangle2D, uv_geometry: Triangle2D, texture: Arc<ImageBuffer>) -> Self {
        Self {
            geometry,
            uv_geometry,
            texture,
        }
    }
    pub fn unipixeled(geometry: Triangle2D, pixel_data: PixelData) -> Self {
        Self::new(
            geometry,
            Triangle2D::default(),
            Arc::new(ImageBuffer::new(0, 0, pixel_data)),
        )
    }
    pub fn is_point_inside(&self, point: &Vector2D) -> bool {
        self.geometry.is_point_inside(point)
    }
    pub fn get_pixel_at(&self, point: &Vector2D) -> PixelData {
        let uv = self.geometry.to_uv_point(&self.uv_geometry, point);
        let u = (uv.x.max(0.)) as usize;
        let v = (uv.y.max(0.)) as usize;
        self.texture.get_pixel_at(u, v)
    }
    pub fn transform(&mut self, matrix: Matrix3D) {
        self.geometry = matrix * self.geometry;
    }
    
    pub fn geometry(&self) -> Triangle2D {
        self.geometry
    }
    
    pub fn uv_geometry(&self) -> Triangle2D {
        self.uv_geometry
    }
    
    pub fn texture(&self) -> Arc<ImageBuffer> {
        self.texture.clone()
    }
}
