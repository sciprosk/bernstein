use bernstein::Bernstein;

use crate::utils::equal_with_abs_tol;

mod utils;

#[test]
fn eval_zero_order_f64() {
    const DBL_TOL: f64 = 1.0e-15;
    let c: Bernstein<f64, f64, 1> = Bernstein::new([1.5]);
    assert!(equal_with_abs_tol(c.eval(0.0), 1.5, DBL_TOL));
    assert!(equal_with_abs_tol(c.eval(1.0), 1.5, DBL_TOL));
    assert!(equal_with_abs_tol(c.eval(0.5), 1.5, DBL_TOL));
}

#[test]
fn eval_first_order_f64() {
    const DBL_TOL: f64 = 1.0e-15;
    let c: Bernstein<f64, f64, 2> = Bernstein::new([1.0, 2.0]);
    assert!(equal_with_abs_tol(c.eval(0.0), 1.0, DBL_TOL));
    assert!(equal_with_abs_tol(c.eval(1.0), 2.0, DBL_TOL));
    assert!(equal_with_abs_tol(c.eval(0.5), 1.5, DBL_TOL));
}

#[test]
fn eval_second_order_f64() {
    const DBL_TOL: f64 = 1.0e-15;
    let c: Bernstein<f64, f64, 3> = Bernstein::new([1.0, 2.0, -1.0]);
    assert!(equal_with_abs_tol(c.eval(0.0), 1.0, DBL_TOL));
    assert!(equal_with_abs_tol(c.eval(1.0), -1.0, DBL_TOL));
    assert!(equal_with_abs_tol(c.eval(0.5), 1.0, DBL_TOL));
}
