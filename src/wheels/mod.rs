// An implementation of a (simple) tyre model, based on the tyre model used in BeamNG.Drive
// For the reference implementation, see /lua/vehicle/wheels.lua in a valid BeamNG.Drive install

pub mod tyre_model;

pub struct Wheel {
    /// Determines the wheel direction, to differentiate between left and right wheels
    direction: f32,
    /// The radius of the wheel, including tyre
    radius: f32,

    /// Deflated, does not imply broken!
    deflated: bool,
    /// Whether the wheel is still attached to its halfshaft
    broken: bool,

    last_angular_vel: f32,

    angular_vel: f32,

    wheel_speed: f32,
}

impl Wheel {
    pub fn updateWheel(&mut self, delta_s: f32, brake_input: f32, parking_brake_input: f32) {
        if self.broken { return; } // Return early if the wheel is broken

        let brake_input = brake_input.min(1.0);
        let parking_brake_input = parking_brake_input.min(1.0);
    }

    fn updateWheelVelocity(&mut self, _delta_s: f32) {
        self.last_angular_vel = self.angular_vel;

        self.wheel_speed = self.angular_vel * self.direction * self.radius;
    }
}
