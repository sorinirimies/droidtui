# DroidTUI Features Documentation 🚀

A comprehensive guide to all features and capabilities in DroidTUI - the Terminal User Interface for Android development.

## Core Features Overview

### 📺 Screen Streaming System (NEW!)
- **Real-time Streaming**: View Android device screen in terminal as ASCII art
- **Multiple Quality Modes**: Choose between speed (fast) and detail (HD)
- **Interactive Controls**: Pause/resume, adjust refresh rate on the fly
- **Performance Monitoring**: Frame counter and FPS display
- **Configurable Refresh Rates**: 100ms to 2000ms (0.5-10 FPS)
- **Lightweight Implementation**: Uses standard ADB screencap command

### 🎨 Visual Effects System
- **TachyonFX Integration**: Stunning visual effects powered by the TachyonFX library
- **Startup Animation**: Dramatic reveal animation with center-out gradient sweep
- **Dynamic Color System**: Position-based color changes with flash effects on selection
- **Animated Backgrounds**: Wave patterns and gradient effects throughout the interface
- **Smooth Transitions**: All UI state changes include smooth visual transitions

### 📱 ADB Command System
- **14 Command Categories**: Comprehensive coverage of Android development tasks
- **Real Command Execution**: Actual ADB commands with formatted output display
- **Error Handling**: Intelligent error messages and troubleshooting hints
- **Command Variations**: Each category includes multiple specific options

### 🧭 Navigation System
- **Dual-Level Navigation**: Main menu and expandable child menus
- **Vim-Style Keys**: Support for j/k navigation alongside arrow keys
- **Visual Indicators**: Clear indicators for expandable items (▶ symbol)
- **Contextual Help**: Dynamic help text that changes based on current mode

## Detailed Feature Breakdown

### Visual Effects & Animation

#### 1. Startup Animation
```
Features:
- Center-out gradient sweep with wave effects
- Progressive content loading (dots → title → subtitle → instructions)
- Alpha blending and fade-in transitions
- Distance-based intensity calculations
- Completion detection with smooth transition to main menu

Technical Implementation:
- Tick-based animation system (30 FPS)
- Mathematical wave functions for organic motion
- Color interpolation using RGB calculations
- State-based progress tracking
```

#### 2. Selection Color System
```
Position-Based Colors (8 variants):
0. Pure Green (0, intensity, 0)
1. Lime Green (intensity/5, intensity, intensity/3)
2. Teal Green (0, intensity, intensity/2)
3. Forest Green (intensity/4, intensity-20, 0)
4. Sea Green (0, intensity-10, intensity/4)
5. Spring Green (intensity/3, intensity, intensity/6)
6. Bright Green (0, intensity+20, intensity/8)
7. Dynamic Green (position_based, intensity, 0)

Flash Effects on Position Change:
- Bright Lime: (flash_intensity/2, 255, flash_intensity/3)
- Electric Green: (0, 255, flash_intensity/2)
- Neon Green: (flash_intensity/4, 255, 0)
- Cyan-Green: (0, 255, flash_intensity)
```

#### 3. Dynamic Border Effects
```
Normal State: Cycles through 6 colors every 20 ticks
- Cyan → LightCyan → Green → LightGreen → Blue → LightBlue

Boost State: Fast flashing every 3 ticks
- LightGreen → Green → RGB(0,255,100) → Cyan
```

### ADB Command Categories

#### 1. 📱 Device Management
```
List Devices:
├── 📋 Basic device list (adb devices)
├── 📝 Detailed info (adb devices -l)
└── 🔍 Serial numbers only (filtered output)

Device Properties:
├── 📋 All properties (adb shell getprop)
├── 🏷️ Device model/brand (filtered getprop)
└── 🔢 Android version (filtered getprop)

Reboot Options:
├── 🔄 Normal reboot (adb reboot)
├── ⚡ Bootloader (adb reboot bootloader)
└── 🔧 Recovery mode (adb reboot recovery)
```

#### 2. 📦 Application Management
```
Package Management:
├── 📦 All packages (pm list packages)
├── 📁 With file paths (pm list packages -f)
├── 👤 User packages only (pm list packages -3)
└── ⚙️ System packages only (pm list packages -s)

Process Management:
├── 📋 All processes (ps -A)
├── 🔝 By CPU usage (top -n 1)
└── 👤 User processes (filtered ps)

System Services:
├── 📋 All services (service list)
├── 🔧 Running services (dumpsys activity services)
└── 📱 App services (filtered services)
```

