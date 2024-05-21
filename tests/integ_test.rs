use num::{Complex, Rational64, Zero};

use bernstein::Bernstein;

use crate::utils::equal_with_abs_tol;

mod utils;

#[test]
fn integ_first_order_complex() {
    const DBL_TOL: f64 = 1.0e-15;
    let c: Bernstein<Complex<f64>, f64, 2> = Bernstein::new(
        [Complex::new(0.0, 1.0), Complex::new(1.0, 0.0)]
    );
    let h = c.integ(Complex::zero());
    let coef = h.coef().clone();

    assert!(equal_with_abs_tol(coef[0].re, 0.0, DBL_TOL));
    assert!(equal_with_abs_tol(coef[1].re, 0.0, DBL_TOL));
    assert!(equal_with_abs_tol(coef[2].re, 0.5, DBL_TOL));

    assert!(equal_with_abs_tol(coef[0].im, 0.0, DBL_TOL));
    assert!(equal_with_abs_tol(coef[1].im, 0.5, DBL_TOL));
    assert!(equal_with_abs_tol(coef[2].im, 0.5, DBL_TOL));
}

// Make sure that `integ()` and `diff()` are mutually inverse.
#[test]
fn integ_second_order_rational() {
    let p0 = Rational64::new(1, 13);
    let p1 = Rational64::new(-3, 11);
    let p2 = Rational64::new(1, 7);
    let coef = [p0, p1, p2];

    let c: Bernstein<Rational64, Rational64, 3> = Bernstein::new(coef);

    assert_eq!(c.integ(Rational64::zero()).diff().coef().clone(), coef);
    assert_eq!(c.diff().integ(Rational64::new(1, 13)).coef().clone(), coef);
}