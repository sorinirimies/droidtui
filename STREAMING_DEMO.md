# Screen Streaming Demo 📺

Visual guide to DroidTUI's screen streaming feature.

## Demo Flow

### 1. Starting DroidTUI

```
┌────────────────────────────────────────────────────────────┐
│  🚀 DroidTUI - Android Development Toolkit                 │
│  Your powerful ADB command center with visual effects      │
│                                                             │
│  Press any key to continue...                              │
└────────────────────────────────────────────────────────────┘
```

### 2. Main Menu Navigation

```
┌─────────────────────────────── Menu ───────────────────────────────┐
│                                                                      │
│  📱 List Devices                ▶  Description:                     │
│  📋 List Packages               ▶  Show all connected Android       │
│  🔋 Battery Info                ▶  devices with their status        │
│  💾 Memory Usage                ▶                                   │
│  📊 CPU Info                    ▶                                   │
│  🔗 Network Info                ▶                                   │
│  📱 Device Properties           ▶                                   │
│  🎯 Running Processes           ▶                                   │
│  📊 System Services             ▶                                   │
│  📷 Screenshot                  ▶                                   │
│  🔄 Reboot Device               ▶                                   │
│  📜 System Log                  ▶                                   │
│  📺 Screen Stream               ▶  ← Navigate here!                 │
│  🔍 ADB Version                 ▶                                   │
│                                                                      │
│  ↑↓: Navigate  Enter: Select  Q: Quit                              │
└──────────────────────────────────────────────────────────────────────┘
```

### 3. Screen Stream Options

```
┌─────────────────────────────── Menu ───────────────────────────────┐
│                                                                      │
│  📺 Screen Stream                   Stream device screen            │
│     📺 Start Screen Stream          in terminal (ASCII art)         │
│     🔍 High Detail Stream      ←                                    │
│     ⚡ Fast Stream                  Description:                    │
│                                     Stream device screen as         │
│                                     ASCII art                       │
│                                                                      │
│  ← Back  Enter: Execute  Q: Quit                                   │
└──────────────────────────────────────────────────────────────────────┘
```

### 4. Streaming View - Active

```
┌──────────────────── 📺 Screen Stream (ASCII Art) ─────────────────────┐
│ ┌────────────────────────── Frame #156 ────────────────────────────┐  │
│ │                                                                   │  │
│ │   @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@      │  │
│ │   @@                                                        @@     │  │
│ │   @@    ####  ###   ####  ###  ###  #####  ##  ##  ###    @@     │  │
│ │   @@   #     #   #  #     #     #   #      ##  ##   #     @@     │  │
│ │   @@   #     #####  ####  ###   #   ###    ##  ##   #     @@     │  │
│ │   @@   #     #      #       #   #   #      ##  ##   #     @@     │  │
│ │   @@    ####  #     ####  ###   #   #####  ##  ##  ###    @@     │  │
│ │   @@                                                        @@     │  │
│ │   @@                                                        @@     │  │
│ │   @@   ################################################    @@     │  │
│ │   @@   ##                                            ##    @@     │  │
│ │   @@   ##   Username: demo@example.com               ##    @@     │  │
│ │   @@   ##   [              Password              ]   ##    @@     │  │
│ │   @@   ##                                            ##    @@     │  │
│ │   @@   ##         [  Sign In  ]                     ##    @@     │  │
│ │   @@   ##                                            ##    @@     │  │
│ │   @@   ################################################    @@     │  │
│ │   @@                                                        @@     │  │
│ │   @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@      │  │
│ │                                                                   │  │
│ └───────────────────────────────────────────────────────────────────┘  │
│ ┌──────────────────────────── ⌨️  Controls ──────────────────────────┐│
│ │ ▶️  STREAMING | Refresh Rate: 500 ms (~2.0 FPS) | Frame: #156     ││
│ │                                                                     ││
│ │ Controls: [Space] Pause/Resume  [+/-] Adjust Speed  [Q] Exit      ││
│ └─────────────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────────────┘
```

### 5. Streaming View - Paused

```
┌──────────────────── 📺 Screen Stream (ASCII Art) ─────────────────────┐
│ ┌────────────────────────── Frame #156 ────────────────────────────┐  │
│ │                                                                   │  │
│ │                                                                   │  │
│ │                                                                   │  │
│ │                       ⏸️  Stream Paused                          │  │
│ │                                                                   │  │
│ │                   Press Space to resume                          │  │
│ │                                                                   │  │
│ │                                                                   │  │
│ │                                                                   │  │
│ └───────────────────────────────────────────────────────────────────┘  │
│ ┌──────────────────────────── ⌨️  Controls ──────────────────────────┐│
│ │ ⏸️  PAUSED | Refresh Rate: 500 ms (~2.0 FPS) | Frame: #156        ││
│ │                                                                     ││
│ │ Controls: [Space] Pause/Resume  [+/-] Adjust Speed  [Q] Exit      ││
│ └─────────────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────────────┘
```

## Real Device Examples

### Example 1: Home Screen

```
Frame #23 - Standard Mode (500ms, ~2 FPS)

@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@                                                @
@   ##########    ##########    ##########      @
@   ##      ##    ##      ##    ##      ##      @
@   ##  📱  ##    ##  📧  ##    ##  🌐  ##      @
@   ##      ##    ##      ##    ##      ##      @
@   ##########    ##########    ##########      @
@                                                @
@   ##########    ##########    ##########      @
@   ##      ##    ##      ##    ##      ##      @
@   ##  🎵  ##    ##  📷  ##    ##  ⚙️  ##      @
@   ##      ##    ##      ##    ##      ##      @
@   ##########    ##########    ##########      @
@                                                @
@   ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░      @
@                                                @
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
```

