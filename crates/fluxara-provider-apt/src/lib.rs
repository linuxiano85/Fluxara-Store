use anyhow::{Context, Result};
use async_trait::async_trait;
use fluxara_core::{InstallPlan, Package, PackageManager, PackageSource, UpdateInfo};
use std::process::Command;

pub struct AptProvider;

impl Default for AptProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl AptProvider {
    pub fn new() -> Self {
        Self
    }

    fn run_command(&self, args: &[&str]) -> Result<String> {
        let output = Command::new("apt-get")
            .args(args)
            .output()
            .context("Failed to execute apt-get command")?;

        if !output.status.success() {
            anyhow::bail!(
                "apt-get command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

#[async_trait]
impl PackageManager for AptProvider {
    async fn search(&self, query: &str) -> Result<Vec<Package>> {
        // Stub implementation - would use apt-cache search
        let output = Command::new("apt-cache")
            .args(["search", query])
            .output()
            .context("Failed to search packages")?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let packages = stdout
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.splitn(2, " - ").collect();
                if parts.len() == 2 {
                    Some(Package {
                        id: parts[0].to_string(),
                        name: parts[0].to_string(),
                        version: None,
                        description: Some(parts[1].to_string()),
                        icon_url: None,
                        source: PackageSource::Apt,
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
        self.run_command(&["remove", "-y", package_id])?;
        Ok(())
    }

    async fn update(&self, package_id: &str) -> Result<()> {
        self.run_command(&["install", "--only-upgrade", "-y", package_id])?;
        Ok(())
    }

    async fn list_installed(&self) -> Result<Vec<Package>> {
        // Stub implementation
        Ok(vec![])
    }

    async fn list_updates(&self) -> Result<Vec<UpdateInfo>> {
        // Stub implementation
        Ok(vec![])
    }

    async fn get_install_plan(&self, package_id: &str) -> Result<InstallPlan> {
        Ok(InstallPlan {
            package_id: package_id.to_string(),
            source: PackageSource::Apt,
            version: None,
            dependencies: vec![],
            requires_root: true,
        })
    }
}
