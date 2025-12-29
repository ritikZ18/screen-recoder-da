#!/bin/bash

# Screen Recorder - Dependency Checker and Installer
# This script checks and installs all required dependencies for Linux systems

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Detect Linux distribution
detect_distro() {
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        DISTRO=$ID
        VERSION=$VERSION_ID
    elif [ -f /etc/lsb-release ]; then
        . /etc/lsb-release
        DISTRO=$DISTRIB_ID
    else
        DISTRO="unknown"
    fi
    echo "$DISTRO"
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check Rust installation
check_rust() {
    if command_exists rustc && command_exists cargo; then
        RUST_VERSION=$(rustc --version | cut -d' ' -f2)
        echo -e "${GREEN}✓${NC} Rust installed: $RUST_VERSION"
        return 0
    else
        echo -e "${RED}✗${NC} Rust not installed"
        return 1
    fi
}

# Install Rust
install_rust() {
    echo -e "${YELLOW}Installing Rust...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo -e "${GREEN}✓${NC} Rust installed successfully"
}

# Check Node.js installation
check_nodejs() {
    if command_exists node; then
        NODE_VERSION=$(node --version | cut -d'v' -f2 | cut -d'.' -f1)
        if [ "$NODE_VERSION" -ge 18 ]; then
            echo -e "${GREEN}✓${NC} Node.js installed: $(node --version)"
            return 0
        else
            echo -e "${RED}✗${NC} Node.js version too old: $(node --version) (requires 18+)"
            return 1
        fi
    else
        echo -e "${RED}✗${NC} Node.js not installed"
        return 1
    fi
}

# Install Node.js (Ubuntu/Debian)
install_nodejs_ubuntu() {
    echo -e "${YELLOW}Installing Node.js 18...${NC}"
    curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
    sudo apt-get install -y nodejs
    echo -e "${GREEN}✓${NC} Node.js installed successfully"
}

# Install Node.js (Fedora/RHEL)
install_nodejs_fedora() {
    echo -e "${YELLOW}Installing Node.js 18...${NC}"
    curl -fsSL https://rpm.nodesource.com/setup_18.x | sudo bash -
    sudo dnf install -y nodejs
    echo -e "${GREEN}✓${NC} Node.js installed successfully"
}

# Check system dependencies (Ubuntu/Debian)
check_system_deps_ubuntu() {
    MISSING_DEPS=()
    
    DEPS=(
        "libwebkit2gtk-4.1-dev"
        "build-essential"
        "curl"
        "wget"
        "libssl-dev"
        "libgtk-3-dev"
        "libayatana-appindicator3-dev"
        "librsvg2-dev"
        "pkg-config"
    )
    
    # Check for FFmpeg (optional but recommended)
    if ! pkg-config --exists libavcodec libavformat libavutil 2>/dev/null; then
        echo -e "${YELLOW}⚠${NC} FFmpeg not found (optional - needed for video encoding)"
        echo -e "   Install with: sudo apt-get install -y libavcodec-dev libavformat-dev libavutil-dev"
    else
        echo -e "${GREEN}✓${NC} FFmpeg development libraries found"
    fi
    
    for dep in "${DEPS[@]}"; do
        if ! dpkg -l | grep -q "^ii  $dep "; then
            MISSING_DEPS+=("$dep")
        fi
    done
    
    if [ ${#MISSING_DEPS[@]} -eq 0 ]; then
        echo -e "${GREEN}✓${NC} All system dependencies installed"
        return 0
    else
        echo -e "${RED}✗${NC} Missing dependencies: ${MISSING_DEPS[*]}"
        return 1
    fi
}

# Install system dependencies (Ubuntu/Debian)
install_system_deps_ubuntu() {
    echo -e "${YELLOW}Installing system dependencies...${NC}"
    sudo apt-get update
    sudo apt-get install -y \
        libwebkit2gtk-4.1-dev \
        build-essential \
        curl \
        wget \
        libssl-dev \
        libgtk-3-dev \
        libayatana-appindicator3-dev \
        librsvg2-dev \
        pkg-config
    
    # Install FFmpeg development libraries (optional but recommended)
    echo -e "${YELLOW}Installing FFmpeg development libraries (optional)...${NC}"
    sudo apt-get install -y \
        libavcodec-dev \
        libavformat-dev \
        libavutil-dev \
        libavfilter-dev \
        libavdevice-dev \
        libswscale-dev \
        libswresample-dev || echo -e "${YELLOW}⚠${NC} FFmpeg installation skipped (optional)"
    
    echo -e "${GREEN}✓${NC} System dependencies installed successfully"
}

# Check system dependencies (Fedora/RHEL)
check_system_deps_fedora() {
    MISSING_DEPS=()
    
    DEPS=(
        "webkit2gtk4.1-devel"
        "gcc"
        "gcc-c++"
        "make"
        "curl"
        "wget"
        "openssl-devel"
        "gtk3-devel"
        "libappindicator-gtk3-devel"
        "librsvg2-devel"
        "pkg-config"
    )
    
    # Check for FFmpeg (optional but recommended)
    if ! pkg-config --exists libavcodec libavformat libavutil 2>/dev/null; then
        echo -e "${YELLOW}⚠${NC} FFmpeg not found (optional - needed for video encoding)"
        echo -e "   Install with: sudo dnf install -y ffmpeg-devel"
    else
        echo -e "${GREEN}✓${NC} FFmpeg development libraries found"
    fi
    
    for dep in "${DEPS[@]}"; do
        if ! rpm -q "$dep" >/dev/null 2>&1; then
            MISSING_DEPS+=("$dep")
        fi
    done
    
    if [ ${#MISSING_DEPS[@]} -eq 0 ]; then
        echo -e "${GREEN}✓${NC} All system dependencies installed"
        return 0
    else
        echo -e "${RED}✗${NC} Missing dependencies: ${MISSING_DEPS[*]}"
        return 1
    fi
}

# Install system dependencies (Fedora/RHEL)
install_system_deps_fedora() {
    echo -e "${YELLOW}Installing system dependencies...${NC}"
    sudo dnf install -y \
        webkit2gtk4.1-devel \
        gcc \
        gcc-c++ \
        make \
        curl \
        wget \
        openssl-devel \
        gtk3-devel \
        libappindicator-gtk3-devel \
        librsvg2-devel \
        pkg-config
    
    # Install FFmpeg development libraries (optional but recommended)
    echo -e "${YELLOW}Installing FFmpeg development libraries (optional)...${NC}"
    sudo dnf install -y ffmpeg-devel || echo -e "${YELLOW}⚠${NC} FFmpeg installation skipped (optional)"
    
    echo -e "${GREEN}✓${NC} System dependencies installed successfully"
}

# Main function
main() {
    echo -e "${BLUE}========================================${NC}"
    echo -e "${BLUE}Screen Recorder - Dependency Checker${NC}"
    echo -e "${BLUE}========================================${NC}"
    echo ""
    
    DISTRO=$(detect_distro)
    echo -e "${BLUE}Detected distribution: $DISTRO${NC}"
    echo ""
    
    NEEDS_INSTALL=false
    
    # Check Rust
    if ! check_rust; then
        NEEDS_INSTALL=true
    fi
    
    # Check Node.js
    if ! check_nodejs; then
        NEEDS_INSTALL=true
    fi
    
    # Check system dependencies
    if [[ "$DISTRO" == "ubuntu" ]] || [[ "$DISTRO" == "debian" ]]; then
        if ! check_system_deps_ubuntu; then
            NEEDS_INSTALL=true
        fi
    elif [[ "$DISTRO" == "fedora" ]] || [[ "$DISTRO" == "rhel" ]] || [[ "$DISTRO" == "centos" ]]; then
        if ! check_system_deps_fedora; then
            NEEDS_INSTALL=true
        fi
    else
        echo -e "${YELLOW}⚠${NC} Unknown distribution. Please install dependencies manually."
        echo -e "   See dependencies.txt for requirements."
    fi
    
    echo ""
    
    if [ "$NEEDS_INSTALL" = true ]; then
        echo -e "${YELLOW}Some dependencies are missing.${NC}"
        read -p "Do you want to install them now? (y/n) " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            echo ""
            
            # Install Rust
            if ! check_rust; then
                install_rust
                source "$HOME/.cargo/env"
            fi
            
            # Install Node.js
            if ! check_nodejs; then
                if [[ "$DISTRO" == "ubuntu" ]] || [[ "$DISTRO" == "debian" ]]; then
                    install_nodejs_ubuntu
                elif [[ "$DISTRO" == "fedora" ]] || [[ "$DISTRO" == "rhel" ]] || [[ "$DISTRO" == "centos" ]]; then
                    install_nodejs_fedora
                fi
            fi
            
            # Install system dependencies
            if [[ "$DISTRO" == "ubuntu" ]] || [[ "$DISTRO" == "debian" ]]; then
                if ! check_system_deps_ubuntu; then
                    install_system_deps_ubuntu
                fi
            elif [[ "$DISTRO" == "fedora" ]] || [[ "$DISTRO" == "rhel" ]] || [[ "$DISTRO" == "centos" ]]; then
                if ! check_system_deps_fedora; then
                    install_system_deps_fedora
                fi
            fi
            
            echo ""
            echo -e "${GREEN}========================================${NC}"
            echo -e "${GREEN}All dependencies installed!${NC}"
            echo -e "${GREEN}========================================${NC}"
        else
            echo -e "${YELLOW}Installation cancelled.${NC}"
            exit 1
        fi
    else
        echo -e "${GREEN}========================================${NC}"
        echo -e "${GREEN}All dependencies are installed!${NC}"
        echo -e "${GREEN}========================================${NC}"
    fi
    
    echo ""
    echo -e "${BLUE}You can now run: npm run tauri dev${NC}"
}

# Run main function
main "$@"

