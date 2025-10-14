use fluxara_core::Config;

pub struct FluxaraApp {
    config: Config,
}

impl FluxaraApp {
    pub fn new() -> Self {
        let config = Config::load().unwrap_or_default();
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut Config {
        &mut self.config
    }

    pub fn save_config(&self) -> anyhow::Result<()> {
        self.config.save()
    }
}
