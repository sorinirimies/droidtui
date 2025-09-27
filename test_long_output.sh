#!/bin/bash

# Test script to generate long output for testing DroidTUI scrolling functionality

echo "=== DroidTUI Scrolling Test ==="
echo "This script generates long output to test the scrolling functionality."
echo ""

# Generate numbered lines to test scrolling
for i in {1..50}; do
    echo "Line $i: This is a test line to demonstrate scrolling functionality in DroidTUI command results."
done

echo ""
echo "=== System Information (Mock) ==="
echo "Device: Test Android Device"
echo "Android Version: 13 (API 33)"
echo "Build Number: TP1A.220624.014"
echo "Security Patch: 2023-10-05"
echo ""

# Generate some package-like output
echo "=== Installed Packages (Sample) ==="
for i in {1..30}; do
    echo "package:com.example.app$i"
done

echo ""
echo "=== Memory Information ==="
echo "Total RAM: 8192 MB"
echo "Available RAM: 4096 MB"
echo "Used RAM: 4096 MB"
echo "Cached: 2048 MB"
echo "Free: 2048 MB"

echo ""
echo "=== CPU Information ==="
echo "Architecture: arm64-v8a"
echo "CPU Cores: 8"
echo "CPU Frequency: 2.4 GHz"
echo "CPU Usage: 25%"

echo ""
echo "=== Network Information ==="
echo "WiFi: Connected"
echo "IP Address: 192.168.1.100"
echo "Signal Strength: -45 dBm"
echo "Connection Speed: 150 Mbps"

echo ""
echo "=== Battery Information ==="
echo "Battery Level: 85%"
echo "Battery Status: Charging"
echo "Battery Health: Good"
echo "Battery Temperature: 32°C"

echo ""
echo "=== End of Test Output ==="
echo "Total lines: ~100+"
echo "This output should be scrollable in DroidTUI!"
