// TODO: Support friction in the diff
// TODO: Support diff gear ratio
pub struct WeldedDiff {
    pub children: [Box<super::Differential>; 2],
}

impl WeldedDiff {
    /// Returns the reaction torque produced by its children
    pub fn update(&mut self, delta_s: f32, vehicle_speed: f32, torque_in: f32) -> f32 {
        let output_a = torque_in * 0.5;
        let output_b = torque_in * 0.5;

        let reaction_a = self.children[0].update(delta_s, vehicle_speed, output_a);
        let reaction_b = self.children[1].update(delta_s, vehicle_speed, output_b);

        reaction_a + reaction_b
    }
}
