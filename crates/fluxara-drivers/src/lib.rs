use anyhow::{Context, Result};
use fluxara_core::{DriverInfo, DriverType};
use std::process::Command;

pub struct DriverManager;

impl Default for DriverManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DriverManager {
    pub fn new() -> Self {
        Self
    }

    /// Detect hardware devices and suggest drivers
    pub fn detect_hardware(&self) -> Result<Vec<HardwareDevice>> {
        let mut devices = Vec::new();

        // Parse lspci output
        if let Ok(pci_devices) = self.list_pci_devices() {
            devices.extend(pci_devices);
        }

        // Parse lsusb output
        if let Ok(usb_devices) = self.list_usb_devices() {
            devices.extend(usb_devices);
        }

        Ok(devices)
    }

    fn list_pci_devices(&self) -> Result<Vec<HardwareDevice>> {
        let output = Command::new("lspci")
            .args(["-nn"])
            .output()
            .context("Failed to run lspci")?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let devices = stdout
            .lines()
            .map(|line| HardwareDevice {
                id: line.to_string(),
                name: line.to_string(),
                device_type: DeviceType::Pci,
            })
            .collect();

        Ok(devices)
    }

    fn list_usb_devices(&self) -> Result<Vec<HardwareDevice>> {
        let output = Command::new("lsusb")
            .output()
            .context("Failed to run lsusb")?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let devices = stdout
            .lines()
            .map(|line| HardwareDevice {
                id: line.to_string(),
                name: line.to_string(),
                device_type: DeviceType::Usb,
            })
            .collect();

        Ok(devices)
    }

    /// Suggest drivers for a hardware device
    pub fn suggest_drivers(&self, _device: &HardwareDevice) -> Vec<DriverInfo> {
        // Stub implementation - would use a database of driver mappings
        vec![]
    }

    /// Check for proprietary driver availability (NVIDIA, AMD, etc.)
    pub fn check_proprietary_drivers(&self) -> Result<Vec<DriverInfo>> {
        let mut drivers = Vec::new();

        // Check for NVIDIA GPU
        if self.has_nvidia_gpu()? {
            drivers.push(DriverInfo {
                name: "nvidia".to_string(),
                description: "NVIDIA Proprietary Driver".to_string(),
                device_ids: vec![],
                driver_type: DriverType::Proprietary,
                recommended: true,
            });
        }

        Ok(drivers)
    }

    fn has_nvidia_gpu(&self) -> Result<bool> {
        let output = Command::new("lspci")
            .output()
            .context("Failed to run lspci")?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.to_lowercase().contains("nvidia"))
    }
}

#[derive(Debug, Clone)]
pub struct HardwareDevice {
    pub id: String,
    pub name: String,
    pub device_type: DeviceType,
}

#[derive(Debug, Clone)]
pub enum DeviceType {
    Pci,
    Usb,
}
