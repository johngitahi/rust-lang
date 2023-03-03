use rand::distributions::{Distribution, Normal};
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate random data about community health
    let n_samples = 100;
    let mut rng = rand::thread_rng();
    let norm_dist = Normal::new(50.0, 10.0);
    let data: Vec<f64> = (0..n_samples).map(|_| norm_dist.sample(&mut rng)).collect();

    // Set up the plot
    let root = BitMapBackend::new("community_health.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Community Health", ("sans-serif", 40))
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..n_samples, 0.0..100.0)?;

    // Plot the data
    chart
        .draw_series(LineSeries::new(
            data.iter().enumerate().map(|(x, y)| (x, *y)),
            &BLUE,
        ))?
        .label("Community Health")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart.configure_series_labels().background_style(&WHITE.mix(0.8)).draw()?;

    Ok(())
}

