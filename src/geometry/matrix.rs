use std::ops::Mul;

use super::Vector2D;

type Matrix3DRow = [f64; 3];

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix3D {
    pub data: [Matrix3DRow; 3],
}

impl Matrix3D {
    pub const fn new(data: [Matrix3DRow; 3]) -> Self {
        Self { data }
    }

    pub const fn identity() -> Self {
        Self::new([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
    }

    pub fn rotation(angle: f64) -> Self {
        let cosine = angle.cos();
        let sine = angle.sin();
        Self::new([[cosine, -sine, 0.0], [sine, cosine, 0.0], [0.0, 0.0, 1.0]])
    }

    pub fn transposition(x: f64, y: f64) -> Self {
        Self::new([[1.0, 0.0, x], [0.0, 1.0, y], [0.0, 0.0, 1.0]])
    }
}

impl Mul for Matrix3D {
    type Output = Matrix3D;

    fn mul(self, other: Matrix3D) -> Matrix3D {
        let mut result_data = [[0.0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                result_data[i][j] = self.data[i][0] * other.data[0][j]
                    + self.data[i][1] * other.data[1][j]
                    + self.data[i][2] * other.data[2][j];
            }
        }
        Matrix3D::new(result_data)
    }
}

impl Mul<Vector2D> for Matrix3D {
    type Output = Vector2D;

    fn mul(self, point: Vector2D) -> Self::Output {
        let x = point.x * self.data[0][0] + point.y * self.data[0][1] + self.data[0][2];
        let y = point.x * self.data[1][0] + point.y * self.data[1][1] + self.data[1][2];
        Vector2D::new(x, y)
    }
}
