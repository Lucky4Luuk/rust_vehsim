use plotters::prelude::*;

use rust_vehsim::wheels::tyre_model::TyreData;

fn plot<F: Fn(f32) -> f32, I: Iterator<Item = f32>>(f: F, i: I, range_x: (f32, f32), range_y: (f32, f32), name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let plot_name = format!("plot_tyre_data_{}.png", name);
    let root = BitMapBackend::new(&plot_name, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(range_x.0..range_x.1, range_y.0..range_y.1)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            i.map(|arg| (arg, f(arg))),
            &RED,
        ))?
        .label("y = x^2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}

fn main() {
    let td = TyreData {
        no_load_coeff: 2.08,
        full_load_coeff: 0.7,
        load_sensitivity: 0.00023,

        static_friction_coeff: 1.0,
        sliding_friction_coeff: 1.0,
        stribeck_velocity: 1.0,
        stribeck_exponent: 2.0,
    };

    plot(|load| td.calculate_friction_coeff(0.0, load), (0..8000).into_iter().map(|i| i as f32), (0.0, 8000.0), (0.0, 2.0), "load").expect("Failed to plot!");
    plot(|sliding| td.calculate_friction_coeff(sliding, 0.0), (0..250).into_iter().map(|i| (i as f32) / 10f32), (0.0, 25.0), (0.0, 2.0), "sliding").expect("Failed to plot!");
}
