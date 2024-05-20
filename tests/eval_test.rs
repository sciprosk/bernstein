use bernstein::Bernstein;

use crate::utils::equal_with_abs_tol;

mod utils;

#[test]
fn eval_zero_order() {
    const DBL_TOL: f64 = 1.0e-14;
    let c: Bernstein<f64, f64, 1> = Bernstein::new([1.5]);
    assert!(equal_with_abs_tol(c.eval(0.0), 1.5, DBL_TOL));
    assert!(equal_with_abs_tol(c.eval(1.0), 1.5, DBL_TOL));
    assert!(equal_with_abs_tol(c.eval(0.5), 1.5, DBL_TOL));
}