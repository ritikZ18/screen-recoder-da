# Screen Recorder - Startup Guide

## Quick Start

### One-Command Setup (Recommended)

**Linux/macOS:**
```bash
cd screen-recorder
./setup.sh
```

**Windows:**
```cmd
cd screen-recorder
setup.bat
```

Or with PowerShell:
```powershell
cd screen-recorder
.\setup.ps1
```

This single command will:
1. ✅ Check all dependencies
2. ✅ Install missing dependencies automatically
3. ✅ Install npm packages
4. ✅ Start the development server

### Manual Setup (Alternative)

If you prefer to set up manually:

#### 1. Check Dependencies

**Linux/macOS:**
```bash
cd screen-recorder
./check-dependencies.sh
```

**Windows:**
```cmd
cd screen-recorder
check-dependencies.bat
```

Or with PowerShell:
```powershell
cd screen-recorder
.\check-dependencies.ps1
```

#### 2. Install npm Dependencies

```bash
npm install
```

#### 3. Start Development

```bash
npm run tauri dev
```

The application will:
- Start the development server
- Compile the Rust backend
- Open the application window

## First-Time Setup

### Windows

1. **Install Visual Studio Build Tools**
   - Download from: https://visualstudio.microsoft.com/downloads/
   - Select "Desktop development with C++" workload

2. **Grant Screen Recording Permission**
   - Open Windows Settings
   - Go to Privacy → Screen Recording
   - Enable "Allow apps to access your screen"

3. **Verify Installation**
   ```bash
   npm run tauri dev
   ```

### Linux

1. **Install System Dependencies**
   ```bash
   sudo apt-get update
   sudo apt-get install -y \
       libwebkit2gtk-4.1-dev \
       build-essential \
       curl \
       wget \
       libssl-dev \
       libgtk-3-dev \
       libayatana-appindicator3-dev \
       librsvg2-dev
   ```

2. **Install Node.js**
   ```bash
   curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
   sudo apt-get install -y nodejs
   ```

3. **Verify Installation**
   ```bash
   npm run tauri dev
   ```

### macOS

1. **Install Xcode Command Line Tools**
   ```bash
   xcode-select --install
   ```

2. **Install Homebrew** (if not installed)
   ```bash
   /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
   ```

3. **Install Node.js**
   ```bash
   brew install node@18
   ```

4. **Grant Screen Recording Permission**
   - Open System Preferences → Security & Privacy → Privacy
   - Select "Screen Recording"
   - Enable the application

5. **Verify Installation**
   ```bash
   npm run tauri dev
   ```

## Troubleshooting Startup Issues

### Issue: "Command not found: tauri"

**Solution**:
```bash
npm install -g @tauri-apps/cli@next
```

### Issue: "Failed to compile Rust code"

**Solution**:
```bash
# Update Rust toolchain
rustup update

# Clean and rebuild
cd src-tauri
cargo clean
cargo build
```

### Issue: "Port 1420 already in use"

**Solution**:
```bash
# Find process using port 1420
# Windows: netstat -ano | findstr :1420
# Linux/macOS: lsof -i :1420

# Kill the process or change port in vite.config.ts
```

### Issue: "Missing system dependencies"

**Solution**:
- Windows: Install Visual Studio Build Tools
- Linux: Install webkit2gtk and related packages
- macOS: Install Xcode Command Line Tools

## Verification Checklist

After startup, verify:

- [ ] Application window opens
- [ ] UI displays correctly
- [ ] Device picker shows monitors/windows
- [ ] No console errors
- [ ] Recording controls are functional
- [ ] Metrics panel displays (may show zeros initially)

## Next Steps

1. **Test Recording**: Select a monitor and start a test recording
2. **Check Output**: Verify recordings are saved to output directory
3. **Review Logs**: Check console for any warnings or errors
4. **Monitor Performance**: Watch CPU and memory usage

## Production Build

Once verified, build for production:

```bash
npm run tauri build
```

Output will be in `src-tauri/target/release/bundle/`

---

For detailed information, see:
- `TECHNICAL_DOCUMENTATION.md` - Complete technical reference
- `README.md` - Project overview
- `QUICKSTART.md` - Quick reference guide

