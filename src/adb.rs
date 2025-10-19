//! ADB Client Abstraction Layer
//!
//! This module provides a high-level interface to ADB operations using the adb_client crate.
//! It abstracts away the complexity of working with ADB and provides typed command execution.

use adb_client::{ADBDeviceExt, ADBServer};
use std::io;
use std::net::{Ipv4Addr, SocketAddrV4};

/// Result type for ADB operations
pub type AdbResult<T> = Result<T, AdbError>;

/// Custom error type for ADB operations
#[derive(Debug)]
pub enum AdbError {
    ConnectionError(String),
    DeviceNotFound,
    CommandFailed(String),
    IoError(io::Error),
    ParseError(String),
    NoDeviceSelected,
}

impl std::fmt::Display for AdbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AdbError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            AdbError::DeviceNotFound => write!(f, "Device not found"),
            AdbError::CommandFailed(msg) => write!(f, "Command failed: {}", msg),
            AdbError::IoError(e) => write!(f, "IO error: {}", e),
            AdbError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            AdbError::NoDeviceSelected => write!(f, "No device selected"),
        }
    }
}

impl std::error::Error for AdbError {}

impl From<io::Error> for AdbError {
    fn from(e: io::Error) -> Self {
        AdbError::IoError(e)
    }
}

impl From<adb_client::RustADBError> for AdbError {
    fn from(e: adb_client::RustADBError) -> Self {
        AdbError::CommandFailed(e.to_string())
    }
}

/// ADB command type
#[derive(Debug, Clone)]
pub enum AdbCommand {
    // Device commands
    ListDevices,
    GetDeviceState,
    GetSerialNumber,

    // Package commands
    ListPackages {
        include_path: bool,
        filter: PackageFilter,
    },
    GetPackageInfo {
        package_name: String,
    },
    InstallPackage {
        apk_path: String,
    },
    UninstallPackage {
        package_name: String,
    },
    ClearPackageData {
        package_name: String,
    },

    // System commands
    GetBatteryInfo,
    GetMemoryInfo,
    GetCpuInfo,
    GetDeviceProperties,
    GetSystemLog {
        lines: usize,
    },

    // Network commands
    GetNetworkInfo,
    GetWifiStatus,

    // Screen commands
    TakeScreenshot,
    GetScreenResolution,

    // Process commands
    ListProcesses,
    ForceStop {
        package_name: String,
    },

    // Shell commands
    Shell {
        command: String,
    },

    // Version
    GetAdbVersion,
}

/// Package filter options
#[derive(Debug, Clone)]
pub enum PackageFilter {
    All,
    User,     // -3
    System,   // -s
    Enabled,  // -e
    Disabled, // -d
}

/// Device information
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub serial: String,
    pub state: String,
    pub model: Option<String>,
    pub device: Option<String>,
}

/// ADB Manager - handles connection and command execution
#[derive(Debug)]
pub struct AdbManager {
    server: Option<ADBServer>,
    selected_device: Option<String>,
}

impl AdbManager {
    /// Create a new ADB manager
    pub fn new() -> Self {
        Self {
            server: None,
            selected_device: None,
        }
    }