#### 3. 🔍 System Monitoring
```
Battery Information:
├── 🔋 Full status (dumpsys battery)
├── ⚡ Battery level (filtered battery)
└── 🔌 Charging status (filtered battery)

Memory Usage:
├── 📊 System memory (dumpsys meminfo)
├── 📱 Available memory (/proc/meminfo)
└── 🔝 Top memory apps (filtered meminfo)

CPU Information:
├── 🔧 CPU details (/proc/cpuinfo)
├── ⚡ Current usage (top -n 1)
└── 📈 Load average (/proc/loadavg)

Network Information:
├── 🌐 Connectivity (dumpsys connectivity)
├── 📶 WiFi details (dumpsys wifi)
└── 🔗 IP configuration (ip addr show)
```

#### 4. 📺 Screen Streaming
```
Screen Streaming:
├── 📺 Start Screen Stream (standard, ~2 FPS)
├── 🔍 High Detail Stream (detailed, ~1 FPS)
└── ⚡ Fast Stream (fast, ~5 FPS)

Controls During Streaming:
- Space: Pause/Resume
- +/-: Adjust refresh rate
- q/Esc: Stop streaming

Features:
- Real-time ASCII art rendering
- Frame counter and FPS display
- Adjustable quality and speed
- Minimal device impact
```

#### 5. 📷 Media & Logging
```
Screenshot Capabilities:
├── 📸 Take & save locally (screencap + pull)
├── 📁 Save to device only (screencap with timestamp)
└── 🖼️ View screenshot paths (ls screenshots)

System Logging:
├── 📜 Recent logs (logcat -d -t 100)
├── 🚨 Errors only (logcat -d *:E)
├── ⚠️ Warnings & errors (logcat -d *:W)
└── 🔄 Clear logs (logcat -c)

ADB Utilities:
├── 🔍 ADB version (adb version)
├── 🔧 ADB help (adb help)
└── 📍 Installation path (which adb)
```

### Screen Streaming System

#### 1. Streaming Modes
```
Standard Mode:
- Refresh Rate: 500ms (~2 FPS)
- Resolution: 120x50 characters
- Quality: Balanced
- Use Case: General monitoring

High Detail Mode:
- Refresh Rate: 1000ms (~1 FPS)
- Resolution: 120x50 characters
- Quality: Maximum detail
- Use Case: Detailed screen inspection

Fast Mode:
- Refresh Rate: 200ms (~5 FPS)
- Resolution: 120x50 characters
- Quality: Lower detail, faster updates
- Use Case: Real-time monitoring
```

#### 2. Technical Implementation
```
Process Flow:
1. Capture: adb exec-out screencap -p
2. Decode: PNG image decoding
3. Resize: Scale to terminal dimensions
4. Convert: Grayscale to ASCII characters
5. Display: Render in TUI

ASCII Conversion:
- Brightness-based character mapping
- Palette: " .:-=+*#%@" (dark to bright)
- Grayscale conversion for simplicity
- Optimized for terminal readability
```

#### 3. Performance Characteristics
```
Latency:
- Capture: ~100-200ms
- Processing: ~50-100ms
- Display: ~10-20ms
- Total: ~200-500ms per frame

Bandwidth:
- Screenshot size: ~100-500KB per frame
- Network impact: Minimal (local USB/WiFi)
- CPU usage: Moderate (image processing)

Optimization:
- Adjustable refresh rates
- Pause functionality to reduce load
- Efficient ASCII conversion algorithm
- Minimal memory allocation
```

#### 4. Use Cases
```
✅ Good For:
- Quick device monitoring
- Terminal-only environments (SSH)
- Checking app layouts
- Monitoring background processes
- Logging/automation workflows
- Development on remote servers

❌ Not Ideal For:
- High-precision UI testing
- Gaming or video playback
- Touch input simulation
- Real-time interaction
- Color-critical applications
```

### Navigation System

#### 1. Main Menu Navigation
```
Controls:
- ↑/↓ or j/k: Move between categories
- Enter or →: Enter child menu (if available)
- q/Esc: Quit application
- Ctrl+C: Force quit

Visual Feedback:
- Selected item: Dynamic background color
- Expandable items: ▶ indicator
- Border: Color changes with selection
- Description: Updates with selection
```

#### 2. Child Menu Navigation
```
Layout Changes:
- Left panel (40%): Category overview
- Right panel (60%): Child options
- Selected category: Highlighted in yellow

Controls:
- ↑/↓ or j/k: Navigate child options
- Enter: Execute selected command
- ← or Backspace: Return to main menu
- q/Esc: Quit application

Visual Feedback:
- Child selection: Same color system as main menu
- Parent context: Dimmed but visible
- Dynamic borders: Respond to child selection
```

### Command Execution System

#### 1. Execution Flow
```
State Transitions:
Menu → Executing → ShowResult → Menu

During Execution:
- Visual feedback with "Executing..." dialog
- Command display showing exact command being run
- Escape option to return to menu early
```

