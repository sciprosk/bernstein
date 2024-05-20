use num::{One, Rational64, Zero};

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

#[test]
fn eval_second_order_rational() {
    let p1 = Rational64::new(1, 1);
    let p2 = Rational64::new(2, 1);
    let p3 = Rational64::new(-1, 1);

    let c: Bernstein<Rational64, Rational64, 3> = Bernstein::new([p1, p2, p3]);

    let u = Rational64::zero();
    assert_eq!(c.eval(u), Rational64::one());

    let u = Rational64::one();
    assert_eq!(c.eval(u), Rational64::new(-1, 1));

    let u = Rational64::new(1, 2);
    assert_eq!(c.eval(u), Rational64::one());

    let u = Rational64::new(1, 17);
    assert_eq!(c.eval(u), Rational64::new(319, 289));
}

#[test]
fn eval_third_order_rational() {
    let p0 = Rational64::new(1, 5);
    let p1 = Rational64::new(-3, 7);
    let p2 = Rational64::new(4, 13);
    let p3 = Rational64::new(-11, 17);

    let c: Bernstein<Rational64, Rational64, 4> = Bernstein::new([p0, p1, p2, p3]);

    let u = Rational64::new(0, 1);
    assert_eq!(c.eval(u), p0);

    let u = Rational64::new(1, 1);
    assert_eq!(c.eval(u), p3);

    let u = Rational64::new(1, 2);
    assert_eq!(c.eval(u), Rational64::new(-6263, 61880));

    let u = Rational64::new(1, 17);
    assert_eq!(c.eval(u), Rational64::new(3899827, 38002055));
}
