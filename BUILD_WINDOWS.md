# Building for Windows

## Quick Start

Since you're in WSL (Linux), you need to build on **Windows** to get a `.exe` file.

## Option 1: Build on Windows (Recommended)

### Prerequisites on Windows

1. **Install Node.js** (if not already installed)
   - Download from: https://nodejs.org/
   - Verify: `node --version` and `npm --version`

2. **Install Rust** (if not already installed)
   ```powershell
   # Download and run rustup-init.exe from:
   # https://rustup.rs/
   # Or use:
   winget install Rustlang.Rustup
   ```

3. **Install Tauri CLI**
   ```powershell
   npm install -g @tauri-apps/cli
   ```

4. **Install Microsoft Visual C++ Build Tools**
   - Download: https://visualstudio.microsoft.com/visual-cpp-build-tools/
   - Or install Visual Studio with C++ workload

### Build Steps

1. **Open PowerShell or Command Prompt on Windows** (not WSL)

2. **Navigate to your project** (access WSL files from Windows)
   ```powershell
   # WSL files are accessible at:
   cd \\wsl$\Ubuntu\home\swamizero\screen-recorder
   
   # Or copy project to Windows:
   # xcopy /E /I \\wsl$\Ubuntu\home\swamizero\screen-recorder C:\screen-recorder
   ```

3. **Install dependencies**
   ```powershell
   npm install
   ```

4. **Build for Windows**
   ```powershell
   npm run tauri build
   ```

5. **Find your executable**
   ```
   src-tauri\target\release\screen-recorder.exe
   ```

6. **Or find the installer**
   ```
   src-tauri\target\release\bundle\msi\Screen Recorder_1.0.0_x64_en-US.msi
   ```

## Option 2: Cross-Compile from WSL (Advanced)

This is more complex and requires Windows toolchain setup. Not recommended unless you're familiar with cross-compilation.

## Running the Windows App

### From File Explorer
1. Navigate to: `src-tauri\target\release\`
2. Double-click `screen-recorder.exe`

### From Start Menu (after MSI install)
1. Install the `.msi` file
2. Search for "Screen Recorder" in Start Menu
3. Click to launch

## Troubleshooting

### "Cannot find node"
- Install Node.js on Windows
- Restart terminal after installation

### "Cannot find rustc"
- Install Rust on Windows using rustup
- Run: `rustup default stable`

### "Failed to build"
- Install Visual C++ Build Tools
- Ensure you're in PowerShell/CMD (not WSL)

### Access WSL files from Windows
- Use: `\\wsl$\Ubuntu\home\swamizero\screen-recorder`
- Or copy project to Windows drive first

