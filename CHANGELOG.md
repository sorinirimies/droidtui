# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2024-12-XX

### Added
- **TachyonFX Integration**: Added `tachyonfx = "0.7.0"` dependency for stunning visual effects
- **Dramatic Startup Animation**: Implemented reveal animation with:
  - Center-out gradient sweep effects
  - Dynamic wave animations with ripple patterns
  - Progressive content loading phases
  - Smooth fade-in transitions and alpha blending
- **12 Functional ADB Commands** with real execution capabilities:
  - 📱 List Devices (`adb devices -l`) - Shows all connected devices with detailed info
  - 📋 List Packages (`adb shell pm list packages -f`) - Lists installed packages with file paths
  - 🔋 Battery Info (`adb shell dumpsys battery`) - Displays detailed battery information
  - 💾 Memory Usage (`adb shell dumpsys meminfo`) - Shows memory usage statistics
  - 📊 CPU Info (`adb shell cat /proc/cpuinfo`) - Displays CPU specifications
  - 🔗 Network Info (`adb shell dumpsys connectivity`) - Shows network configuration
  - 📱 Device Properties (`adb shell getprop`) - Gets all system properties
  - 🎯 Running Processes (`adb shell ps -A`) - Lists all running processes
  - 📊 System Services (`adb shell service list`) - Lists system services status
  - 🔄 Reboot Device (`adb reboot`) - Reboots the connected device
  - 📜 System Log (`adb logcat -d -t 100`) - Views recent system logs
  - 🔍 ADB Version (`adb version`) - Displays ADB version information
- **Enhanced Command Execution System**:
  - Real-time ADB command execution with proper error handling
  - Comprehensive output parsing for stdout/stderr
  - Intelligent error messages for common issues (ADB not found, permissions, etc.)
  - Command result display with formatted output
- **Dynamic Visual Effects**:
  - Animated menu selection with color cycling
  - Background gradient animations with wave effects
  - Pulsing text effects when startup is complete
  - Selection highlighting with smooth color transitions
- **Improved UI States**:  
  - Startup screen with reveal animation
  - Menu navigation with visual feedback
  - Command execution feedback
  - Result display with formatted output
- **Utility Scripts**:
  - `run.sh` - Build and run script with prerequisite checking
  - `demo.sh` - Interactive demo script with feature showcase

### Changed
- **Removed Android Logo**: Replaced static Android ASCII art with dynamic TachyonFX effects
- **Enhanced Menu System**: 
  - Updated all menu items to use functional, complete ADB commands
  - Added comprehensive descriptions with usage information
  - Improved menu layout with better visual hierarchy
- **Startup Experience**: Completely redesigned startup flow with progressive reveal animation
- **Color Scheme**: Enhanced with dynamic color effects and better contrast
- **Command Reliability**: All commands now use complete, tested ADB syntax

### Fixed
- **ADB Command Functionality**: All menu commands now execute properly with real output
- **Error Handling**: Improved error messages and command failure detection
- **UI Responsiveness**: Better handling of different terminal sizes and content
- **Memory Management**: Optimized effects system for better performance

### Technical Details
- **Architecture**: Maintained clean separation between effects, menu, and application logic
- **Performance**: Efficient tick-based animation system with minimal CPU usage
- **Dependencies**: Successfully integrated TachyonFX while maintaining compatibility
- **Code Quality**: Enhanced error handling and user feedback throughout the application

### Developer Experience
- **Documentation**: Updated README with comprehensive usage instructions
- **Build System**: Improved with helper scripts and better error messages
- **Code Structure**: Modular effects system for easy customization
- **Testing**: All ADB commands verified to work with real Android devices

## [0.1.0] - 2024-12-XX

### Added
- Initial release with basic TUI framework
- Simple counter application template
- Event-driven architecture with tokio async runtime
- Basic ratatui integration
- MIT license and initial project structure

---

**Note**: This changelog focuses on the major features and improvements. For detailed technical changes, see the git commit history.