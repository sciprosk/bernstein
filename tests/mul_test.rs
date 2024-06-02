#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use bernstein::Bernstein;

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