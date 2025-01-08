use hamcrest2::prelude::*;

use rust_points::prelude::*;

// Tolerance within which two numbers are considered equal
const TOL_F64: f64 = 1e-8;
const TOL_F32: f32 = 1e-8;

#[test]
fn test_origin_f64() {
    let origin = Point2D::new(0_f64, 0_f64);

    assert_that!(origin.get_x(), close_to(0_f64, TOL_F64));
    assert_that!(origin.get_y(), close_to(0_f64, TOL_F64));

    let dims = origin.get_dims();
    assert_that!(dims.0, close_to(0_f64, TOL_F64));
    assert_that!(dims.1, close_to(0_f64, TOL_F64));
}

#[test]
fn test_origin_f32() {
    let origin = Point2D::new(0_f32, 0_f32);

    assert_that!(origin.get_x(), close_to(0_f32, TOL_F32));
    assert_that!(origin.get_y(), close_to(0_f32, TOL_F32));

    let dims = origin.get_dims();
    assert_that!(dims.0, close_to(0_f32, TOL_F32));
    assert_that!(dims.1, close_to(0_f32, TOL_F32));
}

#[test]
fn test_from_tuple_f32() {
    let point: Point2D<f32> = (1.5_f32, 17.2_f32).into();

    let expected_dims = (1.5_f32, 17.2_f32);
    assert_that!(point.get_x(), close_to(expected_dims.0, TOL_F32));
    assert_that!(point.get_y(), close_to(expected_dims.1, TOL_F32));

    let dims = point.get_dims();
    assert_that!(dims.0, close_to(expected_dims.0, TOL_F32));
    assert_that!(dims.1, close_to(expected_dims.1, TOL_F32));
}

#[test]
fn test_from_tuple_f64() {
    let point = Point2D::from((4.528_f64, 17.2559_f64));

    let expected_dims = (4.528_f64, 17.2559_f64);
    assert_that!(point.get_x(), close_to(expected_dims.0, TOL_F64));
    assert_that!(point.get_y(), close_to(expected_dims.1, TOL_F64));

    let dims = point.get_dims();
    assert_that!(dims.0, close_to(expected_dims.0, TOL_F64));
    assert_that!(dims.1, close_to(expected_dims.1, TOL_F64));
}

#[test]
fn test_add() {
    let pt1 = Point2D::from((1.3_f64, 2.9_f64));
    let pt2 = Point2D::from((1.0_f64, 2.5_f64));

    let expected_dims = (2.3_f64, 5.4_f64);

    let dims = (pt1 + pt2).get_dims();
    assert_that!(dims.0, close_to(expected_dims.0, TOL_F64));
    assert_that!(dims.1, close_to(expected_dims.1, TOL_F64));
}

#[test]
fn test_sub() {
    let pt1 = Point2D::from((1.3_f64, 2.9_f64));
    let pt2 = Point2D::from((1.0_f64, 2.5_f64));

    let expected_dims = (0.3_f64, 0.4_f64);

    let dims = (pt1 - pt2).get_dims();
    assert_that!(dims.0, close_to(expected_dims.0, TOL_F64));
    assert_that!(dims.1, close_to(expected_dims.1, TOL_F64));
}

#[test]
fn test_scale() {
    let pt = Point2D::from((1.0_f64, 2.5_f64)).scale(2_f64);

    let expected_dims = (2.0_f64, 5.0_f64);

    let dims = pt.get_dims();
    assert_that!(dims.0, close_to(expected_dims.0, TOL_F64));
    assert_that!(dims.1, close_to(expected_dims.1, TOL_F64));
}

#[test]
fn test_dot_product() {
    let pt1 = Point2D::from((1.3_f64, 2.9_f64));
    let pt2 = Point2D::from((1.0_f64, 2.0_f64));

    let expected = 7.1_f64;

    assert_that!(pt1.dot(&pt2), close_to(expected, TOL_F64));
}
