# bernstein: generic polynomials in Bernstein basis

**Attention:** This library is experimental and is based on the unstable
`generic_const_exprs` feature that is only available in Nightly Rust.

This library implements basic operation with polynomials in the Bernstein basis
in Rust, which can be used to create and manipulate generic Bézier curves. It is
built on the following basic principles.

* Being generic and work with various data types for both coefficient of the
polynomials (Bézier control polygon), and curve parameterization.

* Rely only on static dispatch and stack memory allocations with the polynomial
degree (the size of the Bézier control polygon) being a compile time constant
expression.

## Examples

### Quintic Pythagorean-hodograph cornering curve
The following example shows how to construct a 5th degree Pythagorean-hodograph curve
complex representation using multiplication and integration in the Bernstein basis.
For the entire source code of this example, see `./examples/pythagorean_hodograph.rs`.

First, let put the following dependencies in the `Cargo.toml`.
```toml
[dependencies]
bernstein = {git = "https://github.com/sciprosk/bernstein.git"}
num = "0.4.3"
```
Specify that we want the unstable `generic_const_exprs` Rust feature, and add necessary 
imports. Our planar Pythagorean-hodograph curve has its control polygon in the complex plane,
and is parameterized with `f32`.
```rust
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use bernstein::Bernstein;
use num::{complex::Complex32 as Complex, Zero};
```
Define curve parameters such as size, shape, orientation, and the initial point, and write down 
the control polygon of an internal cubic hodograph curve in the complex plane. For the details
on the math formalism see Refs. [2, 3].
```rust
// Specify parameters of the curve
let d = 1.0;                             // cornering length -- distance
                                         // from entry/exit to the corner
let theta = 0.0;                         // orientation of the corner
let delta = std::f32::consts::FRAC_PI_2; // angle of the corner

// Actual formula that sets the internal representation
// of the quintic Pythgorean-hodograh curve.
let rho = f32::sqrt(30.0 * d * f32::cos(0.5 * delta) / (6.0 * f32::cos(0.5 * delta) + 1.0));

// Initial point of the curve in the complex plane.
let p0 = Complex::new(-1.0, 0.0);

// Control poligon for the internal representation of the curve
// in the complex plane.
let w0 = rho * Complex::exp(0.5 * theta * Complex::i());
let w1 = Complex::zero();
let w2 = rho * Complex::exp(0.5 * (theta + delta) * Complex::i());
```
The following creates a cubic curve in the complex plane (internal representation), that is
used to generate the fifth order Pythagorean-hodograph curve by taking a square of the
cubic curve (this sets the hodograph of the quintic curve), and then integrating from
the starting point in the complex plane. The resulting curve has six points in its control
polygon.
```rust
// Construct Bernstein polynomial in the complex plane from control polygon.
let w: Bernstein<Complex, f32, 3> = Bernstein::new([w0, w1, w2]);
// Take the square and integrate from the initial point.
let p = (w * w).integ(p0);
```
Now, we can use `p.eval(u)` to sample this curve at some values of the parameter `u`. For example, sampling
using [Plotters](https://docs.rs/plotters/latest/plotters/) gives the following symmetric cornering curves for
90-degree and 60-degree corners. Note that for any _symmetric_ _quintic_ Pythagorean-hodograph curve, the control poygon has identical control 
points (P0, P1 = P2, P3 = P4, P5).
![pythgorean_hodograph1](https://github.com/sciprosk/bernstein/assets/105472369/1a700752-dfd3-4e29-b316-44bfbf8ebf00)
![pythgorean_hodograph](https://github.com/sciprosk/bernstein/assets/105472369/8f2c5202-6839-4960-8924-37c2d9e436de)




## References
1. [L. Piegl & W. Tiller. "The NURBS book". Springer Science & Business Media (2012)](https://link.springer.com/book/10.1007/978-3-642-59223-2).
2. [R. T. Farouki, "Pythagorean-Hodograph Curves: Algebra and Geometry Inseparable", Geometry and Computing (Eds. H. Edelsbrunner, K. Polthier, and L Kobbelt) Springer (2008)](https://link.springer.com/book/10.1007/978-3-540-73398-0).
3. [R.T. Farouki, "Construction of G2 rounded corners with Pythagorean-hodograph curves", Computer Aided Geometric Design, 31(2) (2014)](https://escholarship.org/uc/item/6fq8n655).
