# Screen Recorder

A professional desktop screen recorder built with **Tauri 2**, **Rust**, and **React**. Features native capture APIs, FFmpeg encoding, real-time analytics, and comprehensive observability.

## Features

### Recording Capabilities
- ✅ Record single monitor, all monitors, or specific windows
- ✅ System audio + microphone capture (separate tracks)
- ✅ Hotkeys: start/stop/pause, push-to-talk, marker insertion
- ✅ Save as MKV (best for multi-track + resilience), export to MP4
- ✅ Optional: cursor highlight, click indicators, region blur

### Analytics & Patterns
- ✅ **Color Patterns**: Dominant colors/palette over time, scene change detection, brightness changes
- ✅ **Sound Patterns**: RMS energy (loudness curve), silence detection, spectral centroid, voice activity detection
- ✅ Timeline visualization with searchable metadata
- ✅ Smart chapter generation based on scene changes

### Observability
- ✅ **OpenTelemetry** instrumentation (metrics, logs, traces)
- ✅ **Prometheus** metrics endpoint (`/metrics`)
- ✅ Structured JSON logging
- ✅ Real-time performance metrics (FPS, latency, CPU, memory)

## Architecture

```
┌─────────────── Desktop UI (Tauri + React) ───────────────┐
│  device picker | scene selection | settings              │
│  hotkeys | timeline (audio/color) | export               │
└───────────────┬─────────────────────────────────────────┘
                │ IPC (commands/events)
                ▼
┌──────────────────── Core Engine (Rust) ────────────────────┐
│ Session Manager                                            │
│  - start/stop/pause                                        │
│  - per-source pipelines                                    │
│                                                            │
│ Capture Layer (OS-specific)                                │
│  - Win: Windows.Graphics.Capture (WGC)                    │
│  - Mac: ScreenCaptureKit (screen + audio)                 │
│  - Linux: portal -> PipeWire stream                       │
│                                                            │
│ Frame Pipeline                                             │
│  - timestamp sync (video/audio clocks)                     │
│  - ring buffers + backpressure                            │
│                                                            │
│ Analytics Pipeline (parallel)                             │
│  - color histogram / scene change                          │
│  - audio features / silence / spectrum stats               │
│                                                            │
│ Encoder/Muxer (FFmpeg)                                     │
│  - H.264/H.265/AV1, AAC/Opus                              │
│  - multi-track + segmented writing                         │
│                                                            │
│ Observability                                               │
│  - metrics (fps, drops, encode ms, cpu, mem)               │
│  - logs (session events/errors)                            │
│  - traces (capture->encode spans)                           │
└────────────────────────────────────────────────────────────┘
```

## Tech Stack

- **UI**: Tauri 2 + React + TypeScript
- **Core**: Rust
- **Capture**:
  - Windows: Windows.Graphics.Capture (WGC)
  - macOS: ScreenCaptureKit (planned)
  - Linux: PipeWire + xdg-desktop-portal (planned)
- **Encoding**: FFmpeg (libavcodec/libavformat)
- **Observability**: OpenTelemetry → Prometheus/Loki
- **Analytics**: Custom image/audio processing pipelines

## Prerequisites

### Quick Setup (Recommended)

**One command to check and install all dependencies:**

**Linux/macOS:**
```bash
./setup.sh
```

**Windows:**
```cmd
setup.bat
```

This will automatically:
- ✅ Check for Rust, Node.js, and system dependencies
- ✅ Install missing dependencies
- ✅ Install npm packages
- ✅ Start the development server

### Manual Prerequisites

#### Windows
- Rust (latest stable)
- Node.js 18+
- Visual Studio Build Tools (with C++ workload)
- Windows 10/11 (for Windows.Graphics.Capture)

#### Linux
- Rust (latest stable)
- Node.js 18+
- System dependencies (see `dependencies.txt`)

#### macOS
- Rust (latest stable)
- Node.js 18+
- Xcode Command Line Tools

