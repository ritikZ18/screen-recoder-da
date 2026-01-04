# Desktop App Build & Run Guide

## ⚠️ Important: This is a Desktop App, Not a Web App

The screen recorder is built with **Tauri**, which creates a **desktop application**. You cannot run it as a regular web app in a browser.

## Running the Desktop App

### Development Mode (Desktop App)

```bash
cd screen-recorder

# Install dependencies first (if not done)
npm install

# Run the desktop app in development mode
npm run tauri dev
```

This will:
1. ✅ Start the Vite dev server (frontend)
2. ✅ Compile the Rust backend
3. ✅ Launch a **desktop window** (not a browser)
4. ✅ Enable hot-reload for frontend changes

**Expected Result**: A desktop window opens (not a browser tab)

### Production Build (Desktop App)

```bash
cd screen-recorder

# Build the desktop application
npm run tauri build
```

**Output Location**:
- **Windows**: `src-tauri/target/release/bundle/msi/screen-recorder_1.0.0_x64_en-US.msi`
- **Linux**: `src-tauri/target/release/bundle/deb/screen-recorder_1.0.0_amd64.deb`
- **macOS**: `src-tauri/target/release/bundle/dmg/screen-recorder_1.0.0_x64.dmg`

## What NOT to Do

❌ **Don't run**: `npm run dev` (this is just the web frontend, Tauri APIs won't work)
❌ **Don't open in browser**: The app needs the Tauri runtime to work

## Troubleshooting

### Error: "Cannot read properties of undefined (reading 'invoke')"

**Cause**: You're running `npm run dev` instead of `npm run tauri dev`

**Solution**: 
```bash
npm run tauri dev
```

### Error: "Tauri CLI not found"

**Solution**:
```bash
npm install -g @tauri-apps/cli@next
# Or use npx
npx @tauri-apps/cli@next dev
```

### Error: "Rust not found"

**Solution**: Install Rust first
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Error: "Build failed"

**Solution**: Check dependencies
```bash
# Linux
./check-dependencies.sh

# Windows
check-dependencies.bat
```

## Quick Start (One Command)

```bash
# Linux/macOS
./setup.sh

# Windows
setup.bat
```

This will:
1. Check and install dependencies
2. Install npm packages
3. Start the desktop app

## Running the Built App

After building with `npm run tauri build`:

### Windows
1. Navigate to `src-tauri/target/release/bundle/msi/`
2. Double-click the `.msi` installer
3. Install the app
4. Run from Start Menu

### Linux
1. Navigate to `src-tauri/target/release/bundle/deb/`
2. Install: `sudo dpkg -i screen-recorder_*.deb`
3. Run: `screen-recorder`

### macOS
1. Navigate to `src-tauri/target/release/bundle/dmg/`
2. Open the `.dmg` file
3. Drag the app to Applications
4. Run from Applications

## Development Workflow

1. **Start desktop app**: `npm run tauri dev`
2. **Make changes** to React code in `src/`
3. **Hot-reload** happens automatically
4. **Rust changes** require restart: Stop (Ctrl+C) and run `npm run tauri dev` again

## Architecture

```
┌─────────────────────────────────────┐
│   Desktop Window (Tauri Runtime)   │
│  ┌───────────────────────────────┐  │
│  │  React UI (Frontend)          │  │
│  │  - DevicePicker               │  │
│  │  - RecordingControls          │  │
│  │  - Timeline                   │  │
│  └───────────┬───────────────────┘  │
│              │ IPC (invoke/listen)   │
│  ┌───────────┴───────────────────┐  │
│  │  Rust Backend                 │  │
│  │  - Session Manager            │  │
│  │  - Capture Layer              │  │
│  │  - Encoder                    │  │
│  │  - Analytics                  │  │
│  └───────────────────────────────┘  │
└─────────────────────────────────────┘
```

The React frontend runs **inside** the Tauri desktop window, not in a browser.

## Verification

When running correctly, you should see:
- ✅ A desktop window (not a browser tab)
- ✅ Window title: "Screen Recorder"
- ✅ No browser address bar
- ✅ Tauri APIs work (no "invoke is undefined" errors)

---

**Remember**: Always use `npm run tauri dev` for development, not `npm run dev`!

