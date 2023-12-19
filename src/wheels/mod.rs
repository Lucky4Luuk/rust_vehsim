// An implementation of a (simple) tyre model, based on the tyre model used in BeamNG.Drive
// For the reference implementation, see /lua/vehicle/wheels.lua in a valid BeamNG.Drive install

pub mod tyre_model;

#[derive(Debug, Copy, Clone)]
pub struct Wheel {
    /// Tyre data
    pub tyre: tyre_model::TyreData,
    /// Determines the wheel direction, to differentiate between left and right wheels
    pub direction: f32,
    /// The radius of the wheel, including tyre
    pub radius: f32,
    /// The mass of the wheel, including tyre, in kg
    pub mass: f32,

    /// Deflated, does not imply broken!
    pub deflated: bool,
    /// Whether the wheel is still attached to its halfshaft
    pub broken: bool,

    /// Updated whenever calc_wheel_accel_torque is called
    pub last_slip: f32,

    pub last_angular_vel: f32,

    pub angular_vel: f32,

    pub wheel_speed: f32,
}

impl Wheel {
    /// Returns the reaction torque
    pub fn update(&mut self, delta_s: f32, vehicle_speed: f32, torque_in: f32) -> f32 {
        if self.broken { return 0.0; } // Return early if the wheel is broken

        // let brake_input = brake_input.min(1.0);
        // let parking_brake_input = parking_brake_input.min(1.0);

        let accel_torque = self.calc_wheel_accel_torque(vehicle_speed, torque_in);
        println!("accel / in: {} / {}", accel_torque, torque_in);
        // TODO: Brake torque? Seems like we can just calculate the brake torque and add it to this
        let total_torque = accel_torque;

        self.update_wheel_velocity(delta_s, torque_in);

        total_torque
    }

    // TODO: Incorporate tyre model
    // TODO: Incorporate ground model friction coefficient
    // TODO: I think vehicle speed needs to be the individual wheel speed here.
    //       This should be good enough for testing, but it's not correct!
    fn calc_wheel_accel_torque(&mut self, vehicle_speed: f32, torque_in: f32) -> f32 {
        let tyre_friction_coefficient = 0.9; // TODO: This should be calculated using the tyre model
        let ground_mat_friction_coefficient = 1.0;
        let friction_coefficient = tyre_friction_coefficient * ground_mat_friction_coefficient;

        // We introduce a tiny minimum speed to prevent division by zero
        // Same for the angular velocity
        const EPSILON: f32 = 0.01;
        let slip_ratio = (self.last_angular_vel.abs() * self.radius).min(EPSILON) / vehicle_speed.abs().max(EPSILON);

        self.last_slip = slip_ratio;

        self.tyre.calculate_accel_force(slip_ratio)
    }

    fn update_wheel_velocity(&mut self, delta_s: f32, torque: f32) {
        self.last_angular_vel = self.angular_vel;

        let inertia = self.mass * (self.radius * self.radius) / 2.0;
        let angular_accel = torque / inertia;
        self.angular_vel += angular_accel * delta_s;

        self.wheel_speed = self.angular_vel * self.direction * self.radius;
    }
}
