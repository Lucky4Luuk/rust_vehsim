// A tyre model based on the one seen in BeamNG.Drive
// Tyre models are incredibly complex, so I've decided
// to instead replicate the model BeamNG.Drive has.
// It is easier to implement and we have existing
// reference data available already, and it does
// fairly well for now.

pub struct TyreData {
    /// Friction coefficient under no load
    pub no_load_coeff: f32,
    /// Friction coefficient under full load
    pub full_load_coeff: f32,
    /// Affects how quickly the tyre reaches full_load_coeff under load
    pub load_sensitivity: f32,

    /// Friction coefficient with zero slip
    pub static_friction_coeff: f32,
    /// Friction coefficient with max slip
    pub sliding_friction_coeff: f32,
    /// The bigger the value, the slower the transition to the sliding friction coefficient is
    pub stribeck_velocity: f32,
    /// Affects the smoothing of the stribeck curve
    pub stribeck_exponent: f32,
}

impl TyreData {
    /// sliding_vel:    m/s
    /// load:           N
    // TODO: This function needs more investigations. This is almost certainly a bit wrong.
    pub fn calculate_friction_coeff(&self, sliding_vel: f32, load: f32) -> f32 {
        let load = load / 1_000f32;
        let coeff_a = friction_coeff_while_sliding(
            self.static_friction_coeff,
            self.sliding_friction_coeff,
            self.stribeck_velocity,
            self.stribeck_exponent,
            sliding_vel,
        );
        let coeff_b = friction_coeff_under_load(
            self.no_load_coeff,
            self.full_load_coeff,
            self.load_sensitivity,
            load
        );
        coeff_a * coeff_b
    }
}

/// sliding_vel:    m/s
fn friction_coeff_while_sliding(static_friction_coeff: f32, sliding_friction_coeff: f32, stribeck_velocity: f32, stribeck_exponent: f32, sliding_vel: f32) -> f32 {
    let t = (sliding_vel / stribeck_velocity).powf(stribeck_exponent);
    lerp(static_friction_coeff, sliding_friction_coeff, t.min(1.0))
}

/// load:   N
// TODO: This function needs more investigation. I'm not sure if this is accurate to how BeamNG.Drive implements it.
//       With some reference data, it should be fairly easy to figure out how load_sensitivity needs to be used.
fn friction_coeff_under_load(no_load_coeff: f32, full_load_coeff: f32, load_sensitivity: f32, load: f32) -> f32 {
    let t = (load / load_sensitivity).min(1.0);
    lerp(no_load_coeff, full_load_coeff, t)
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
