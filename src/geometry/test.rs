use super::*;

#[test]
fn point_inside_triangle() {
    let triangle = Triangle2D::new([
        Vector2D::new(0.0, 0.0),
        Vector2D::new(4.0, 0.0),
        Vector2D::new(2.0, 4.0),
    ]);

    // Test points inside the triangle
    assert!(triangle.is_point_inside(&Vector2D::new(2.0, 1.0)));
    assert!(triangle.is_point_inside(&Vector2D::new(1.0, 1.0)));
    assert!(triangle.is_point_inside(&Vector2D::new(2.0, 2.0)));
    assert!(triangle.is_point_inside(&Vector2D::new(3.0, 1.0)));
}

#[test]
fn point_outside_triangle() {
    let triangle = Triangle2D::new([
        Vector2D::new(0.0, 0.0),
        Vector2D::new(4.0, 0.0),
        Vector2D::new(2.0, 4.0),
    ]);

    // Test points outside the triangle
    assert!(!triangle.is_point_inside(&Vector2D::new(0.0, 5.0)));
    assert!(!triangle.is_point_inside(&Vector2D::new(6.0, 1.0)));
    assert!(!triangle.is_point_inside(&Vector2D::new(-1.0, 0.0)));
    assert!(!triangle.is_point_inside(&Vector2D::new(2.0, -1.0)));
}

#[test]
fn point_on_triangle_edges() {
    let triangle = Triangle2D::new([
        Vector2D::new(0.0, 0.0),
        Vector2D::new(4.0, 0.0),
        Vector2D::new(2.0, 4.0),
    ]);

    // Test points on triangle edges
    assert!(triangle.is_point_inside(&Vector2D::new(0.0, 0.0)));
    assert!(triangle.is_point_inside(&Vector2D::new(4.0, 0.0)));
    assert!(triangle.is_point_inside(&Vector2D::new(2.0, 4.0)));
    assert!(triangle.is_point_inside(&Vector2D::new(1.0, 2.0)));
    assert!(triangle.is_point_inside(&Vector2D::new(3.0, 2.0)));
}

#[test]
fn identity_matrix() {
    let identity = Matrix3D::identity();
    let point = Vector2D::new(2.0, 3.0);
    assert_eq!(identity * point, point);
}

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
    let identity_matrix = Matrix3D::identity();

    // Test associativity of matrix multiplication
    let result1 = (rotation_matrix * transposition_matrix) * identity_matrix;
    let result2 = rotation_matrix * (transposition_matrix * identity_matrix);
    assert_eq!(result1, result2);
}


#[test]
fn rotate_around() {
    let original_point = Vector2D::new(2.0, 1.0);
    let relative_point = Vector2D::new(1.0, 1.0);
    let rotated_point = original_point.rotate_around(relative_point, std::f64::consts::PI / 2.0);
    let expected_point = Vector2D::new(1.0, 2.0);
    assert_eq!(rotated_point, expected_point);
}
