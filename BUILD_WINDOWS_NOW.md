# 🪟 Build Windows Version - Step by Step

## Why You Need Windows Build

You're seeing "No monitors found" because you built the **Linux version** (AppImage). Recording features only work on **Windows**. You need to build the **Windows .exe** version.

---

## ✅ Method 1: Build Directly on Windows (Recommended)

### Step 1: Open Windows PowerShell

**IMPORTANT:** You must use **Windows PowerShell** or **Command Prompt**, NOT WSL!

- Press `Win + X` → Select "Windows PowerShell" or "Terminal"
- Or search "PowerShell" in Start Menu

### Step 2: Navigate to Your Project

From Windows PowerShell, access your WSL files:

```powershell
cd \\wsl$\Ubuntu-22.04\home\swamizero\screen-recorder
```

**Alternative (if UNC path doesn't work):**

Copy project to Windows first:
```powershell
xcopy /E /I \\wsl$\Ubuntu-22.04\home\swamizero\screen-recorder C:\screen-recorder
cd C:\screen-recorder
```

### Step 3: Install Prerequisites (If Not Already Installed)

Check if you have everything:

```powershell
# Check Node.js
node --version
# If not found, install from: https://nodejs.org/

# Check Rust
rustc --version
# If not found, run: winget install Rustlang.Rustup

# Check Visual C++ Build Tools
# If missing, download from: https://visualstudio.microsoft.com/visual-cpp-build-tools/
```

### Step 4: Install Dependencies

```powershell
npm install
```

### Step 5: Build Windows Version

**Option A: Use the build script (Easiest)**
```powershell
.\build-windows.ps1
```

**Option B: Build manually**
```powershell
npm run tauri build
```

**Note:** The build will take several minutes (compiling Rust).

### Step 6: Find Your Windows App

After build completes, you'll find:

**Executable (run directly):**
```
src-tauri\target\release\screen-recorder.exe
```

**Installer (recommended):**
```
src-tauri\target\release\bundle\msi\Screen Recorder_1.0.0_x64_en-US.msi
```

### Step 7: Run the Windows App

1. **Double-click** `screen-recorder.exe` to run, OR
2. **Double-click** the `.msi` file to install (then find it in Start Menu)

Now you'll see actual monitors and windows! 🎉

---

## 📋 Prerequisites Checklist

Make sure you have these installed on **Windows** (not WSL):

### ✅ Node.js
- Download: https://nodejs.org/
- Verify: `node --version` shows a version

### ✅ Rust
- Download: https://rustup.rs/ (run rustup-init.exe)
- Or: `winget install Rustlang.Rustup`
- Verify: `rustc --version` shows a version

### ✅ Visual C++ Build Tools (REQUIRED!)
- Download: https://visualstudio.microsoft.com/visual-cpp-build-tools/
- Install with C++ workload
- This is **essential** - builds will fail without it

### ✅ Tauri CLI (optional - can install globally)
```powershell
npm install -g @tauri-apps/cli
```

---

## ⚡ Quick One-Liner

If everything is installed, from Windows PowerShell:

```powershell
cd \\wsl$\Ubuntu-22.04\home\swamizero\screen-recorder && npm install && npm run tauri build
```

---

## 🐛 Troubleshooting

### "Cannot find node"
- Install Node.js on Windows
- Restart PowerShell after installation

### "Cannot find rustc"
- Install Rust on Windows: `winget install Rustlang.Rustup`
- Run: `rustup default stable`

### "Failed to build" / "linker errors"
- **Most common issue!** Install Visual C++ Build Tools
- Download: https://visualstudio.microsoft.com/visual-cpp-build-tools/
- Install with "Desktop development with C++" workload

### "UNC path not supported"
- Use the `build-windows.bat` script (handles UNC paths)
- Or copy project to Windows drive first

### Build is slow
- First build takes 10-20 minutes (compiling Rust)
- Subsequent builds are faster
- This is normal!

---

## 📁 Project Structure After Build

```
screen-recorder/
└── src-tauri/
    └── target/
        └── release/
            ├── screen-recorder.exe          ← Run this!
            └── bundle/
                └── msi/
                    └── Screen Recorder_1.0.0_x64_en-US.msi  ← Or install this!
```

---

## 🎯 What's Different on Windows?

Once you build and run the Windows version:
- ✅ **Monitors will be detected** (shows your actual displays)
- ✅ **Windows will be detected** (shows running applications)
- ✅ **Recording will work!** (can capture screen/windows)
- ✅ **System audio capture** (Windows only feature)

The Linux version you built shows "No monitors found" because screen capture isn't implemented for Linux yet - that's expected and correct behavior.

---

## 💡 Pro Tips

1. **First build takes time** - be patient (10-20 minutes)
2. **Use the .msi installer** - easier to install/uninstall
3. **Copy project to Windows** - faster than accessing WSL files directly
4. **Check build logs** - if it fails, the error message usually tells you what's missing

---

**Ready to build?** Open Windows PowerShell and follow the steps above! 🚀

