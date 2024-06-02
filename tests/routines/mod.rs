use nalgebra::Vector3;

pub fn equal_with_abs_tol(x: f64, y: f64, tol: f64) -> bool {
    (x - y).abs() < tol
}

pub fn vector3_equal_with_abs_tol(v: Vector3<f64>, w: Vector3<f64>, tol: f64) -> bool {
    (v - w).norm() < tol
}