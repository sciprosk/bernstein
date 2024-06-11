//! Basic operations with generic polynomials in the Bernstein basis.
//!
//! This crate is experimental and depend on the unstable feature
//! `generic_const_exprs` that is only available in the nightly build.

#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(step_trait)]

use num::Num;

mod impls;
mod ops;

/// Contains coefficients of a polynomial of type `T` in the Bernstein basis
/// over the default interval (0, 1). The number of dimensions in the basis
/// is `N` which implies that the order of the basis polynomials in `N - 1`.
#[derive(Debug, Clone, Copy)]
pub struct Bernstein<T, U, const N: usize> {
    coef: [T; N],
    segm: (U, U),
}

impl<T, U, const N: usize> Bernstein<T, U, N> where
    U: Num,
{

    /// Create new instance of a Bernstein polynomial from an array of
    /// coefficients in the Bernstein basis over the default interval (0, 1).
    pub fn new(coef: [T; N]) -> Bernstein<T, U, N> {
        Bernstein {
            coef: coef,
            segm: (U::zero(), U::one()),
        }
    }

    /// Return an array of coefficients for a polynomial in the Bernstein basis.
    pub fn coef(&self) -> &[T; N] {
        &self.coef
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bernstein_private_container_basic() {
        let c = Bernstein{coef: [0, 1, 2], segm: (0, 1)};
        assert_eq!(c.coef, [0, 1, 2]);
        assert_eq!(c.segm, (0, 1));
    }

    #[test]
    fn bernstein_public_getset_api() {
        let coef_in = [1, 2, 3];
        let c: Bernstein<i32, i32, 3> = Bernstein::new(coef_in);
        let coef_out = c.coef().clone();
        assert_eq!(coef_in, coef_out);
    }
}
