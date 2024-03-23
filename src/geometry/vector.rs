use super::Matrix3D;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

impl Vector2D {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    pub fn rotate_around(self, relative_to: Self, angle: f64) -> Self {
        let transpose = Matrix3D::transposition(-relative_to.x, -relative_to.y);
        let rotate = Matrix3D::rotation(angle);
        let transpose_back = Matrix3D::transposition(relative_to.x, relative_to.y);
        transpose_back * rotate * transpose * self
    }
}
