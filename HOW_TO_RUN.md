# 🚀 How to Run the Screen Recorder

## ⚠️ Important: Use the Correct Command!

**DO NOT** run `npm run dev` - that's just the web frontend without Tauri APIs!

**DO** run:
```bash
npm run tauri:dev
```

This will:
- ✅ Start the Vite dev server (frontend)
- ✅ Compile the Rust backend (Tauri)
- ✅ Open a **desktop window** with full functionality
- ✅ Enable Tauri APIs for recording

---

## Running the App

### Step 1: Start the Desktop App

```bash
cd /home/swamizero/screen-recorder
npm run tauri:dev
```

**First time:** This may take a few minutes as Rust dependencies compile.

### Step 2: What You'll See

- **Windows:** The app will show available monitors and windows to record
- **Linux:** You'll see a message that screen capture isn't implemented yet (Windows support coming soon!)
- **macOS:** Similar message (support planned)

---

## Troubleshooting

### "Running in web mode" Warning

**Problem:** You see: "⚠️ Running in web mode. Tauri APIs are not available."

**Solution:** You're running `npm run dev` instead of `npm run tauri:dev`
- Stop the current server (Ctrl+C)
- Run: `npm run tauri:dev`

### Blank Screen

**If you see a blank screen:**

1. **Check the terminal** for compilation errors
2. **Wait a moment** - first build takes time to compile Rust
3. **Check browser console** (F12) for JavaScript errors
4. **Verify dependencies:**
   ```bash
   npm install
   cd src-tauri && cargo check
   ```

### Port 1420 Already in Use

The `tauri:dev` script automatically cleans up port 1420, but if issues persist:

```bash
npm run cleanup
# Then try again:
npm run tauri:dev
```

---

## Platform Support

### ✅ Windows
- Full screen capture support
- Monitor and window recording
- System audio capture

### 🚧 Linux (WSL)
- App runs and UI works
- Screen capture **not yet implemented**
- Shows helpful message about upcoming support

### 🚧 macOS
- App runs
- Screen capture **not yet implemented**

---

## Development Commands

```bash
# Run desktop app (with Tauri)
npm run tauri:dev

# Build for production
npm run tauri:build

# Clean up port conflicts
npm run cleanup

# Verify build setup
./verify_build.sh
```

---

## What to Expect

When running `npm run tauri:dev`:

1. **Terminal output:** You'll see Rust compilation progress
2. **Desktop window:** A window opens (not a browser tab!)
3. **UI:** Full interface with device picker, controls, timeline
4. **Functionality:** Recording features work on Windows

The app should **never** show a blank screen. If it does, check the terminal for errors!

