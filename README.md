# bernstein: polynomials in Bernstein basis

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