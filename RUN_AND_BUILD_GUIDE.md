# 🚀 Run & Build Guide

## Running the Application (Test Camera/Recording)

### Step 1: Start the Development Server

From your Linux/WSL terminal:

```bash
cd /home/swamizero/screen-recorder
npm run tauri:dev
```

This will:
- ✅ Start the Vite dev server on port 1420
- ✅ Compile the Rust backend
- ✅ Open a desktop window (not browser)
- ✅ Enable Tauri APIs for camera/recording

**Note:** The first build may take a few minutes as it compiles Rust dependencies.

### Step 2: Test Camera Recording

Once the desktop window opens:
1. Select a capture source (monitor, window, or camera)
2. Click "Start Recording" 
3. Test recording functionality
4. Check real-time metrics and timeline

### Troubleshooting Running

- **Port already in use**: The `tauri:dev` script automatically runs `cleanup-port.sh` before starting
- **Rust compilation errors**: Run `./verify_build.sh` to check for issues
- **Dependencies missing**: Run `npm install` first

---

## Building Windows Version

**⚠️ Important:** You're on Linux/WSL, but Windows executables must be built **on Windows**. Cross-compilation from Linux is not supported for Tauri Windows builds.

### Option 1: Build on Windows (Recommended)

#### Prerequisites on Windows

1. **Install Node.js** (if not installed)
   - Download: https://nodejs.org/
   - Verify: `node --version`

2. **Install Rust** (if not installed)
   ```powershell
   # Option 1: Download rustup-init.exe from https://rustup.rs/
   # Option 2: Use winget
   winget install Rustlang.Rustup
   ```
   - Verify: `rustc --version`

3. **Install Visual C++ Build Tools** (Required!)
   - Download: https://visualstudio.microsoft.com/visual-cpp-build-tools/
   - Or install Visual Studio with C++ workload

4. **Install Tauri CLI globally** (optional)
   ```powershell
   npm install -g @tauri-apps/cli
   ```

#### Build Steps on Windows

1. **Open PowerShell or Command Prompt on Windows** (NOT WSL!)

2. **Navigate to your project**

   **Option A: Access WSL files directly from Windows**
   ```powershell
   cd \\wsl$\Ubuntu\home\swamizero\screen-recorder
   ```
   
   **Option B: Copy project to Windows (Recommended for faster builds)**
   ```powershell
   # Copy from WSL to Windows
   xcopy /E /I \\wsl$\Ubuntu\home\swamizero\screen-recorder C:\screen-recorder
   cd C:\screen-recorder
   ```

3. **Install dependencies** (first time only)
   ```powershell
   npm install
   ```

4. **Build for Windows**
   
   **Option A: PowerShell script (Recommended)**
   ```powershell
   .\build-windows.ps1
   ```
   Note: Fixed encoding issues - now uses plain ASCII for compatibility.
   
   **Option B: Batch file (handles UNC paths automatically)**
   ```powershell
   .\build-windows.bat
   ```
   Note: Fixed to handle `\\wsl$\...` UNC paths using `pushd`.
   
   **Option C: Manual build**
   ```powershell
   npm install
   npm run tauri build
   ```

5. **Find your Windows executable**
   - **Executable (direct run):**
     ```
     src-tauri\target\release\screen-recorder.exe
     ```
   - **Installer (MSI - recommended):**
     ```
     src-tauri\target\release\bundle\msi\Screen Recorder_1.0.0_x64_en-US.msi
     ```

### Option 2: Use GitHub Actions (CI/CD)

If you push to GitHub, you can set up GitHub Actions to automatically build Windows, Linux, and macOS versions. This is the best approach for cross-platform builds.

### Running the Windows Build

1. **Double-click** `screen-recorder.exe` to run directly, OR
2. **Double-click** the `.msi` installer to install like any Windows app
3. After MSI install, find "Screen Recorder" in Start Menu

---

## Quick Reference

### Linux/WSL (Development)
```bash
# Run dev version
npm run tauri:dev

# Build Linux version
npm run tauri build
```

### Windows (Production Build)
```powershell
# Access project
cd \\wsl$\Ubuntu\home\swamizero\screen-recorder

# Build (either script works - both are fixed!)
.\build-windows.ps1
# OR
.\build-windows.bat
# OR manually:
npm install
npm run tauri build

# Run the built app
.\src-tauri\target\release\screen-recorder.exe
```

**Fixed Issues:**
- ✅ PowerShell script: Removed emoji characters that caused encoding errors
- ✅ Batch file: Added `pushd`/`popd` to handle UNC paths (`\\wsl$\...`)

---

## Build Output Locations

### Linux Build
- Executable: `src-tauri/target/release/screen-recorder`
- Debian package: `src-tauri/target/release/bundle/deb/screen-recorder_*.deb`

### Windows Build
- Executable: `src-tauri\target\release\screen-recorder.exe`
- MSI installer: `src-tauri\target\release\bundle\msi\Screen Recorder_*.msi`

---

## Need Help?

- **Build errors?** Run `./verify_build.sh` to check setup
- **Windows build failing?** Ensure Visual C++ Build Tools are installed
- **Can't access WSL files?** Copy project to Windows drive first

