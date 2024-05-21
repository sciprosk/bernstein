//! Implementations of fundamental operations on polynomials in Bernstein basis.

use num::FromPrimitive;
use num::Num;
use std::array;
use std::ops::{Add, Sub, Mul};

use crate::Bernstein;

impl<T, U, const N: usize> Bernstein<T, U, N> where
    T: Copy + Add<T, Output = T> + Sub<T, Output = T> + Mul<U, Output = T>,
    U: Copy + Num + FromPrimitive,
{

    /// Evaluate polynomial in the Bernstein basis at the point `0 <= u <= 1`
    /// using the De Casteljau's algorithm.
    ///
    /// See Piegl & Tiller. "The NURBS book". Springer Science & Business Media
    /// (2012) -- p.24, A1.5.
    pub fn eval(&self, u: U) -> T {
        let mut q = self.coef;

        for k in 1..=N - 1 {
            for i in 0..=N - k - 1 {
                q[i] = q[i]     * ((self.segm.1 - u) / (self.segm.1 - self.segm.0))
                     + q[i + 1] * ((u - self.segm.0) / (self.segm.1 - self.segm.0));
            }
        }
        q[0]
    }

    /// Return new instance that is a derivative of the original polynomial in
    /// the Bernstein basis.
    ///
    /// See Piegl & Tiller. "The NURBS book". Springer Science & Business Media
    /// (2012) -- p.22, Eq. (1.9).
    pub fn diff(&self) -> Bernstein<T, U, {N - 1}> where
        [(); N - 1]:
    {
        let coef: [T; N - 1] = array::from_fn(
            |i| -> T {
                (self.coef[i + 1] - self.coef[i]) *
                (U::from_usize(N - 1).unwrap() / (self.segm.1 - self.segm.0))
            }
        );

        Bernstein {
            segm: self.segm,
            coef: coef,
        }
    }

}