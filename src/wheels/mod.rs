// An implementation of a (simple) tyre model, based on the tyre model used in BeamNG.Drive
// For the reference implementation, see /lua/vehicle/wheels.lua in a valid BeamNG.Drive install

pub mod tyre_model;

pub struct Wheel {
    /// Tyre data
    tyre: tyre_model::TyreData,
    /// Determines the wheel direction, to differentiate between left and right wheels
    direction: f32,
    /// The radius of the wheel, including tyre
    radius: f32,
    /// The mass of the wheel, including tyre, in kg
    mass: f32,

    /// Deflated, does not imply broken!
    deflated: bool,
    /// Whether the wheel is still attached to its halfshaft
    broken: bool,

    /// Updated whenever calc_wheel_accel_torque is called
    last_slip: f32,

    last_angular_vel: f32,

    angular_vel: f32,

    wheel_speed: f32,
}

impl Wheel {
    /// Returns the reaction torque
    pub fn update(&mut self, delta_s: f32, torque_in: f32) -> f32 {
        if self.broken { return 0.0; } // Return early if the wheel is broken

        // let brake_input = brake_input.min(1.0);
        // let parking_brake_input = parking_brake_input.min(1.0);

        let accel_torque = self.calc_wheel_accel_torque(torque_in);
        // TODO: Brake torque? Seems like we can just calculate the brake torque and add it to this
        let total_torque = accel_torque;

        self.update_wheel_velocity(delta_s, total_torque);

        total_torque
    }

    // TODO: Incorporate tyre model
    // TODO: Incorporate ground model friction coefficient
    // TODO: This seems very wrong?
    fn calc_wheel_accel_torque(&mut self, torque_in: f32) -> f32 {
        let tyre_friction_coefficient = 0.9; // TODO: This should be calculated using the tyre model
        let ground_mat_friction_coefficient = 1.0;
        let friction_coefficient = tyre_friction_coefficient * ground_mat_friction_coefficient;
        self.last_slip = 1.0 - friction_coefficient;
        torque_in * self.last_slip
    }

    fn update_wheel_velocity(&mut self, delta_s: f32, torque: f32) {
        self.last_angular_vel = self.angular_vel;

        let inertia = self.mass * (self.radius * self.radius) / 2.0;
        let angular_accel = torque / inertia;
        self.angular_vel += angular_accel * delta_s;

        self.wheel_speed = self.angular_vel * self.direction * self.radius;
    }
}
