# 🪟 Quick Start: Build Windows Version

## You're in WSL, but need Windows .exe?

**You built on Linux (WSL), so you got Linux bundles. To get Windows .exe, build on Windows!**

## 🚀 Fastest Way (3 Steps)

### Step 1: Open Windows PowerShell
- Press `Win + X` → Select "Windows PowerShell" or "Terminal"
- **NOT WSL** - use native Windows terminal

### Step 2: Navigate to Your Project
```powershell
# Access WSL files from Windows:
cd \\wsl$\Ubuntu\home\swamizero\screen-recorder

# OR copy to Windows first (recommended):
# xcopy /E /I \\wsl$\Ubuntu\home\swamizero\screen-recorder C:\screen-recorder
# cd C:\screen-recorder
```

### Step 3: Build
```powershell
# Install dependencies (first time only)
npm install

# Build Windows version
npm run tauri build
```

## 📍 Where to Find Your App

After build completes:

**Executable (click to run):**
```
src-tauri\target\release\screen-recorder.exe
```

**Installer (double-click to install):**
```
src-tauri\target\release\bundle\msi\Screen Recorder_1.0.0_x64_en-US.msi
```

## ⚡ Even Faster: Use the Script

1. Copy `build-windows.ps1` to Windows
2. Right-click → "Run with PowerShell"
3. Done!

## ❓ Prerequisites (if missing)

### Node.js
- Download: https://nodejs.org/
- Verify: `node --version`

### Rust
- Download rustup: https://rustup.rs/
- Or: `winget install Rustlang.Rustup`
- Verify: `rustc --version`

### Visual C++ Build Tools
- Download: https://visualstudio.microsoft.com/visual-cpp-build-tools/
- Or install Visual Studio with C++ workload

## 🎯 What You'll Get

- ✅ `screen-recorder.exe` - Double-click to run
- ✅ `.msi` installer - Install like any Windows app
- ✅ App appears in Start Menu after install

## 💡 Pro Tip

After building, you can:
1. **Run directly**: Double-click `screen-recorder.exe`
2. **Install**: Double-click the `.msi` file
3. **Share**: Copy the `.exe` or `.msi` to share with others

---

**Need help?** Check `BUILD_WINDOWS.md` for detailed instructions.

