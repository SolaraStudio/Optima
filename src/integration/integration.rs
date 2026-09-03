use crate::api::config::EngineConfig;
use crate::api::engine::Engine;
use crate::config::settings::Settings;

pub struct Integration {
    pub engine: Engine,
    pub settings: Settings,
}

impl Integration {
    pub fn new(settings: Settings) -> Self {
        Integration {
            engine: Engine::new(EngineConfig::from_settings(&settings)),
            settings,
        }
    }

    pub fn initialize(&mut self) -> Result<(), String> {
        self.engine.start();
        Ok(())
    }

    pub fn shutdown(&mut self) {
        self.engine.stop();
    }

    pub fn load_html(&mut self, html: &str, base_url: &str) -> Result<(), String> {
        self.engine.load_html(html, base_url);
        Ok(())
    }

    pub fn tick(&mut self) {
        self.engine.tick();
    }
}
