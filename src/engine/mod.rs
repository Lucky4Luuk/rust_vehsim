pub mod combustion_engine;

pub enum Engine {
    CombustionEngine(combustion_engine::CombustionEngine),
}

pub struct EngineContainer {
    pub engine: Engine,
    pub child: crate::differential::Differential,
}

impl EngineContainer {
    pub fn update(&mut self, delta_s: f32, throttle_input: f32) {
        match &mut self.engine {
            Engine::CombustionEngine(engine) => engine.update(delta_s, throttle_input, &mut self.child),
        }
    }
}
