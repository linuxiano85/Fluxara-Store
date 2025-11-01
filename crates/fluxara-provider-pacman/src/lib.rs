use anyhow::{Context, Result};
use async_trait::async_trait;
use fluxara_core::{InstallPlan, Package, PackageManager, PackageSource, UpdateInfo};
use std::process::Command;

pub struct PacmanProvider {
    is_manjaro: bool,
}

impl Default for PacmanProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl PacmanProvider {
    pub fn new() -> Self {
        Self {
            is_manjaro: Self::detect_manjaro(),
        }
    }

    fn detect_manjaro() -> bool {
        std::fs::read_to_string("/etc/os-release")
            .map(|content| content.contains("manjaro") || content.contains("Manjaro"))
            .unwrap_or(false)
    }

    fn run_command(&self, args: &[&str]) -> Result<String> {
        let output = Command::new("pacman")
            .args(args)
            .output()
            .context("Failed to execute pacman command")?;

        if !output.status.success() {
            anyhow::bail!(
                "Pacman command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn is_manjaro(&self) -> bool {
        self.is_manjaro
    }
}

#[async_trait]
impl PackageManager for PacmanProvider {
    async fn search(&self, query: &str) -> Result<Vec<Package>> {
        let output = self.run_command(&["-Ss", query])?;

        let mut packages = Vec::new();
        let lines: Vec<&str> = output.lines().collect();

        for chunk in lines.chunks(2) {
            if let Some(first_line) = chunk.first() {
                let parts: Vec<&str> = first_line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let name = parts[0].split('/').next_back().unwrap_or(parts[0]);
                    packages.push(Package {
                        id: name.to_string(),
                        name: name.to_string(),
                        version: Some(parts[1].to_string()),
                        description: chunk.get(1).map(|s| s.trim().to_string()),
                        icon_url: None,
                        source: PackageSource::Pacman,
                        installed: false,
                    });
                }
            }
        }

        Ok(packages)
    }

    async fn install(&self, package_id: &str) -> Result<()> {
        self.run_command(&["-S", "--noconfirm", package_id])?;
        Ok(())
    }

    async fn remove(&self, package_id: &str) -> Result<()> {
        self.run_command(&["-R", "--noconfirm", package_id])?;
        Ok(())
    }

    async fn update(&self, package_id: &str) -> Result<()> {
        self.run_command(&["-S", "--noconfirm", package_id])?;
        Ok(())
    }

    async fn list_installed(&self) -> Result<Vec<Package>> {
        let output = self.run_command(&["-Q"])?;

        let packages = output
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    Some(Package {
                        id: parts[0].to_string(),
                        name: parts[0].to_string(),
                        version: Some(parts[1].to_string()),
                        description: None,
                        icon_url: None,
                        source: PackageSource::Pacman,
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
            source: PackageSource::Pacman,
            version: None,
            dependencies: vec![],
            requires_root: true,
        })
    }
}