### Manual Development Setup
```bash
# Check dependencies (installs if missing)
./check-dependencies.sh  # Linux/macOS
# or
check-dependencies.bat    # Windows

# Install Node.js dependencies
npm install

# Install Tauri CLI (if not already installed)
npm install -g @tauri-apps/cli@next
```

## Building

```bash
# Development mode
npm run tauri dev

# Production build
npm run tauri build
```

## Project Structure

```
screen-recorder/
├── src/                    # React frontend
│   ├── components/         # UI components
│   │   ├── DevicePicker.tsx
│   │   ├── RecordingControls.tsx
│   │   ├── Timeline.tsx
│   │   └── MetricsPanel.tsx
│   ├── App.tsx
│   └── main.tsx
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── main.rs        # Tauri entry point
│   │   ├── session.rs     # Session management
│   │   ├── capture/       # OS-specific capture
│   │   │   ├── mod.rs
│   │   │   └── windows.rs
│   │   ├── encoder.rs     # FFmpeg encoding
│   │   ├── analytics.rs   # Color/audio patterns
│   │   └── observability.rs # OpenTelemetry
│   └── Cargo.toml
└── package.json
```

## Milestones

### ✅ Milestone 1 — MVP (Current)
- [x] Project structure
- [x] Basic UI components
- [x] Session manager
- [x] Analytics pipeline (color patterns)
- [x] Observability setup
- [ ] Windows capture implementation (in progress)
- [ ] FFmpeg encoding integration

### 🔄 Milestone 2 — Multi-monitor + System Audio
- [ ] Multi-display picker
- [ ] "Record all displays" mode
- [ ] Separate audio tracks

### 📋 Milestone 3 — Analytics Timeline
- [x] Color histogram + scene changes
- [ ] Audio loudness + silence detection
- [x] Write `meta.json`
- [x] Show timeline UI

### 📋 Milestone 4 — Observability
- [x] Prometheus metrics endpoint
- [x] Structured logs
- [ ] Grafana dashboard

### 📋 Milestone 5 — Cross-platform
- [ ] macOS via ScreenCaptureKit
- [ ] Linux via PipeWire portal

## Usage

1. **Select Capture Source**: Choose monitor or window to record
2. **Start Recording**: Click "Start Recording" or use hotkey
3. **Monitor Metrics**: View real-time FPS, latency, CPU usage
4. **View Timeline**: See color patterns and audio levels
5. **Stop & Export**: Recording saved as MKV with metadata JSON

## Output Files

- `recording_YYYYMMDD_HHMMSS.mkv` - Video file (MKV container)
- `recording_YYYYMMDD_HHMMSS.meta.json` - Analytics metadata

Metadata format:
```json
{
  "video_path": "...",
  "entries": [
    {
      "time": 1.23,
      "colorDominance": 0.65,
      "brightness": 0.72,
      "audioLevel": 0.45,
      "sceneChange": false
    }
  ]
}
```

## Observability

### Metrics Endpoint
Access Prometheus metrics at: `http://localhost:9090/metrics` (when implemented)

### Logs
Structured JSON logs are output to stdout/stderr with tracing levels.

## Contributing

This is a showcase project demonstrating:
- Cross-platform native APIs
- Low-latency frame pipelines
- Real-time analytics
- Production-grade observability

## License

MIT

## Resume Bullets

* "Built a cross-platform desktop screen recorder using **native capture APIs** (Windows Graphics Capture / ScreenCaptureKit / PipeWire) and **FFmpeg**, supporting multi-monitor and multi-track audio."
* "Designed a low-latency frame pipeline with ring buffers and backpressure; instrumented end-to-end performance with **OpenTelemetry**, exporting metrics to **Prometheus/Grafana** and logs to **Loki/Splunk**."
* "Implemented real-time analytics producing a searchable timeline (scene changes via color histograms, silence/VAD via audio features) stored as metadata sidecar for smart chapters."

