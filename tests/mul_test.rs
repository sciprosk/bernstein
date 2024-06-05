#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use bernstein::Bernstein;

use num::complex::Complex64;
use num::rational::Rational64;

mod routines;
pub use routines::*;

#[test]
fn product_fist_order() {
    const DBL_TOL: f64 = 1.0e-15;
    let p: Bernstein<f64, f64, 2> = Bernstein::new([0.0, 1.0]);
    let q: Bernstein<f64, f64, 2> = Bernstein::new([2.0, 0.0]);

    let c = p * q;

    let coef = c.coef().clone();
    assert!(equal_with_abs_tol(coef[0], 0.0, DBL_TOL));
    assert!(equal_with_abs_tol(coef[1], 1.0, DBL_TOL));
    assert!(equal_with_abs_tol(coef[2], 0.0, DBL_TOL));

    let p: Bernstein<f64, f64, 2> = Bernstein::new([0.0, 1.0]);
    let q: Bernstein<f64, f64, 2> = Bernstein::new([2.0, 0.0]);

    let c = q * p;

    let coef = c.coef().clone();
    assert!(equal_with_abs_tol(coef[0], 0.0, DBL_TOL));
    assert!(equal_with_abs_tol(coef[1], 1.0, DBL_TOL));
    assert!(equal_with_abs_tol(coef[2], 0.0, DBL_TOL));
}

#[test]
fn product_first_order_by_second_order() {
    const DBL_TOL: f64 = 2.0e-15;
    let p: Bernstein<f64, f64, 2> = Bernstein::new([5.0, 3.0]);
    let q: Bernstein<f64, f64, 3> = Bernstein::new([1.0, 2.0, 5.0]);

    let c = p * q;

    let coef = c.coef().clone();
    assert!(equal_with_abs_tol(coef[0], 5.0, DBL_TOL));
    assert!(equal_with_abs_tol(coef[1], 23.0 / 3.0, DBL_TOL));
    assert!(equal_with_abs_tol(coef[2], 37.0 / 3.0, DBL_TOL));
    assert!(equal_with_abs_tol(coef[3], 15.0, DBL_TOL));

    let p: Bernstein<f64, f64, 2> = Bernstein::new([5.0, 3.0]);
    let q: Bernstein<f64, f64, 3> = Bernstein::new([1.0, 2.0, 5.0]);

    let c = q * p;

    let coef = c.coef().clone();
    assert!(equal_with_abs_tol(coef[0], 5.0, DBL_TOL));
    assert!(equal_with_abs_tol(coef[1], 23.0 / 3.0, DBL_TOL));
    assert!(equal_with_abs_tol(coef[2], 37.0 / 3.0, DBL_TOL));
    assert!(equal_with_abs_tol(coef[3], 15.0, DBL_TOL));
}

#[test]
fn product_second_order_by_third_order() {
    const DBL_TOL: f64 = 1.0e-15;
    let p: Bernstein<f64, f64, 3> = Bernstein::new([0.0, 2.0, 1.0]);
    let q: Bernstein<f64, f64, 4> = Bernstein::new([1.0, 2.0, 0.0, 0.0]);

    let c = p * q;

    let coef = c.coef().clone();
    assert!(equal_with_abs_tol(coef[0], 0.0, DBL_TOL));
    assert!(equal_with_abs_tol(coef[1], 0.8, DBL_TOL));
    assert!(equal_with_abs_tol(coef[2], 2.5, DBL_TOL));
    assert!(equal_with_abs_tol(coef[3], 0.6, DBL_TOL));
    assert!(equal_with_abs_tol(coef[4], 0.0, DBL_TOL));
    assert!(equal_with_abs_tol(coef[5], 0.0, DBL_TOL));

    let p: Bernstein<f64, f64, 3> = Bernstein::new([0.0, 2.0, 1.0]);
    let q: Bernstein<f64, f64, 4> = Bernstein::new([1.0, 2.0, 0.0, 0.0]);

    let c = q * p;

    let coef = c.coef().clone();
    assert!(equal_with_abs_tol(coef[0], 0.0, DBL_TOL));
    assert!(equal_with_abs_tol(coef[1], 0.8, DBL_TOL));
    assert!(equal_with_abs_tol(coef[2], 2.5, DBL_TOL));
    assert!(equal_with_abs_tol(coef[3], 0.6, DBL_TOL));
    assert!(equal_with_abs_tol(coef[4], 0.0, DBL_TOL));
    assert!(equal_with_abs_tol(coef[5], 0.0, DBL_TOL));
}

#[test]
fn mul_right_by_scalar() {
    const DBL_TOL: f64 = 1.0e-15;

    let c: Bernstein<_, f64, 3> = Bernstein::new([1.0, 2.0, 3.0]);
    let p = c * 3.0;
    let coef = p.coef().clone();
    assert!(equal_with_abs_tol(coef[0], 3.0, DBL_TOL));
    assert!(equal_with_abs_tol(coef[1], 6.0, DBL_TOL));
    assert!(equal_with_abs_tol(coef[2], 9.0, DBL_TOL));

    let p0 = Complex64::new(1.0, 2.0);
    let p1 = Complex64::new(3.0, 4.0);

    let c: Bernstein<_, f64, 2> = Bernstein::new([p0, p1]);
    let p = c * 2.0;
    let coef = p.coef().clone();
    assert!(equal_with_abs_tol(coef[0].re, 2.0, DBL_TOL));
    assert!(equal_with_abs_tol(coef[0].im, 4.0, DBL_TOL));
    assert!(equal_with_abs_tol(coef[1].re, 6.0, DBL_TOL));
    assert!(equal_with_abs_tol(coef[1].im, 8.0, DBL_TOL));

    let p0 = Rational64::new(1, 2);
    let p1 = Rational64::new(3, 4);
    let c: Bernstein<_, f64, 2> = Bernstein::new([p0, p1]);
    let p = c * 2;
    let coef = p.coef().clone();
    assert_eq!(coef[0], Rational64::new(1, 1));
    assert_eq!(coef[1], Rational64::new(3, 2));
}