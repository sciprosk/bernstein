use num::Complex;

use bernstein::Bernstein;

mod routines;
pub use routines::*;

#[test]
fn diff_zero_order() {
    let c: Bernstein<f64, f64, 1> = Bernstein::new([1.0]);
    assert_eq!(c.diff().coef().clone(), []);
}

#[test]
fn diff_first_order_complex() {
    const DBL_TOL: f64 = 1.0e-15;
    let p0 = Complex::new(0.0, 0.0);
    let p1 = Complex::new(2.0, 1.0);

    let c: Bernstein<Complex<f64>, f64, 2> = Bernstein::new([p0, p1]);
    let d = c.diff().coef().clone();
    assert!(equal_with_abs_tol(d[0].re, 2.0, DBL_TOL));
    assert!(equal_with_abs_tol(d[0].im, 1.0, DBL_TOL));
}

#[test]
fn diff_second_order_f64() {
    const DBL_TOL: f64 = 1.0e-15;
    let c:Bernstein<f64, f64, 3> = Bernstein::new([1.0, -1.0, 2.0]);
    let coef = c.diff().coef().clone();
    assert!(equal_with_abs_tol(coef[0], -4.0, DBL_TOL));
    assert!(equal_with_abs_tol(coef[1], 6.0, DBL_TOL));
}

#[test]
fn diff_third_order_f64() {
    const DBL_TOL: f64 = 1.0e-15;
    let c:Bernstein<f64, f64, 4> = Bernstein::new([1.0, 0.0, 1.0, -1.0]);
    let coef = c.diff().coef().clone();
    assert!(equal_with_abs_tol(coef[0], -3.0, DBL_TOL));
    assert!(equal_with_abs_tol(coef[1], 3.0, DBL_TOL));
    assert!(equal_with_abs_tol(coef[2], -6.0, DBL_TOL));
}