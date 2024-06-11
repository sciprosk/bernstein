//! Implementations of binary operations on polynomials in the Bernstein basis.

use num::{FromPrimitive, Num};
use std::iter::Step;
use std::ops::{Add, Sub, Mul};

use crate::Bernstein;

// Calculate binomial coefficient (n, k) "n choose k".
// Should only be called on types that can be safely compared with `==`.
fn binom<T>(n: T, k: T) -> T where
    T: Copy + Num + Step,
{
    assert!(k >= T::zero());
    assert!(n >= T::zero());

    if k > n {
        return T::zero();
    }
    if k == T::zero() {
        return T::one();
    }

    let mut next = n - k + T::one();

    for i in T::one()..k {
        next = next * (n - k + T::one() + i) / (i + T::one());
    }

    next
}

// Lower summation bound that does not overflow for unsized types.
fn low_bound<T>(a: T, b: T) -> T where T: Num + PartialOrd {
    if a >= b {
        return a - b
    }
    T::zero()
}

/// Calculate a product of two polynomials in the Bernstein basis.
///
/// See R. T. Farouki, "Pythagorean-Hodograph Curves: Algebra and Geometry
/// Inseparable", Geometry and Computing (Eds. H. Edelsbrunner, K. Polthier,
/// and L Kobbelt) Springer (2008). -- p. 258, Sec. 11.7, Eq. (11.20).
impl<T, U, const N: usize, const M: usize>
Mul<Bernstein<T, U, {M}>> for Bernstein<T, U, {N}> where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Mul<U, Output = T>,
    U: Num + FromPrimitive,
    [(); N]:,
    [(); M]:,
    [(); N + M - 1]:
{
    type Output = Bernstein<T, U, {N + M - 1}>;

    fn mul(self, rhs: Bernstein<T, U, {M}>) -> Self::Output {
        let mut coef = [self.coef[0] - self.coef[0]; N + M - 1];

        let n = M - 1;
        let m = N - 1;

        for k in 0..=m + n {
            for j in low_bound(k, n)..=std::cmp::min(m, k) {
                coef[k] = coef[k] + self.coef[j] * rhs.coef[k - j]
                        * (U::from_usize(binom(m, j)).unwrap()
                        * U::from_usize(binom(n, k - j)).unwrap()
                        / U::from_usize(binom(m + n, k)).unwrap());
            }
        }

        Bernstein {
            coef: coef,
            segm: self.segm,
        }
    }
}

/// Right hand side coefficient-wise multiplication by a scalar.
impl<T, U, W, const N: usize> Mul<W> for Bernstein<T, U, N> where
    T: Copy + Mul<W, Output = T>,
    W: Copy + Num,
    [(); N]:
{
    type Output = Bernstein<T, U, N>;
    fn mul(self, rhs: W) -> Self::Output {
        let mut coef = self.coef;
        for i in 0..N {
            coef[i] = self.coef[i] * rhs;
        }

        Bernstein {
            coef: coef,
            segm: self.segm,
        }
    }
}

macro_rules! left_scalar_mul_impl(
    ($($U: ty),* $(,)*) => {$(
        impl<T, const N: usize> Mul<Bernstein<T, $U, {N}>> for $U where
            T: Copy + Mul<$U, Output = T>,
            [(); N]:
        {
            type Output = Bernstein<T, $U, {N}>;

            fn mul(self, rhs: Bernstein<T, $U, {N}>) -> Self::Output {
                let mut coef = rhs.coef;
                for i in 0..N {
                    coef[i] = rhs.coef[i] * self;
                }

                Bernstein {
                    coef: coef,
                    segm: rhs.segm,
                }
            }
        }
    )*}
);

left_scalar_mul_impl!(f32, f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binom_n_zero() {
        assert_eq!(binom(0, 0), 1);
        assert_eq!(binom(0, 1), 0);
    }

    #[test]
    fn binom_n_k() {
        assert_eq!(binom(7, 0), 1);
        assert_eq!(binom(7, 1), 7);
        assert_eq!(binom(7, 2), 21);
        assert_eq!(binom(7, 3), 35);
        assert_eq!(binom(7, 4), 35);
        assert_eq!(binom(7, 5), 21);
        assert_eq!(binom(7, 6), 7);
        assert_eq!(binom(7, 7), 1);
        assert_eq!(binom(7, 8), 0);

        assert_eq!(binom(6, 0), 1);
        assert_eq!(binom(6, 1), 6);
        assert_eq!(binom(6, 2), 15);
        assert_eq!(binom(6, 3), 20);
        assert_eq!(binom(6, 4), 15);
        assert_eq!(binom(6, 5), 6);
        assert_eq!(binom(6, 6), 1);
        assert_eq!(binom(6, 7), 0);
    }

    #[test]
    fn low_bound_usize() {
        let u: usize = 3;
        let v: usize = 7;
        assert_eq!(low_bound(u, v), 0);
        assert_eq!(low_bound(v, u), 4);
    }
}