    /// Connect to ADB server
    pub fn connect(&mut self) -> AdbResult<()> {
        let socket_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 5037);
        let server = ADBServer::new(socket_addr);
        self.server = Some(server);
        Ok(())
    }

    /// Ensure server is connected
    fn get_server(&mut self) -> AdbResult<&mut ADBServer> {
        if self.server.is_none() {
            self.connect()?;
        }
        self.server
            .as_mut()
            .ok_or(AdbError::ConnectionError("Failed to connect".to_string()))
    }

    /// Set the selected device
    pub fn select_device(&mut self, serial: String) {
        self.selected_device = Some(serial);
    }

    /// Get the selected device serial
    fn get_selected_device(&self) -> AdbResult<&str> {
        self.selected_device
            .as_deref()
            .ok_or(AdbError::NoDeviceSelected)
    }

    /// Execute an ADB command
    pub fn execute(&mut self, command: AdbCommand) -> AdbResult<String> {
        match command {
            AdbCommand::ListDevices => self.list_devices(),
            AdbCommand::GetDeviceState => self.get_device_state(),
            AdbCommand::GetSerialNumber => self.get_serial_number(),
            AdbCommand::ListPackages {
                include_path,
                filter,
            } => self.list_packages(include_path, filter),
            AdbCommand::GetPackageInfo { package_name } => self.get_package_info(&package_name),
            AdbCommand::InstallPackage { apk_path } => self.install_package(&apk_path),
            AdbCommand::UninstallPackage { package_name } => self.uninstall_package(&package_name),
            AdbCommand::ClearPackageData { package_name } => self.clear_package_data(&package_name),
            AdbCommand::GetBatteryInfo => self.get_battery_info(),
            AdbCommand::GetMemoryInfo => self.get_memory_info(),
            AdbCommand::GetCpuInfo => self.get_cpu_info(),
            AdbCommand::GetDeviceProperties => self.get_device_properties(),
            AdbCommand::GetSystemLog { lines } => self.get_system_log(lines),
            AdbCommand::GetNetworkInfo => self.get_network_info(),
            AdbCommand::GetWifiStatus => self.get_wifi_status(),
            AdbCommand::TakeScreenshot => self.take_screenshot(),
            AdbCommand::GetScreenResolution => self.get_screen_resolution(),
            AdbCommand::ListProcesses => self.list_processes(),
            AdbCommand::ForceStop { package_name } => self.force_stop(&package_name),
            AdbCommand::Shell { command } => self.shell_command(&command),
            AdbCommand::GetAdbVersion => self.get_adb_version(),
        }
    }

    /// List all connected devices
    fn list_devices(&mut self) -> AdbResult<String> {
        let server = self.get_server()?;
        let devices = server.devices()?;

        if devices.is_empty() {
            return Ok("No devices found.\n\nMake sure:\n- Device is connected via USB\n- USB debugging is enabled\n- Device is authorized".to_string());
        }

        let mut output = String::from("List of devices attached:\n");
        for device in devices {
            output.push_str(&format!("{}\t{:?}\n", device.identifier, device.state));

            // Auto-select first device if none selected
            if self.selected_device.is_none() {
                self.selected_device = Some(device.identifier.clone());
            }
        }

        Ok(output)
    }

    /// Get device state
    fn get_device_state(&mut self) -> AdbResult<String> {
        let serial = self.get_selected_device()?.to_string();
        let server = self.get_server()?;
        let devices = server.devices()?;

        for device in devices {
            if device.identifier == serial {
                return Ok(format!("Device state: {:?}", device.state));
            }
        }

        Err(AdbError::DeviceNotFound)
    }

    /// Get device serial number
    fn get_serial_number(&mut self) -> AdbResult<String> {
        let serial = self.get_selected_device()?;
        Ok(format!("Serial number: {}", serial))
    }

    /// List packages
    fn list_packages(&mut self, include_path: bool, filter: PackageFilter) -> AdbResult<String> {
        let filter_arg = match filter {
            PackageFilter::All => "",
            PackageFilter::User => " -3",
            PackageFilter::System => " -s",
            PackageFilter::Enabled => " -e",
            PackageFilter::Disabled => " -d",
        };

        let path_arg = if include_path { " -f" } else { "" };
        let command = format!("pm list packages{}{}", path_arg, filter_arg);

        self.shell_command(&command)
    }

    /// Get package information
    fn get_package_info(&mut self, package_name: &str) -> AdbResult<String> {
        let command = format!("dumpsys package {}", package_name);
        self.shell_command(&command)
    }

    /// Install package
    fn install_package(&mut self, _apk_path: &str) -> AdbResult<String> {
        // Note: adb_client doesn't have direct install support in the API we're using
        // We'll use shell command as fallback
        Err(AdbError::CommandFailed(
            "Install via adb_client not yet implemented. Use shell command.".to_string(),
        ))
    }

    /// Uninstall package
    fn uninstall_package(&mut self, package_name: &str) -> AdbResult<String> {
        let command = format!("pm uninstall {}", package_name);
        self.shell_command(&command)
    }

    /// Clear package data
    fn clear_package_data(&mut self, package_name: &str) -> AdbResult<String> {
        let command = format!("pm clear {}", package_name);
        self.shell_command(&command)
    }

    /// Get battery information
    fn get_battery_info(&mut self) -> AdbResult<String> {
        self.shell_command("dumpsys battery")
    }

    /// Get memory information
    fn get_memory_info(&mut self) -> AdbResult<String> {
        self.shell_command("dumpsys meminfo")
    }

    /// Get CPU information
    fn get_cpu_info(&mut self) -> AdbResult<String> {
        self.shell_command("cat /proc/cpuinfo")
    }

    /// Get device properties
    fn get_device_properties(&mut self) -> AdbResult<String> {
        self.shell_command("getprop")
    }

    /// Get system log
    fn get_system_log(&mut self, lines: usize) -> AdbResult<String> {
        let command = format!("logcat -d -t {}", lines);
        self.shell_command(&command)
    }

    /// Get network information
    fn get_network_info(&mut self) -> AdbResult<String> {
        self.shell_command("dumpsys connectivity")
    }

    /// Get WiFi status
    fn get_wifi_status(&mut self) -> AdbResult<String> {
        self.shell_command("ip addr show wlan0")
    }

    /// Take screenshot
    fn take_screenshot(&mut self) -> AdbResult<String> {
        self.shell_command("screencap -p /sdcard/screenshot.png")
    }

    /// Get screen resolution
    fn get_screen_resolution(&mut self) -> AdbResult<String> {
        let size = self.shell_command("wm size")?;
        let density = self.shell_command("wm density")?;
        Ok(format!("{}\n{}", size, density))
    }

    /// List processes
    fn list_processes(&mut self) -> AdbResult<String> {
        self.shell_command("ps")
    }

    /// Force stop application
    fn force_stop(&mut self, package_name: &str) -> AdbResult<String> {
        let command = format!("am force-stop {}", package_name);
        self.shell_command(&command)
    }

    /// Execute shell command
    fn shell_command(&mut self, command: &str) -> AdbResult<String> {
        let server = self.get_server()?;
        let mut device = server
            .get_device()
            .map_err(|e| AdbError::ConnectionError(format!("Failed to get device: {}", e)))?;

        let mut output = Vec::new();
        device
            .shell_command(&[command], &mut output)
            .map_err(|e| AdbError::CommandFailed(format!("Shell command failed: {}", e)))?;

        let result = String::from_utf8_lossy(&output).to_string();

        if result.trim().is_empty() {
            Ok("Command executed successfully (no output)".to_string())
        } else {
            Ok(result)
        }
    }

    /// Get ADB version
    fn get_adb_version(&mut self) -> AdbResult<String> {
        let server = self.get_server()?;
        let version = server.version()?;
        Ok(format!(
            "ADB server version: {}.{}.{}",
            version.major, version.minor, version.revision
        ))
    }
}

impl Default for AdbManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adb_manager_creation() {
        let manager = AdbManager::new();
        assert!(manager.server.is_none());
        assert!(manager.selected_device.is_none());
    }

    #[test]
    fn test_package_filter() {
        let filter = PackageFilter::User;
        assert!(matches!(filter, PackageFilter::User));
    }

    #[test]
    fn test_adb_command_creation() {
        let cmd = AdbCommand::ListDevices;
        assert!(matches!(cmd, AdbCommand::ListDevices));
    }

    #[test]
    fn test_error_display() {
        let error = AdbError::DeviceNotFound;
        assert_eq!(error.to_string(), "Device not found");
    }
}
