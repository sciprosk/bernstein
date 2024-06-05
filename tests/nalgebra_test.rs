#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use bernstein::Bernstein;

use nalgebra::Vector3;

mod routines;
pub use routines::*;

#[test]
fn nalgebra_types() {
    const DBL_TOL: f64 = 1.0e-15;
    // Bezier control polygon in 3D.
    let p0: Vector3<f64> = Vector3::new(0.0, 0.0, 0.0);
    let p1: Vector3<f64> = Vector3::new(1.0, 1.0, 0.0);
    let p2: Vector3<f64> = Vector3::new(2.0, 0.0, 0.0);

    // Quadratic Bezier curve in 3D.
    let c: Bernstein<_, f64, 3> = Bernstein::new([p0, p1, p2]);

    let start = c.eval(0.0);
    let end = c.eval(1.0);
    let middle = c.eval(0.5);

    assert!(vector3_equal_with_abs_tol(start, p0, DBL_TOL));
    assert!(vector3_equal_with_abs_tol(end, p2, DBL_TOL));
    assert!(vector3_equal_with_abs_tol(middle, 0.25 * (p0 + 2.0 * p1 + p2), DBL_TOL));
}

#[test]
fn nalgebra_vector_right_mul_by_scalar(){
    const DBL_TOL: f64 = 1.0e-15;
    let p0: Vector3<f64> = Vector3::new(1.0, 1.0, 6.0);
    let p1: Vector3<f64> = Vector3::new(2.0, 3.0, 4.0);

    let c: Bernstein<_, f64, 2> = Bernstein::new([p0, p1]);
    let p = c * 5.0;
    let start = p.eval(0.0);
    let end = p.eval(1.0);
    assert!(vector3_equal_with_abs_tol(start, 5.0 * p0, DBL_TOL));
    assert!(vector3_equal_with_abs_tol(end, 5.0 * p1, DBL_TOL));
}


