#!/bin/bash

# DroidTUI Demo Script
# This script demonstrates the features of the DroidTUI application

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m' # No Color

# Function to print section headers
print_header() {
    echo
    echo -e "${CYAN}================================${NC}"
    echo -e "${CYAN}$1${NC}"
    echo -e "${CYAN}================================${NC}"
    echo
}

# Function to print feature descriptions
print_feature() {
    echo -e "${GREEN}✓${NC} $1"
}

# Function to wait for user input
wait_for_user() {
    echo -e "${YELLOW}Press Enter to continue...${NC}"
    read -r
}

clear

# ASCII Art Header
echo -e "${GREEN}"
cat << "EOF"
     ____            _     _ _____ _   _ _____
    |  _ \ _ __ ___  (_) __| |_   _| | | |_   _|
    | | | | '__/ _ \ | |/ _` | | | | | | | | |
    | |_| | | | (_) || | (_| | | | | |_| | | |
    |____/|_|  \___/ |_|\__,_| |_|  \___/  |_|

        Android Development TUI Demo
EOF
echo -e "${NC}"

print_header "🤖 Welcome to DroidTUI Demo"

echo -e "${BLUE}DroidTUI is a beautiful Terminal User Interface for Android development${NC}"
echo -e "${BLUE}that provides an intuitive interface for ADB commands with visual effects.${NC}"
echo
wait_for_user

print_header "✨ Key Features"

print_feature "Beautiful startup animation with Android logo and gradient background"
print_feature "Interactive menu system with ADB command categories"
print_feature "Real-time command execution with result display"
print_feature "Keyboard navigation (vim-style j/k or arrow keys)"
print_feature "Visual effects powered by TachyonFX library"
print_feature "Clean, responsive terminal interface"
print_feature "Error handling with user-friendly messages"

wait_for_user

print_header "📱 Available ADB Commands"

echo -e "${MAGENTA}Device Management:${NC}"
print_feature "List connected devices"
print_feature "Get device information"
print_feature "Reboot device"

echo
echo -e "${MAGENTA}App Management:${NC}"
print_feature "Install APK files"
print_feature "Uninstall applications"
print_feature "List installed packages"

echo
echo -e "${MAGENTA}File Operations:${NC}"
print_feature "Push files to device"
print_feature "Pull files from device"
print_feature "Access device shell"

echo
echo -e "${MAGENTA}Development Tools:${NC}"
print_feature "Take screenshots"
print_feature "Record screen"
print_feature "View system logs (logcat)"

wait_for_user

print_header "⌨️ Navigation Controls"

echo -e "${YELLOW}Menu Navigation:${NC}"
echo "  ↑/↓ or j/k    - Move up/down in menu"
echo "  Enter         - Execute selected command"
echo "  q/Esc         - Quit application"
echo "  Ctrl+C        - Force quit"
echo
echo -e "${YELLOW}During Command Execution:${NC}"
echo "  Any key       - Return to menu after viewing results"
echo "  Esc           - Cancel execution (if supported)"

wait_for_user

print_header "🚀 Demo Time!"

echo -e "${BLUE}Let's start the DroidTUI application...${NC}"
echo -e "${YELLOW}You'll see:${NC}"
echo "1. Startup screen with animated Android logo"
echo "2. Main menu with ADB commands"
echo "3. Interactive navigation"
echo "4. Command execution and results"
echo
echo -e "${GREEN}Try navigating with arrow keys or j/k!${NC}"
echo -e "${GREEN}Select a command with Enter to see it in action!${NC}"
echo

wait_for_user

print_header "🔧 Prerequisites Check"

# Check if cargo is installed
if command -v cargo &> /dev/null; then
    print_feature "Rust/Cargo is installed"
else
    echo -e "${RED}❌ Rust/Cargo not found. Please install from: https://rustup.rs/${NC}"
    exit 1
fi

# Check if adb is available
if command -v adb &> /dev/null; then
    print_feature "ADB is available in PATH"
    echo -e "  ${CYAN}ADB Version: $(adb version | head -1)${NC}"
else
    echo -e "${YELLOW}⚠️  ADB not found in PATH${NC}"
    echo -e "${YELLOW}   Some commands will show connection errors${NC}"
    echo -e "${YELLOW}   Install Android SDK tools for full functionality${NC}"
fi

echo
wait_for_user

print_header "🎬 Starting DroidTUI"

echo -e "${BLUE}Building application...${NC}"
if cargo build --release --quiet; then
    print_feature "Build successful!"
else
    echo -e "${RED}❌ Build failed!${NC}"
    exit 1
fi

echo
echo -e "${GREEN}🚀 Launching DroidTUI...${NC}"
echo -e "${YELLOW}   Use Ctrl+C or 'q' to exit${NC}"
echo -e "${YELLOW}   Explore the menu and try different commands!${NC}"
echo

sleep 2

# Launch the application
./target/release/droidtui

print_header "🎉 Demo Complete"

echo -e "${GREEN}Thank you for trying DroidTUI!${NC}"
echo
echo -e "${BLUE}What's next?${NC}"
echo "• Customize the menu items in src/menu.rs"
echo "• Add your own ADB commands and shortcuts"
echo "• Contribute to the project on GitHub"
echo "• Share feedback and suggestions"
echo
echo -e "${CYAN}Happy Android development! 🤖✨${NC}"
echo
EOF
