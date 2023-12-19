use plotters::prelude::*;

use rust_vehsim::engine::combustion_engine::CombustionEngine;

fn plot<F: FnMut(f32) -> f32, I: Iterator<Item = f32>>(mut f: F, i: I, range_x: (f32, f32), range_y: (f32, f32), name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let plot_name = format!("plot_engine_torque_{}.png", name);
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
    let mut engine = CombustionEngine {
        torque_curve: vec![
            (1000.0, 393.0),
            (1500.0, 420.0),
            (2000.0, 435.0),
            (2500.0, 448.0),
            (3000.0, 455.0),
            (3500.0, 463.0),
            (4000.0, 471.0),
            (4500.0, 475.0),
            (5000.0, 463.0),
            (5500.0, 440.0),
            (5800.0, 395.0),
        ],
        idle_rpm: 1100.0,
        max_rpm: 5750.0,

        current_rpm: 0.0,

        inertia: 0.21,
        static_friction: 8.0,
        variable_friction: 0.008,
        variable_friction_mult: 1.0,
        // engine_brake_torque: 58,
    };

    plot(|rpm| {
        engine.current_rpm = rpm;
        let (torque, _friction_torque) = engine.calc_torque(1.0);
        torque
    }, (1100..5750).into_iter().map(|i| i as f32), (0.0, 7000.0), (0.0, 550.0), "torque").expect("Failed to plot!");
    plot(|rpm| {
        engine.current_rpm = rpm;
        let (_torque, friction_torque) = engine.calc_torque(1.0);
        friction_torque
    }, (1100..5750).into_iter().map(|i| i as f32), (0.0, 7000.0), (0.0, 550.0), "frictionTorque").expect("Failed to plot!");

    let root = BitMapBackend::new(&"plot_engine_torque_throttle.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..7000, -200..550).unwrap();

    chart.configure_mesh().draw().unwrap();

    for i in 0..6 {
        let throttle = i as f32 / 5f32;
        chart
            .draw_series(LineSeries::new(
                (1100..5750).map(|rpm| {
                    engine.current_rpm = rpm as f32;
                    let (torque, friction_torque) = engine.calc_torque(throttle);
                    (rpm, (torque - friction_torque) as i32)
                }),
                &RED,
            )).unwrap()
            .label("y = x^2")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    }

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw().unwrap();

    root.present().unwrap();
}
