#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use bernstein::Bernstein;

use crate::utils::equal_with_abs_tol;

mod utils;

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
    }