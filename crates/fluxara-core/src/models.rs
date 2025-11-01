use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub id: String,
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub source: PackageSource,
    pub installed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PackageSource {
    Flatpak,
    Apt,
    Pacman,
    Aur,
    Snap,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallPlan {
    pub package_id: String,
    pub source: PackageSource,
    pub version: Option<String>,
    pub dependencies: Vec<String>,
    pub requires_root: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoInfo {
    pub name: String,
    pub url: String,
    pub enabled: bool,
    pub speed_mbps: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInfo {
    pub package_id: String,
    pub current_version: String,
    pub new_version: String,
    pub source: PackageSource,
    pub size_bytes: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverInfo {
    pub name: String,
    pub description: String,
    pub device_ids: Vec<String>,
    pub driver_type: DriverType,
    pub recommended: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DriverType {
    Open,
    Proprietary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppRating {
    pub app_id: String,
    pub average_rating: f32,
    pub total_reviews: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppReview {
    pub app_id: String,
    pub rating: u32,
    pub summary: String,
    pub description: String,
    pub reviewer: String,
    pub date: String,
}
