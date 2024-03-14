use super::*;

#[test]
fn test_point_inside_triangle() {
    let triangle = Triangle2D::new(
        Point2D::new(0.0, 0.0),
        Point2D::new(4.0, 0.0),
        Point2D::new(2.0, 4.0),
    );

    // Test points inside the triangle
    assert!(triangle.is_point_inside(&Point2D::new(2.0, 1.0)));
    assert!(triangle.is_point_inside(&Point2D::new(1.0, 1.0)));
    assert!(triangle.is_point_inside(&Point2D::new(2.0, 2.0)));
    assert!(triangle.is_point_inside(&Point2D::new(3.0, 1.0)));
}

#[test]
fn test_point_outside_triangle() {
    let triangle = Triangle2D::new(
        Point2D::new(0.0, 0.0),
        Point2D::new(4.0, 0.0),
        Point2D::new(2.0, 4.0),
    );

    // Test points outside the triangle
    assert!(!triangle.is_point_inside(&Point2D::new(0.0, 5.0)));
    assert!(!triangle.is_point_inside(&Point2D::new(6.0, 1.0)));
    assert!(!triangle.is_point_inside(&Point2D::new(-1.0, 0.0)));
    assert!(!triangle.is_point_inside(&Point2D::new(2.0, -1.0)));
}

#[test]
fn test_point_on_triangle_edges() {
    let triangle = Triangle2D::new(
        Point2D::new(0.0, 0.0),
        Point2D::new(4.0, 0.0),
        Point2D::new(2.0, 4.0),
    );

    // Test points on triangle edges
    assert!(triangle.is_point_inside(&Point2D::new(0.0, 0.0)));
    assert!(triangle.is_point_inside(&Point2D::new(4.0, 0.0)));
    assert!(triangle.is_point_inside(&Point2D::new(2.0, 4.0)));
    assert!(triangle.is_point_inside(&Point2D::new(1.0, 2.0)));
    assert!(triangle.is_point_inside(&Point2D::new(3.0, 2.0)));
}
