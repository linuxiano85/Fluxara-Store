use anyhow::{Context, Result};
use async_trait::async_trait;
use fluxara_core::{InstallPlan, Package, PackageManager, PackageSource, UpdateInfo};
use std::process::Command;

pub struct FlatpakProvider;

impl Default for FlatpakProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl FlatpakProvider {
    pub fn new() -> Self {
        Self
    }

    fn run_command(&self, args: &[&str]) -> Result<String> {
        let output = Command::new("flatpak")
            .args(args)
            .output()
            .context("Failed to execute flatpak command")?;

        if !output.status.success() {
            anyhow::bail!(
                "Flatpak command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

#[async_trait]
impl PackageManager for FlatpakProvider {
    async fn search(&self, query: &str) -> Result<Vec<Package>> {
        let output = self.run_command(&["search", query])?;

        // Parse flatpak search output (stub implementation)
        let packages = output
            .lines()
            .skip(1) // Skip header
            .filter_map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    Some(Package {
                        id: parts[1].to_string(),
                        name: parts[0].to_string(),
                        version: parts.get(2).map(|s| s.to_string()),
                        description: parts.get(3..).map(|s| s.join(" ")),
                        icon_url: None,
                        source: PackageSource::Flatpak,
                        installed: false,
                    })
                } else {
                    None
                }
            })
            .collect();

        Ok(packages)
    }

    async fn install(&self, package_id: &str) -> Result<()> {
        self.run_command(&["install", "-y", package_id])?;
        Ok(())
    }

    async fn remove(&self, package_id: &str) -> Result<()> {
        self.run_command(&["uninstall", "-y", package_id])?;
        Ok(())
    }

    async fn update(&self, package_id: &str) -> Result<()> {
        self.run_command(&["update", "-y", package_id])?;
        Ok(())
    }

    async fn list_installed(&self) -> Result<Vec<Package>> {
        let output = self.run_command(&["list", "--app"])?;

        let packages = output
            .lines()
            .skip(1)
            .filter_map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    Some(Package {
                        id: parts[1].to_string(),
                        name: parts[0].to_string(),
                        version: parts.get(2).map(|s| s.to_string()),
                        description: None,
                        icon_url: None,
                        source: PackageSource::Flatpak,
                        installed: true,
                    })
                } else {
                    None
                }
            })
            .collect();

        Ok(packages)
    }

    async fn list_updates(&self) -> Result<Vec<UpdateInfo>> {
        // Stub implementation
        Ok(vec![])
    }

    async fn get_install_plan(&self, package_id: &str) -> Result<InstallPlan> {
        Ok(InstallPlan {
            package_id: package_id.to_string(),
            source: PackageSource::Flatpak,
            version: None,
            dependencies: vec![],
            requires_root: false,
        })
    }
}
