use crate::api::config::EngineConfig;
use crate::api::engine::Engine;

pub struct PrivateApi {
    engine: Engine,
}

impl PrivateApi {
    pub fn new(config: EngineConfig) -> Self {
        PrivateApi {
            engine: Engine::new(config),
        }
    }

    pub fn engine(&self) -> &Engine {
        &self.engine
    }
    pub fn engine_mut(&mut self) -> &mut Engine {
        &mut self.engine
    }
}
