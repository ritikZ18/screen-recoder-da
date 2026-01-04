# Quick Fix: Desktop App Not Starting

## Problem

You're seeing:
- ❌ Web app in browser (not desktop window)
- ❌ Error: "Cannot read properties of undefined (reading 'invoke')"
- ❌ Recording buttons don't work

## Solution

### Step 1: Stop the Web Server

If you're running `npm run dev`, stop it (Ctrl+C).

### Step 2: Run the Desktop App

```bash
cd screen-recorder
npm run tauri dev
```

**This is the correct command!** It will:
- ✅ Open a **desktop window** (not browser)
- ✅ Enable Tauri APIs
- ✅ Make recording work

### Step 3: Verify

You should see:
- ✅ A desktop window titled "Screen Recorder"
- ✅ No browser address bar
- ✅ No "invoke is undefined" errors
- ✅ Device picker loads monitors/windows
- ✅ Recording buttons work

## Commands Reference

| Command | What It Does | Use When |
|---------|-------------|----------|
| `npm run dev` | Web app only (browser) | ❌ Don't use - Tauri APIs won't work |
| `npm run tauri dev` | **Desktop app** (development) | ✅ **Use this for development** |
| `npm run tauri build` | Build desktop app (production) | ✅ Use to create installer |

## Still Having Issues?

1. **Check Rust is installed**:
   ```bash
   rustc --version
   ```

2. **Check Tauri CLI**:
   ```bash
   npm list -g @tauri-apps/cli
   ```

3. **Install dependencies**:
   ```bash
   ./check-dependencies.sh  # Linux
   # or
   check-dependencies.bat   # Windows
   ```

4. **Clean and rebuild**:
   ```bash
   cd src-tauri
   cargo clean
   cd ..
   npm run tauri dev
   ```

---

**Remember**: Always use `npm run tauri dev` for the desktop app!

