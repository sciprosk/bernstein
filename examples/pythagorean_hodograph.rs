// Construct a quintic Pyhagorean-hodograph cornering curve in the complex
// representation an draw it using Plotters backend.
//
// For details about Pythagorean-hodograph curves see e.g.
// See e.g. R.T. Farouki, "Construction of G2 rounded corners with
// Pythagorean-hodograph curves", Computer Aided Geometric Design, 31(2) (2014).
// https://escholarship.org/uc/item/6fq8n655.
//
// For more examples on Plotters, see https://github.com/plotters-rs/plotters.

#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use bernstein::Bernstein;
use num::{complex::Complex32 as Complex, Zero};
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Specify parameters of the curve
    let d = 1.0;                             // cornering length -- distance
                                                  // from entry/exit to the corner
    let theta = 0.0;                         // orientation of the corner
    let delta = std::f32::consts::FRAC_PI_2; // angle of the corner

    // Actual formula that sets the parameter for the internal representation
    // of the quintic Pythgorean-hodograh curve.
    // See e.g. R.T. Farouki, "Construction of G2 rounded corners with
    // Pythagorean-hodograph curves", Computer Aided Geometric Design, 31(2) (2014).
    // https://escholarship.org/uc/item/6fq8n655.
    let rho = f32::sqrt(30.0 * d * f32::cos(0.5 * delta) / (6.0 * f32::cos(0.5 * delta) + 1.0));

    // Initial point of the curve in the complex plane.
    let p0 = Complex::new(-1.0, 0.0);

    // Control poligon of the internal representation of the curve
    // in the complex plane (cubic).
    let w0 = rho * Complex::exp(0.5 * theta * Complex::i());
    let w1 = Complex::zero();
    let w2 = rho * Complex::exp(0.5 * (theta + delta) * Complex::i());

    // Construct Bernstein polynomial in the complex plane from control polygon.
    let w: Bernstein<Complex, f32, 3> = Bernstein::new([w0, w1, w2]);

    // Take the square and integrate from the initial point.
    let p = (w * w).integ(p0);

    // Draw the picture using Plotters backend.
    let root = BitMapBackend::new("pythgorean_hodograph.png", (800, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("5th Degree Pythagorean-Hodograph", ("sans-serif", 30).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-1.1f32..0.1f32, -0.1f32..1.1f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new((0..=1000)
        // Sample the curve at each point along the x-axis.
        .map(|x| x as f32 / (1000 as f32))
        .map(|x| (p.eval(x).re, p.eval(x).im)), &RED))?;

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}