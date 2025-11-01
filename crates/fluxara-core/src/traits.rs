use crate::models::{InstallPlan, Package, UpdateInfo};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait PackageManager: Send + Sync {
    async fn search(&self, query: &str) -> Result<Vec<Package>>;
    async fn install(&self, package_id: &str) -> Result<()>;
    async fn remove(&self, package_id: &str) -> Result<()>;
    async fn update(&self, package_id: &str) -> Result<()>;
    async fn list_installed(&self) -> Result<Vec<Package>>;
    async fn list_updates(&self) -> Result<Vec<UpdateInfo>>;
    async fn get_install_plan(&self, package_id: &str) -> Result<InstallPlan>;
}