### Example 2: Text Content

```
Frame #89 - High Detail Mode (1000ms, ~1 FPS)

████████████████████████████████████████████████
█                                              █
█  ┌──────────────────────────────────────┐   █
█  │  Settings                             │   █
█  ├──────────────────────────────────────┤   █
█  │                                       │   █
█  │  > Network & Internet              →  │   █
█  │                                       │   █
█  │  > Connected Devices               →  │   █
█  │                                       │   █
█  │  > Apps                            →  │   █
█  │                                       │   █
█  │  > Notifications                   →  │   █
█  │                                       │   █
█  │  > Battery                         →  │   █
█  │                                       │   █
█  │  > Storage                         →  │   █
█  │                                       │   │
█  └──────────────────────────────────────┘   █
█                                              █
████████████████████████████████████████████████
```

### Example 3: Loading Animation

```
Frame #42 - Fast Mode (200ms, ~5 FPS)

▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
▓                                            ▓
▓                                            ▓
▓         ████████████████████████           ▓
▓         ██                    ██           ▓
▓         ██    Loading...      ██           ▓
▓         ██                    ██           ▓
▓         ██    ████████        ██           ▓
▓         ██    ██░░░░░░        ██           ▓
▓         ██    ████████        ██           ▓
▓         ██                    ██           ▓
▓         ██       45%          ██           ▓
▓         ██                    ██           ▓
▓         ████████████████████████           ▓
▓                                            ▓
▓                                            ▓
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
```

## Control Examples

### Adjusting Speed

```
Initial: 500ms (~2 FPS)
  ↓ Press '+'
Faster:  400ms (~2.5 FPS)
  ↓ Press '+'
Faster:  300ms (~3.3 FPS)
  ↓ Press '-'
Back to: 400ms (~2.5 FPS)
```

### Pause Workflow

```
1. Streaming → Press 'Space' → Paused
2. Examine frame carefully
3. Press 'Space' → Streaming resumes
4. Continue monitoring
```

## Terminal Size Impact

### Small Terminal (80x24)

```
- Less detail visible
- More compression artifacts
- Faster rendering
- Good for quick checks
```

### Medium Terminal (120x50)

```
- Balanced detail and performance
- Recommended default
- Good for most use cases
- Clear text readability
```

### Large Terminal (160x80)

```
- Maximum detail
- Best for inspection
- Slower rendering
- Shows more screen area
```

## Performance Comparison

### Standard Mode (500ms)
```
Frame time: ~500ms
FPS: ~2
Detail: Medium
CPU: Moderate
Use: General monitoring
```

### High Detail Mode (1000ms)
```
Frame time: ~1000ms
FPS: ~1
Detail: High
CPU: High
Use: Careful inspection
```

### Fast Mode (200ms)
```
Frame time: ~200ms
FPS: ~5
Detail: Lower
CPU: Very High
Use: Real-time monitoring
```

## ASCII Art Quality Examples

### High Contrast Content (Good)
```
Clear text, buttons, icons
Distinct UI elements
Sharp borders
Easy to read
```

### Low Contrast Content (Fair)
```
Gradients become patterns
Photos lose detail
Subtle colors merge
Recognizable but less clear
```

### Moving Content (Challenging)
```
Animations blur together
Fast motion hard to track
Static frames clearer
Pause to examine
```

## Tips for Best Results

### 1. Terminal Setup
```
✓ Use modern terminal emulator
✓ Enable Unicode support
✓ Use monospace font
✓ Maximize window size
✓ Good font rendering
```

### 2. Device Preparation
```
✓ Bright screen brightness
✓ High contrast theme
✓ Disable animations
✓ Lock screen orientation
✓ Close battery saver
```

### 3. Content Types
```
Best:    Text, menus, settings
Good:    Icons, simple graphics
Fair:    Photos, gradients
Poor:    Videos, animations
```

## Troubleshooting Visual Guide

### Problem: Blank Screen
```
□□□□□□□□□□□□□□□□□□□□
□                  □
□  No frame yet    □
□                  □
□□□□□□□□□□□□□□□□□□□□

Solution: Wait, device may be locked
```

### Problem: Error Message
```
┌──────────────────────┐
│ Error capturing      │
│ frame: screencap     │
│ failed               │
│                      │
│ Press 'q' to exit   │
└──────────────────────┘

Solution: Check ADB connection
```

### Problem: Garbled Output
```
█▓▒░█▓▒░█▓▒░█▓▒░█▓▒░
░▒▓█░▒▓█░▒▓█░▒▓█░▒▓█
▒░█▓▒░█▓▒░█▓▒░█▓▒░█▓
█▓▒░█▓▒░█▓▒░█▓▒░█▓▒░

Solution: Resize terminal, check Unicode
```

## Feature Comparison Chart

```
                    DroidTUI    Scrcpy
Display Method:     ASCII       Video
Frame Rate:         1-5 FPS     30-120 FPS
Latency:           200-500ms    35-70ms
Control:           View Only    Full
Environment:       Terminal     GUI
Installation:      Easy         Moderate
Dependencies:      Minimal      Many
```

## Success Indicators

### Streaming Working Well
```
✓ Regular frame updates
✓ Readable content
✓ Responsive controls
✓ Stable frame rate
✓ No error messages
```

### Performance Good
```
✓ Low CPU usage
✓ Smooth updates
✓ Quick captures
✓ No lag or stutter
✓ Consistent timing
```

## Conclusion

The screen streaming feature transforms your terminal into a device monitoring tool, providing a unique way to observe your Android device. While not a replacement for full video mirroring tools, it excels in terminal-only environments and quick device checks.

---

**Happy Streaming! 🚀📺**