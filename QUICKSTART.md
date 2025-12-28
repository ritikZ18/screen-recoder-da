# Quick Start Guide

## Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js 18+ (if not already installed)
# Visit: https://nodejs.org/

# Install Tauri CLI
npm install -g @tauri-apps/cli@next
```

## Setup

```bash
cd screen-recorder

# Install dependencies
npm install
```

## Development

```bash
# Start development server
npm run tauri dev
```

This will:
1. Start the Vite dev server (frontend)
2. Build the Rust backend
3. Launch the Tauri app

**Note**: The app will launch but recording won't work until Windows capture is implemented. The UI is fully functional for testing.

## Building

```bash
# Build for production
npm run tauri build
```

Output will be in `src-tauri/target/release/`

## Project Status

### ✅ Working
- UI components and layout
- Device picker UI
- Recording controls UI
- Metrics panel UI
- Timeline visualization UI
- Session management logic
- Analytics pipeline (color patterns)
- Observability setup

### 🔄 Needs Implementation
- Windows.Graphics.Capture integration
- FFmpeg encoding
- Audio capture
- Actual frame capture loop

## Testing the UI

Even without capture implemented, you can:
1. See the device picker (shows placeholder monitor)
2. Click "Start Recording" (will show recording state)
3. View the metrics panel (shows placeholder values)
4. See the timeline component

## Next Implementation Steps

See `IMPLEMENTATION_NOTES.md` for detailed instructions on:
1. Implementing Windows capture
2. Integrating FFmpeg
3. Adding audio capture

## Troubleshooting

### Rust not found
```bash
# Add Rust to PATH
source $HOME/.cargo/env
```

### Tauri build fails
- Ensure you have the Tauri prerequisites installed
- For Windows: Visual Studio Build Tools
- For Linux: webkit2gtk, libssl-dev, etc.

### npm install fails
- Clear cache: `npm cache clean --force`
- Delete `node_modules` and `package-lock.json`, then reinstall

## Architecture Overview

```
Frontend (React) → Tauri IPC → Backend (Rust)
                                    ↓
                    ┌───────────────┴───────────────┐
                    ↓                               ↓
            Capture Layer                    Analytics Pipeline
            (Windows API)                    (Color patterns)
                    ↓                               ↓
                    └───────────────┬───────────────┘
                                    ↓
                            Encoder (FFmpeg)
                                    ↓
                            Output (MKV + JSON)
```

## Key Files

- `src/App.tsx` - Main React component
- `src-tauri/src/main.rs` - Tauri entry point
- `src-tauri/src/session.rs` - Recording session logic
- `src-tauri/src/capture/windows.rs` - **Implement here for Windows capture**
- `src-tauri/src/encoder.rs` - **Implement here for FFmpeg encoding**

## Resources

- [Tauri Documentation](https://tauri.app/)
- [Windows.Graphics.Capture](https://learn.microsoft.com/en-us/uwp/api/windows.graphics.capture)
- [FFmpeg Documentation](https://ffmpeg.org/documentation.html)
- [OpenTelemetry Rust](https://opentelemetry.io/docs/instrumentation/rust/)

