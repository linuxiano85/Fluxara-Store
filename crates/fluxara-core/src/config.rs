use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub ui: UiConfig,
    pub repos: ReposConfig,
    pub security: SecurityConfig,
    pub telemetry: TelemetryConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub tray_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReposConfig {
    pub flathub: FlathubConfig,
    pub aur: AurConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlathubConfig {
    pub beta_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AurConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub conversion_policy: ConversionPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConversionPolicy {
    Safe,
    Permissive,
    Strict,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryConfig {
    pub enabled: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ui: UiConfig { tray_enabled: true },
            repos: ReposConfig {
                flathub: FlathubConfig { beta_enabled: true },
                aur: AurConfig {
                    enabled: Self::is_arch_based(),
                },
            },
            security: SecurityConfig {
                conversion_policy: ConversionPolicy::Safe,
            },
            telemetry: TelemetryConfig { enabled: false },
        }
    }
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            let config = Self::default();
            config.save()?;
            return Ok(config);
        }

        let content = std::fs::read_to_string(config_path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let config_path = Self::config_path()?;

        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        std::fs::write(config_path, content)?;
        Ok(())
    }

    fn config_path() -> anyhow::Result<std::path::PathBuf> {
        let home = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE"))?;
        Ok(std::path::PathBuf::from(home)
            .join(".config")
            .join("fluxara")
            .join("config.toml"))
    }

    fn is_arch_based() -> bool {
        std::fs::read_to_string("/etc/os-release")
            .map(|content| {
                content.contains("arch")
                    || content.contains("Arch")
                    || content.contains("manjaro")
                    || content.contains("Manjaro")
            })
            .unwrap_or(false)
    }
}
