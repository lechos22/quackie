use std::ops::Mul;

use super::{triangle::Triangle2D, vector::Vector2D};

type Matrix3DRow = [f64; 3];

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix3D {
    pub data: [Matrix3DRow; 3],
}

impl Matrix3D {
    pub const fn new(data: [Matrix3DRow; 3]) -> Self {
        Self { data }
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

    #[allow(clippy::needless_range_loop)]
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

impl Mul<Triangle2D> for Matrix3D {
    type Output = Triangle2D;

    fn mul(self, mut triangle: Triangle2D) -> Self::Output {
        triangle.points[0] = self * triangle.points[0];
        triangle.points[1] = self * triangle.points[1];
        triangle.points[2] = self * triangle.points[2];
        triangle
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::vector::Vector2D;

    use super::Matrix3D;

    pub const IDENTITY_MATRIX: Matrix3D =
        Matrix3D::new([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);

    #[test]
    fn rotation_matrix() {
        // Test rotation by 90 degrees (π/2 radians)
        let rotation_matrix = Matrix3D::rotation(std::f64::consts::PI / 2.0);
        let point = Vector2D::new(1.0, 0.0);
        let rotated_point = rotation_matrix * point;
        assert!((rotated_point.x - 0.0).abs() < f64::EPSILON);
        assert!((rotated_point.y - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn transposition_matrix() {
        let transposition_matrix = Matrix3D::transposition(2.0, 3.0);
        let point = Vector2D::new(1.0, 1.0);
        let transposed_point = transposition_matrix * point;
        assert_eq!(transposed_point.x, 3.0);
        assert_eq!(transposed_point.y, 4.0);
    }

    #[test]
    fn matrix_multiplication() {
        let rotation_matrix = Matrix3D::rotation(std::f64::consts::PI / 2.0);
        let transposition_matrix = Matrix3D::transposition(2.0, 3.0);

        // Test associativity of matrix multiplication
        let result1 = (rotation_matrix * transposition_matrix) * IDENTITY_MATRIX;
        let result2 = rotation_matrix * (transposition_matrix * IDENTITY_MATRIX);
        assert_eq!(result1, result2);
    }
}