#### 2. Output Handling
```
Success Cases:
- stdout available: Display formatted output
- stderr with success: Show stderr (some commands use stderr for output)
- No output: "Command executed successfully" message

Error Cases:
- Command not found: Specific error for ADB vs other commands
- Permission denied: Clear permission error message
- Exit code failure: Show stderr with exit code
- Execution failure: IO error details
```

#### 3. Result Display
```
Output Formatting:
- Scrollable text area (80% width, 70% height)
- Proper line wrapping
- Color-coded titles (green for success, red for errors)
- Clear return instructions

Navigation:
- Any key: Return to menu
- Maintains navigation state
- Clears result when returning
```

### Performance & Technical Details

#### 1. Animation System
```
Frame Rate: 30 FPS (33ms per tick)
CPU Usage: Minimal - simple arithmetic only
Memory: No heap allocation during animations
Responsiveness: Immediate visual feedback on input
```

#### 2. Color Calculations
```
Mathematical Functions:
- Sine waves for pulsing effects
- Distance calculations for gradients
- Linear interpolation for smooth transitions
- Modulo operations for cycling effects
```

#### 3. State Management
```
Application States:
- Startup: Reveal animation active
- Menu: Normal navigation mode
- Executing: Command running
- ShowResult: Display command output

Menu States:
- Main menu mode: Category selection
- Child menu mode: Sub-option selection
- Position tracking: Current and previous selections
- Boost system: Flash effects on position change
```

### Customization Options

#### 1. Color Themes
```
Modify in effects.rs:
- Selection colors: Change RGB values in get_selection_color()
- Border colors: Update get_menu_border_color() arrays
- Flash effects: Modify boost color calculations
- Background gradients: Adjust wave intensity and colors
```

#### 2. Animation Timing
```
Adjustable Parameters:
- tick_count divisors: Control animation speed
- boost duration: Flash effect length (default: 50 ticks)
- wave frequency: Background animation speed
- color cycle rate: How fast colors change
```

#### 3. Menu Structure
```
Adding Commands:
1. Create MenuItem with label, description, command
2. Add MenuChild items for sub-options
3. Commands execute exactly as written
4. Support for shell pipes and complex commands

Example Complex Command:
"adb shell dumpsys battery | grep -E 'level|status'"
```

### Accessibility Features

#### 1. Visual Indicators
```
Clear Navigation:
- ▶ symbols for expandable items
- Color changes for selection feedback
- Contextual help text
- Progress indication during execution
```

#### 2. Keyboard Support
```
Multiple Key Options:
- Arrow keys and vim-style (j/k/h/l)
- Enter and right arrow for forward navigation
- Backspace and left arrow for back navigation
- Multiple quit options (q, Esc, Ctrl+C)
```

#### 3. Error Prevention
```
User Guidance:
- Clear command descriptions
- Expected output explanations
- Error troubleshooting hints
- Safe command defaults (read-only when possible)
```

### Future Enhancement Possibilities

#### 1. Advanced Features
```
Potential Additions:
- Command favorites/bookmarks
- Custom command creation wizard
- Output export to files
- Command history
- Multiple device support
- Batch command execution
```

#### 2. Visual Enhancements
```
Possible Improvements:
- Additional color themes
- User-configurable animations
- Custom ASCII art
- Progress bars for long commands
- Syntax highlighting for output
```

#### 3. Functionality Extensions
```
Development Ideas:
- APK installation wizard
- Log filtering and search
- Device file browser
- Performance monitoring dashboard
- Network analysis tools
```

#### 4. Streaming Enhancements
```
Future Improvements:
- Color ASCII art using ANSI colors
- Sixel/Kitty graphics protocol support
- Higher frame rates with optimization
- Recording streams to file
- Multiple device streams simultaneously
- Region-of-interest streaming
- Touch input forwarding
- Hardware acceleration
- Adaptive quality based on terminal size
- Network streaming over TCP/IP
```

### Screen Streaming Comparison

#### vs. Scrcpy
```
DroidTUI Streaming:
+ Terminal-based, works over SSH
+ No GUI dependencies
+ Integrated with ADB commands
+ Lightweight installation
- Lower frame rate (~1-5 FPS)
- ASCII art only (no true video)
- View-only (no control)
- Higher latency

Scrcpy:
+ High frame rate (30-120 FPS)
+ True video quality
+ Full device control
+ Low latency (~35-70ms)
- Requires GUI environment
- Separate application
- More complex setup
```

#### Recommended Usage
```
Use DroidTUI Streaming When:
- Working in terminal-only environment
- Quick device screen checks needed
- Monitoring background processes
- Logging/automation workflows
- Remote server development
- Terminal aesthetics preferred

Use Scrcpy When:
- Full device control needed
- High frame rate required
- Video playback monitoring
- UI testing and debugging
- Gaming or animations
- Precision touch input needed
```

---

**Note**: This documentation covers all current features including the new Screen Streaming feature as of version 0.2.9. Features are actively developed and may be expanded in future versions.