//! Implementations of fundamental operations on polynomials in Bernstein basis.

use num::Num;
use std::ops::{Add, Mul};

use crate::Bernstein;

impl<T, U, const N: usize> Bernstein<T, U, N> where
    T: Copy + Add<T, Output = T> + Mul<U, Output = T>,
    U: Copy + Num,
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

}