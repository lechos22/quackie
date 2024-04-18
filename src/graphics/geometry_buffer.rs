use crate::geometry::matrix::Matrix3D;

use super::textured_triangle::TexturedTriangle2D;

pub struct GeometryBuffer {
    triangles: Vec<TexturedTriangle2D>,
}

impl GeometryBuffer {
    pub fn new(triangles: Vec<TexturedTriangle2D>) -> Self {
        Self { triangles }
    }

    pub fn triangles(&self) -> &[TexturedTriangle2D] {
        &self.triangles
    }

    pub fn transform(&mut self, matrix: Matrix3D) {
        for triangle in self.triangles.iter_mut() {
            triangle.transform(matrix);
        }
    }
}
