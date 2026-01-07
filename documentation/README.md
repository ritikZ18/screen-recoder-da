# Screen Recorder - New User Setup

This guide consolidates the minimum steps a new user needs to start the app, pick a window to observe, and find the generated metadata.

## Quick start (recommended)

These scripts check dependencies, install npm packages, and start the desktop app:

Windows (Command Prompt):
```cmd
cd c:\dev\screen-recorder
setup.bat
```

Windows (PowerShell):
```powershell
cd c:\dev\screen-recorder
.\setup.ps1
```

Linux/macOS:
```bash
cd /path/to/screen-recorder
./setup.sh
```

## Manual start (if dependencies are already installed)

```bash
cd /path/to/screen-recorder
npm install
npm run tauri:dev
```

Important: do not use `npm run dev`. That only starts the web frontend without Tauri APIs.

## WSL note (graphics warnings)

`npm run tauri:dev` already enables software rendering for WSL compatibility. You can also use:
```bash
./scripts/run-tauri-dev.sh
```

## Pick a window and view observability data

1. Start the app with `npm run tauri:dev`.
2. Use the device picker to choose a monitor or a specific window.
3. Start recording to generate timeline analytics.
4. Watch the metrics panel for FPS, encode latency, CPU usage, and memory usage.

## Output files and locations

The app writes outputs into your Videos folder (or your home folder if Videos is unavailable):

- Output directory: `Videos\ScreenRecordings` on Windows, or `~/ScreenRecordings` as fallback.
- File pattern: `recording_YYYYMMDD_HHMMSS.mkv`
- Metadata file: `recording_YYYYMMDD_HHMMSS.meta.json` (same folder)

## Current limitations (important)

- The Windows capture pipeline is stubbed, so real video frames are not produced yet.
- The encoder is stubbed, so `.mkv` video files are not created. Only `.meta.json` files are generated.

See `documentation/RECORDING_STATUS.md` and `VIDEO_FILE_STATUS.md` for details.

## Troubleshooting quick fixes

- Port 1420 in use: `npm run cleanup`
- Blank window: open `http://localhost:1420` in a browser and check Tauri devtools (F12)
- Tauri CLI missing: `npm install -g @tauri-apps/cli@next`

## More detailed docs

- `documentation/STARTUP_GUIDE.md`
- `documentation/DEPENDENCY_GUIDE.md`
- `documentation/RUN_AND_BUILD_GUIDE.md`
- `documentation/TROUBLESHOOTING.md`
- `documentation/WSL_FIX.md`


