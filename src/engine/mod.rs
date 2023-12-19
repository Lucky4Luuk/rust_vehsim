pub mod combustion_engine;

pub enum Engine {
    CombustionEngine(combustion_engine::CombustionEngine),
}
