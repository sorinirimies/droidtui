#!/bin/bash

# DroidTUI - Android Development TUI Launcher
# This script builds and runs the DroidTUI application

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}🤖 DroidTUI - Android Development TUI${NC}"
echo -e "${BLUE}======================================${NC}"
echo

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Error: Rust/Cargo is not installed${NC}"
    echo -e "${YELLOW}Please install Rust from: https://rustup.rs/${NC}"
    exit 1
fi

# Check if ADB is available
if ! command -v adb &> /dev/null; then
    echo -e "${YELLOW}⚠️  Warning: ADB is not in PATH${NC}"
    echo -e "${YELLOW}   Some features may not work without Android SDK tools${NC}"
    echo
fi

# Build the application
echo -e "${BLUE}🔨 Building DroidTUI...${NC}"
if cargo build --release; then
    echo -e "${GREEN}✅ Build successful!${NC}"
    echo
else
    echo -e "${RED}❌ Build failed!${NC}"
    exit 1
fi

# Run the application
echo -e "${BLUE}🚀 Starting DroidTUI...${NC}"
echo -e "${YELLOW}   Press Ctrl+C to exit${NC}"
echo

# Execute the built binary
exec ./target/release/droidtui
