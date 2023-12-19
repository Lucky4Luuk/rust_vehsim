use plotters::prelude::*;

use rust_vehsim::{
    differential::{
        Differential,
        welded_diff::WeldedDiff,
    },
    engine::{
        Engine,
        EngineContainer,
        combustion_engine::CombustionEngine,
    },
    wheels::{
        Wheel,
        tyre_model::TyreData,
    },
};

fn main() {
    let mut container = setup();

    let delta_s = 1.0 / 15.0;
    let test_length_s = 30.0;

    let mut data_slip = Vec::new();
    let mut data_wheel_speed = Vec::new();

    let mut fake_veh_speed = 0.0;
    let mut total_s = 0.0;
    while total_s < test_length_s {
        container.update(delta_s, fake_veh_speed, 1.0);

        if let Differential::WheelConnector(wheel) = &container.child {
            data_slip.push((total_s, wheel.last_slip.abs()));
            data_wheel_speed.push((total_s, wheel.wheel_speed));
        }

        fake_veh_speed += delta_s * 0.5;
        total_s += delta_s;
    }

    let root = BitMapBackend::new("plot_drivetrain_wheel_slip.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..test_length_s, 0f32..5.0f32).unwrap();

    chart.configure_mesh().draw().unwrap();

    // for i in 0..6 {
    //     let throttle = i as f32 / 5f32;
    //     chart
    //         .draw_series(LineSeries::new(
    //             (1100..5750).map(|rpm| {
    //                 engine.current_rpm = rpm as f32;
    //                 let (torque, friction_torque) = engine.calc_torque(throttle);
    //                 (rpm, (torque - friction_torque) as i32)
    //             }),
    //             &RED,
    //         )).unwrap()
    //         .label("y = x^2")
    //         .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    // }

    chart
        .draw_series(LineSeries::new(
            (0..(data_slip.len() as isize)).map(|i| data_slip[i as usize]),
            &RED,
        )).unwrap()
        .label("wheel slip")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw().unwrap();

    root.present().unwrap();
}

fn setup() -> EngineContainer {
    let td = TyreData {
        no_load_coeff: 2.08,
        full_load_coeff: 0.7,
        load_sensitivity: 0.00023,

        static_friction_coeff: 1.0,
        sliding_friction_coeff: 1.0,
        stribeck_velocity: 1.0,
        stribeck_exponent: 2.0,

        tyre_steepness: 22.0,
        tyre_amplitude: 3220.0,
        tyre_falloff: 2700.0,
    };

    let wheel = Wheel {
        /// Tyre data
        tyre: td,
        /// Determines the wheel direction, to differentiate between left and right wheels
        direction: 1.0,
        /// The radius of the wheel, including tyre
        radius: 0.4,
        /// The mass of the wheel, including tyre, in kg
        mass: 60.0,

        /// Deflated, does not imply broken!
        deflated: false,
        /// Whether the wheel is still attached to its halfshaft
        broken: false,

        /// Updated whenever calc_wheel_accel_torque is called
        last_slip: 0.0,

        last_angular_vel: 0.0,

        angular_vel: 0.0,

        wheel_speed: 0.0,
    };

    // let diff = Differential::WeldedDiff(WeldedDiff {
    //     children: [
    //         Box::new(Differential::WheelConnector(wheel)),
    //         Box::new(Differential::WheelConnector(wheel)),
    //     ]
    // });

    let diff = Differential::WheelConnector(wheel);

    let engine = CombustionEngine {
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

        current_rpm: 4500.0,

        inertia: 0.21,
        static_friction: 8.0,
        variable_friction: 0.008,
        variable_friction_mult: 1.0,
        // engine_brake_torque: 58,
    };

    EngineContainer {
        engine: Engine::CombustionEngine(engine),
        child: diff,
    }
}
