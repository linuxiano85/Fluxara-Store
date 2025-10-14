use anyhow::{Context, Result};
use fluxara_core::Config;
use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("Package conversion blocked by security policy: {0}")]
    SecurityBlocked(String),
    #[error("Alien command not found")]
    AlienNotFound,
    #[error("Conversion failed: {0}")]
    ConversionFailed(String),
}

pub struct PackageConverter {
    config: Config,
}

impl PackageConverter {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Blocked package categories for safe conversion
    const BLOCKED_CATEGORIES: &'static [&'static str] = &[
        "kernel",
        "driver",
        "bootloader",
        "glibc",
        "systemd",
        "grub",
        "linux-",
    ];

    pub fn convert_package(&self, package_path: &str, target_format: &str) -> Result<String> {
        // Check if alien is available
        if !self.is_alien_available() {
            return Err(ConversionError::AlienNotFound.into());
        }

        // Security check
        if self.is_blocked_package(package_path) {
            return Err(ConversionError::SecurityBlocked(format!(
                "Package {} contains blocked keywords",
                package_path
            ))
            .into());
        }

        // Run alien conversion
        let output = Command::new("alien")
            .args(&[
                "--to-".to_string() + target_format,
                package_path.to_string(),
            ])
            .output()
            .context("Failed to run alien command")?;

        if !output.status.success() {
            return Err(ConversionError::ConversionFailed(
                String::from_utf8_lossy(&output.stderr).to_string(),
            )
            .into());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.to_string())
    }

    fn is_alien_available(&self) -> bool {
        Command::new("which")
            .arg("alien")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    fn is_blocked_package(&self, package_path: &str) -> bool {
        let package_lower = package_path.to_lowercase();

        Self::BLOCKED_CATEGORIES
            .iter()
            .any(|blocked| package_lower.contains(blocked))
    }
}
