pub fn equal_with_abs_tol(x: f64, y: f64, tol: f64) -> bool {
    (x - y).abs() < tol
}