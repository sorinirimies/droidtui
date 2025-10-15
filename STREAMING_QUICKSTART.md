# Screen Streaming Quick Start Guide 📺

Get started with DroidTUI's screen streaming feature in 60 seconds!

## Prerequisites

✅ **Before you start:**
- Android device with USB debugging enabled
- Device connected via USB
- ADB installed and in PATH

## Quick Test

```bash
# 1. Verify ADB connection
adb devices

# Should show your device like:
# List of devices attached
# ABC123456789    device
```

## Launch Streaming

```bash
# 1. Start DroidTUI
droidtui

# 2. Navigate with arrow keys to:
#    📺 Screen Stream

# 3. Press Enter to expand options

# 4. Select one of:
#    📺 Start Screen Stream    (standard, ~2 FPS)
#    🔍 High Detail Stream     (slower, more detail)
#    ⚡ Fast Stream            (faster, less detail)

# 5. Press Enter to start streaming!
```

## Controls

Once streaming starts:

```
Space       → Pause/Resume
+  or  =    → Speed up (decrease delay)
-  or  _    → Slow down (increase delay)
q  or  Esc  → Stop and return to menu
```

## What You'll See

```
┌──────────────── 📺 Screen Stream (ASCII Art) ────────────────┐
│ ┌───────────── Frame #42 ─────────────┐                      │
│ │                                      │                      │
│ │     Your device screen               │                      │
│ │     rendered as ASCII art            │                      │
│ │     in real-time!                    │                      │
│ │                                      │                      │
│ └──────────────────────────────────────┘                      │
│ ┌───────────── ⌨️  Controls ───────────────┐                 │
│ │ ▶️  STREAMING | Rate: 500ms (~2 FPS) | Frame: #42         │
│ │                                                             │
│ │ [Space] Pause  [+/-] Speed  [Q] Exit                       │
│ └─────────────────────────────────────────────────────────────┘
└───────────────────────────────────────────────────────────────┘
```

## Tips

### Better Quality
- Maximize your terminal window
- Use "High Detail Stream" mode
- Reduce refresh rate with `-` key

### Better Performance
- Use "Fast Stream" mode
- Keep terminal window smaller
- Close other ADB applications

### Troubleshooting

**No devices found?**
```bash
# Enable USB debugging on device:
# Settings → Developer Options → USB Debugging
adb devices
```

**Screen not updating?**
- Press Space (might be paused)
- Check device isn't locked
- Try unplugging and reconnecting

**Garbled display?**
- Resize terminal window
- Use a modern terminal emulator
- Check Unicode support

## Examples

### Monitor App Launch
```
1. Start streaming
2. Launch your app on device
3. Watch it appear in terminal!
```

### Check UI Layout
```
1. Use "High Detail Stream"
2. Press Space to pause
3. Examine layout carefully
4. Press Space to resume
```

### Quick Device Check
```
1. Use "Fast Stream"
2. Verify screen is responsive
3. Press Q to exit quickly
```

## Comparison

| Mode | Speed | Detail | Use Case |
|------|-------|--------|----------|
| Standard | ~2 FPS | Medium | General monitoring |
| High Detail | ~1 FPS | High | Careful inspection |
| Fast | ~5 FPS | Lower | Quick checks |

## Next Steps

- Read [STREAMING.md](STREAMING.md) for full documentation
- Explore other DroidTUI features in [FEATURES.md](FEATURES.md)
- Check out [README.md](README.md) for all ADB commands

## Fun Fact

The ASCII art rendering gives your Android screen a retro terminal aesthetic while still being functional for monitoring! 🎨

---

**Enjoy streaming your Android device in the terminal! 🚀**