//! Implementations of binary operations on polynomials in the Bernstein basis.

use num::Num;
use std::iter::Step;
use std::ops::Mul;

use crate::Bernstein;

// Calculate binomial coefficient (n, k) "n choose k".
// Should only be called on types that can be safely compared with `==`.
fn binom<T>(n: T, k: T) -> T where
    T: Copy + Num + PartialOrd<T> + Step
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

impl<T, U, const N: usize, const M: usize>
Mul<Bernstein<T, U, {M}>> for Bernstein<T, U, N> where
    [(); N]:,
    [(); M]:,
    [(); N + M]:
{
    type Output = Bernstein<T, U, {N + M}>;

    fn mul(self, rhs: Bernstein<T, U, {M}>) -> Self::Output {
        todo!();
    }
}

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

}