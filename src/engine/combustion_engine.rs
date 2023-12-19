pub struct CombustionEngine {
    pub torque_curve: Vec<(f32, f32)>,
    pub idle_rpm: f32,
    pub max_rpm: f32,

    pub current_rpm: f32,

    /// Engine inertia
    pub inertia: f32,

    /// Friction that is always applied, regardless of spinning speed
    pub static_friction: f32,
    /// Friction that goes up depending on RPM
    pub variable_friction: f32,
    /// Multiplier for variable friction
    pub variable_friction_mult: f32,
}

impl CombustionEngine {
    pub fn get_torque_data(&self) -> &Vec<(f32, f32)> {
        &self.torque_curve
    }

    fn find_closest_torque_points(&self, rpm: f32) -> ((f32, f32), (f32, f32)) {
        for i in 0..(self.torque_curve.len()-1) {
            let this = self.torque_curve[i];
            let next = self.torque_curve[i+1];
            if this.0 <= rpm && next.0 >= rpm {
                return (this, next);
            }
        }
        // If nothing was found, check if RPM exceeds the RPM limit or is below the minimum rpm
        let min_rpm = self.torque_curve[0].0;
        let max_rpm = self.torque_curve[self.torque_curve.len()-1].0;
        if rpm < min_rpm {
            return (self.torque_curve[0], self.torque_curve[0]);
        }
        if rpm > max_rpm {
            return (self.torque_curve[self.torque_curve.len()-1], self.torque_curve[self.torque_curve.len()-1]);
        }
        unreachable!("We should not be here! Rpm: {}", rpm);
    }

    pub fn sample_torque_at_rpm(&self, rpm: f32) -> f32 {
        let ((rpm_lower, torque_lower), (rpm_upper, torque_upper)) = self.find_closest_torque_points(rpm);
        let t = (rpm - rpm_lower) / (rpm_upper - rpm_lower);
        torque_lower + (torque_upper - torque_lower) * t
    }

    pub fn calc_torque(&mut self, throttle_input: f32) -> (f32, f32) {
        let torque = self.sample_torque_at_rpm(self.current_rpm) * throttle_input;
        let friction = self.static_friction + (self.variable_friction * self.variable_friction_mult * self.current_rpm);
        let friction_torque = friction.min(self.current_rpm * self.inertia * 2000f32);

        (torque, friction_torque)
    }

    /// Updates the engine and returns the torque produced by the engine
    pub fn update(&mut self, delta_s: f32, throttle_input: f32, child: &mut crate::differential::Differential) {
        let throttle_input = throttle_input.max(0.0).min(1.0);

        let (torque, friction_torque) = self.calc_torque(throttle_input);
        let reaction_torque = child.update(delta_s, torque);
        let final_torque = torque - friction_torque - reaction_torque;


    }
}
