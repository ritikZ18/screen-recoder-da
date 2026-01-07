# Dependency Management Guide

## Overview

The screen-recorder project includes automated dependency checking and installation scripts for both Windows and Linux systems. These scripts ensure all required dependencies are installed before starting the application.

## Quick Start

### One-Command Setup

**Linux/macOS:**
```bash
./setup.sh
```

**Windows:**
```cmd
setup.bat
```

This single command will:
1. Check all dependencies
2. Install missing dependencies
3. Install npm packages
4. Start the development server

## Dependency Checker Scripts

### Linux/macOS: `check-dependencies.sh`

**Features:**
- Detects Linux distribution (Ubuntu/Debian, Fedora/RHEL)
- Checks for Rust installation
- Checks for Node.js 18+
- Checks for system dependencies
- Automatically installs missing dependencies
- Color-coded output for easy reading

**Usage:**
```bash
./check-dependencies.sh
```

**What it checks:**
- Rust toolchain (1.70+)
- Node.js (18+)
- System packages:
  - Ubuntu/Debian: libwebkit2gtk-4.1-dev, build-essential, etc.
  - Fedora/RHEL: webkit2gtk4.1-devel, gcc, etc.

### Windows: `check-dependencies.ps1` / `check-dependencies.bat`

**Features:**
- Checks for Rust installation
- Checks for Node.js 18+
- Checks for Visual Studio Build Tools
- Automatically installs missing dependencies
- Color-coded PowerShell output

**Usage:**
```powershell
.\check-dependencies.ps1
```

Or with batch file:
```cmd
check-dependencies.bat
```

**What it checks:**
- Rust toolchain (1.70+)
- Node.js (18+)
- Visual Studio Build Tools (with C++ workload)

## Dependencies File

### `dependencies.txt`

This file lists all required dependencies for reference:

- **Core Requirements**: Rust, Node.js, npm
- **Windows Requirements**: Visual Studio Build Tools
- **Linux Requirements**: System packages (distribution-specific)
- **macOS Requirements**: Xcode Command Line Tools

## Manual Dependency Installation

If you prefer to install dependencies manually:

### Linux (Ubuntu/Debian)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js 18
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs

# Install system dependencies
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
```

### Linux (Fedora/RHEL)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js 18
curl -fsSL https://rpm.nodesource.com/setup_18.x | sudo bash -
sudo dnf install -y nodejs

# Install system dependencies
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
```

### Windows

1. **Install Rust:**
   - Download from: https://rustup.rs/
   - Or run: `Invoke-WebRequest https://win.rustup.rs/x86_64 -OutFile rustup-init.exe; .\rustup-init.exe`

2. **Install Node.js:**
   - Download from: https://nodejs.org/ (version 18+)
   - Or use Chocolatey: `choco install nodejs`

3. **Install Visual Studio Build Tools:**
   - Download from: https://visualstudio.microsoft.com/downloads/
   - Select "Build Tools for Visual Studio"
   - Install "Desktop development with C++" workload

### macOS

```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js (via Homebrew)
brew install node@18
```

## Verification

After installation, verify all dependencies:

### Check Rust
```bash
rustc --version  # Should be 1.70+
cargo --version
```

### Check Node.js
```bash
node --version   # Should be 18+
npm --version
```

### Check System Dependencies (Linux)
```bash
# Ubuntu/Debian
dpkg -l | grep -E "libwebkit2gtk|build-essential|libssl-dev"

# Fedora/RHEL
rpm -qa | grep -E "webkit2gtk|gcc|openssl"
```

## Troubleshooting

### Script Fails to Run

**Linux/macOS:**
```bash
# Make script executable
chmod +x check-dependencies.sh setup.sh
```

**Windows:**
```powershell
# Enable script execution (run as Administrator)
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

### Dependencies Not Found After Installation

**Solution:** Restart your terminal/PowerShell to refresh PATH environment variable.

**Linux/macOS:**
```bash
source ~/.cargo/env
```

**Windows:**
- Close and reopen PowerShell/Command Prompt
- Or run: `refreshenv` (if Chocolatey is installed)

### Permission Denied (Linux)

**Solution:** Use `sudo` for system package installation:
```bash
sudo ./check-dependencies.sh
```

### Visual Studio Build Tools Not Detected (Windows)

**Solution:** 
1. Verify installation path
2. Restart computer
3. Reinstall if necessary

## Customization

### Skip Automatic Installation

Edit the scripts to remove automatic installation prompts and only check dependencies.

### Add Additional Dependencies

1. Add to `dependencies.txt`
2. Update check functions in scripts
3. Add installation functions if needed

## Integration with CI/CD

The dependency checker scripts can be used in CI/CD pipelines:

```yaml
# Example GitHub Actions
- name: Check Dependencies
  run: ./check-dependencies.sh
  
- name: Install Dependencies
  run: |
    ./check-dependencies.sh <<< "y"
```

## Support

For issues with dependency installation:
1. Check `dependencies.txt` for requirements
2. Review script output for specific errors
3. Verify system compatibility
4. See `TROUBLESHOOTING.md` for common issues

---

**Last Updated**: 2024-12-28

