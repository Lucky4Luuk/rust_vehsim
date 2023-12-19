pub mod welded_diff;

pub enum Differential {
    WheelConnector(crate::wheels::Wheel),
    WeldedDiff(welded_diff::WeldedDiff),
}

impl Differential {
    /// Returns the reaction torque produced by its children
    pub fn update(&mut self, delta_s: f32, vehicle_speed: f32, torque_in: f32) -> f32 {
        match self {
            Self::WheelConnector(wheel) => wheel.update(delta_s, vehicle_speed, torque_in),
            Self::WeldedDiff(diff) => diff.update(delta_s, vehicle_speed, torque_in),
        }
    }
}
