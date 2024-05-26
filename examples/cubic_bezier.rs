// Draw cubic Bezier curve by specifying it control polygon using
// Plotters crate: https://github.com/plotters-rs/plotters.
//
// Adapted from plotters examples:

use bernstein::Bernstein;
use num::Complex;
use plotters::prelude::*;

const FILENAME: &str = "cubic_bezier.png";
const PLOT_LABEL: &str = "Control polygon: (0.0), (-0.5, 1.0), (2.5, 1.0), (2.0, 0.0)";
const NUMBER_OF_POINTS: i32 = 2000;

// Specify control polygon of the Bezier curve as points in the complex plane.
const CONTROL_POLYGON: [Complex<f32>; 4] = [
    Complex::new(0.0, 0.0),
    Complex::new(2.5, 1.0),
    Complex::new(-0.5, 1.0),
    Complex::new(2.0, 0.0)];

fn main()-> Result<(), Box<dyn std::error::Error>> {

    // Create an instance of a Bezier curve from the control polygon.
    let c: Bernstein<Complex<f32>, f32, 4> = Bernstein::new(CONTROL_POLYGON);

    // For drawing with Plotters, see: https://github.com/plotters-rs/plotters.
    let root = BitMapBackend::new(FILENAME, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Cubic Bezier Curve", ("sans-serif", 30).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-0.1f32..2.1f32, -0.1f32..1f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new((0..=NUMBER_OF_POINTS)
        // Sample the curve at each point along the x-axis.
        .map(|x| x as f32 / (NUMBER_OF_POINTS as f32))
        .map(|x| (c.eval(x).re, c.eval(x).im)), &RED)).unwrap()
        .label(PLOT_LABEL)
        .legend(|(x,y)| PathElement::new(vec![(x,y), (x + 20,y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}