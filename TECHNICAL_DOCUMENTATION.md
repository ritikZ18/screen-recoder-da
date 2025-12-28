# Screen Recorder - Technical Documentation

## Table of Contents

1. [System Architecture](#system-architecture)
2. [Startup Procedures](#startup-procedures)
3. [Configuration](#configuration)
4. [Maintenance Guide](#maintenance-guide)
5. [Troubleshooting](#troubleshooting)
6. [Performance Tuning](#performance-tuning)
7. [Deployment](#deployment)
8. [Monitoring & Observability](#monitoring--observability)

---

## System Architecture

### Overview

The Screen Recorder is a cross-platform desktop application built with:
- **Frontend**: Tauri 2 + React + TypeScript
- **Backend**: Rust (async with Tokio)
- **Capture**: OS-specific native APIs
- **Encoding**: FFmpeg (libavcodec/libavformat)
- **Observability**: OpenTelemetry + Prometheus

### Component Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Frontend (React)                         │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  │
│  │ Device   │  │Recording │  │ Timeline │  │ Metrics  │  │
│  │ Picker   │  │Controls  │  │          │  │ Panel    │  │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘  │
└────────────────────────┬────────────────────────────────────┘
                         │ Tauri IPC
                         ▼
┌─────────────────────────────────────────────────────────────┐
│              Backend Core (Rust)                             │
│  ┌──────────────────────────────────────────────────────┐  │
│  │           Session Manager                              │  │
│  │  - Recording lifecycle                                 │  │
│  │  - State management                                    │  │
│  │  - Task orchestration                                  │  │
│  └───────────────┬──────────────────────────────────────┘  │
│                  │                                           │
│  ┌───────────────┴──────────────────────────────────────┐  │
│  │           Capture Layer                                │  │
│  │  Windows: Windows.Graphics.Capture                     │  │
│  │  macOS: ScreenCaptureKit (planned)                      │  │
│  │  Linux: PipeWire (planned)                            │  │
│  └───────────────┬──────────────────────────────────────┘  │
│                  │                                           │
│  ┌───────────────┴──────────────────────────────────────┐  │
│  │           Frame Pipeline                               │  │
│  │  - Frame capture → Analytics → Encoding                │  │
│  │  - Backpressure handling                              │  │
│  │  - Timestamp synchronization                          │  │
│  └───────────────┬──────────────────────────────────────┘  │
│                  │                                           │
│  ┌───────────────┴──────────────────────────────────────┐  │
│  │           Analytics Pipeline                           │  │
│  │  - Color pattern analysis                              │  │
│  │  - Scene change detection                              │  │
│  │  - Audio level estimation                             │  │
│  └───────────────┬──────────────────────────────────────┘  │
│                  │                                           │
│  ┌───────────────┴──────────────────────────────────────┐  │
│  │           Encoder (FFmpeg)                             │  │
│  │  - H.264/H.265 encoding                               │  │
│  │  - MKV container                                       │  │
│  │  - Multi-track support                                 │  │
│  └───────────────┬──────────────────────────────────────┘  │
│                  │                                           │
│  ┌───────────────┴──────────────────────────────────────┐  │
│  │           Observability                                │  │
│  │  - OpenTelemetry metrics                              │  │
│  │  - Structured logging                                │  │
│  │  - System metrics (CPU, memory)                      │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

### Data Flow

1. **Capture**: OS API → Frame (RGB) → Timestamp
2. **Analytics**: Frame → Color Analysis → Scene Detection → Timeline Entry
3. **Encoding**: Frame → FFmpeg → Encoded Packet → MKV File
4. **Metrics**: System Stats → Prometheus → UI Display

### Key Design Patterns

- **Trait-based Capture**: `CaptureTrait` allows platform-specific implementations
- **Arc<Mutex<>>**: Shared state for concurrent access
- **Async/Await**: Non-blocking I/O throughout
- **Error Handling**: `anyhow::Result` for error propagation
- **Observability**: Structured logging with tracing

---

## Startup Procedures

### Prerequisites

#### Windows
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js 18+
# Download from: https://nodejs.org/

# Install Visual Studio Build Tools
# Required for Windows native compilation
```

#### Linux
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install system dependencies
sudo apt-get update
sudo apt-get install -y \
    libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev

# Install Node.js
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs
```

#### macOS
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Homebrew (if not installed)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install dependencies
brew install node@18

# Install Xcode Command Line Tools
xcode-select --install
```

### Initial Setup

1. **Clone/Download Project**
   ```bash
   cd screen-recorder
   ```

2. **Install Dependencies**
   ```bash
   npm install
   ```

3. **Verify Installation**
   ```bash
   # Check Rust version
   rustc --version  # Should be 1.70+

   # Check Node version
   node --version   # Should be 18+

   # Check Tauri CLI
   npm list -g @tauri-apps/cli
   ```

### Development Startup

1. **Start Development Server**
   ```bash
   npm run tauri dev
   ```

   This command:
   - Starts Vite dev server on `http://localhost:1420`
   - Compiles Rust backend
   - Launches Tauri window
   - Enables hot-reload for frontend changes

2. **Verify Application**
   - Window should open with UI
   - Device picker should show monitors/windows
   - No console errors

### Production Build

1. **Build Application**
   ```bash
   npm run tauri build
   ```

2. **Output Location**
   - Windows: `src-tauri/target/release/screen-recorder.exe`
   - Linux: `src-tauri/target/release/screen-recorder`
   - macOS: `src-tauri/target/release/bundle/`

3. **Distribution**
   - Windows: `.msi` installer in `src-tauri/target/release/bundle/msi/`
   - Linux: `.deb` or `.AppImage` in `src-tauri/target/release/bundle/`
   - macOS: `.dmg` in `src-tauri/target/release/bundle/dmg/`

---

## Configuration

### Application Configuration

#### Tauri Configuration (`src-tauri/tauri.conf.json`)

```json
{
  "productName": "Screen Recorder",
  "version": "1.0.0",
  "identifier": "com.screenrecorder.app",
  "app": {
    "windows": [{
      "title": "Screen Recorder",
      "width": 1200,
      "height": 800,
      "resizable": true
    }]
  }
}
```

### Recording Configuration

#### Default Settings
- **Output Format**: MKV
- **Video Codec**: H.264
- **Frame Rate**: 30 FPS (configurable)
- **Quality**: High (CRF 18)
- **Output Directory**: `~/Videos/ScreenRecordings/`

#### Customization

Edit `src-tauri/src/encoder.rs` to modify:
- Codec selection
- Bitrate/quality settings
- Frame rate
- Resolution scaling

### Observability Configuration

#### Logging Levels

Set via environment variable:
```bash
RUST_LOG=screen_recorder=info npm run tauri dev
```

Available levels:
- `error`: Errors only
- `warn`: Warnings and errors
- `info`: Informational messages (default)
- `debug`: Debug information
- `trace`: Detailed tracing

#### Metrics Endpoint

Prometheus metrics available at:
- Development: `http://localhost:9090/metrics` (when implemented)
- Production: Configured via environment

---

## Maintenance Guide

### Regular Maintenance Tasks

#### 1. Dependency Updates

**Monthly**
```bash
# Update Rust dependencies
cd src-tauri
cargo update

# Update Node dependencies
npm update

# Check for security vulnerabilities
npm audit
```

#### 2. Clean Build Artifacts

**Weekly or after major changes**
```bash
# Clean Rust build
cd src-tauri
cargo clean

# Clean Node modules (if issues)
rm -rf node_modules package-lock.json
npm install
```

#### 3. Log Rotation

**Automated** (recommended):
- Configure system log rotation for application logs
- Keep last 7 days of logs
- Compress older logs

**Manual**:
```bash
# Clear old logs (if stored locally)
find ~/.screen-recorder/logs -name "*.log" -mtime +7 -delete
```

#### 4. Output Directory Cleanup

**Monthly**
```bash
# Remove recordings older than 30 days
find ~/Videos/ScreenRecordings -name "*.mkv" -mtime +30 -delete
find ~/Videos/ScreenRecordings -name "*.meta.json" -mtime +30 -delete
```

### Code Maintenance

#### 1. Code Quality Checks

```bash
# Rust formatting
cd src-tauri
cargo fmt

# Rust linting
cargo clippy -- -D warnings

# TypeScript linting
npm run lint  # If configured
```

#### 2. Testing

```bash
# Run Rust tests
cd src-tauri
cargo test

# Run integration tests (if implemented)
cargo test --test integration
```

#### 3. Documentation Updates

- Update `README.md` for user-facing changes
- Update `TECHNICAL_DOCUMENTATION.md` for architecture changes
- Update code comments for API changes

### Performance Monitoring

#### Key Metrics to Monitor

1. **Capture FPS**: Should be stable at target FPS
2. **Encode FPS**: Should match or exceed capture FPS
3. **Dropped Frames**: Should be < 1% of total frames
4. **CPU Usage**: Should be < 50% on modern hardware
5. **Memory Usage**: Should be stable, not growing

#### Monitoring Commands

```bash
# View real-time metrics (when metrics endpoint is active)
curl http://localhost:9090/metrics

# Check process resource usage
# Windows: Task Manager
# Linux: htop or top
# macOS: Activity Monitor
```

---

## Troubleshooting

### Common Issues

#### 1. Application Won't Start

**Symptoms**: Window doesn't open, process exits immediately

**Diagnosis**:
```bash
# Check logs
RUST_LOG=debug npm run tauri dev

# Check for missing dependencies
npm list
cd src-tauri && cargo check
```

**Solutions**:
- Verify all prerequisites are installed
- Check for port conflicts (port 1420)
- Verify Rust toolchain: `rustc --version`
- Reinstall dependencies: `rm -rf node_modules && npm install`

#### 2. Recording Fails to Start

**Symptoms**: "Failed to create capture source" error

**Diagnosis**:
- Check Windows permissions (screen recording requires permission)
- Verify monitor/window selection
- Check for other applications using capture APIs

**Solutions**:
- Grant screen recording permission (Windows Settings → Privacy → Screen Recording)
- Close other screen recording applications
- Restart application
- Check Windows version (requires Windows 10 1809+)

#### 3. Poor Performance / Dropped Frames

**Symptoms**: Low FPS, high CPU usage, dropped frames

**Diagnosis**:
```bash
# Check system resources
# Windows: Task Manager → Performance
# Linux: htop
# macOS: Activity Monitor
```

**Solutions**:
- Reduce recording resolution
- Lower frame rate (edit encoder settings)
- Close other resource-intensive applications
- Check disk I/O (slow disk can cause issues)
- Update graphics drivers

#### 4. Encoding Errors

**Symptoms**: "Encoding error" in logs, corrupted output files

**Diagnosis**:
- Check disk space: `df -h` (Linux/macOS) or check drive (Windows)
- Verify FFmpeg installation (if using system FFmpeg)
- Check output directory permissions

**Solutions**:
- Free up disk space
- Change output directory
- Verify write permissions
- Reinstall FFmpeg (if using system installation)

#### 5. Analytics Not Working

**Symptoms**: Timeline shows no data, empty metadata files

**Diagnosis**:
- Check analytics pipeline logs
- Verify frame processing is occurring
- Check memory usage (analytics buffers)

**Solutions**:
- Restart recording
- Check available memory
- Verify analytics module is initialized

### Debug Mode

Enable detailed logging:
```bash
RUST_LOG=screen_recorder=debug npm run tauri dev
```

View logs:
- **Windows**: Console window or Event Viewer
- **Linux/macOS**: Terminal output or system logs

### Getting Help

1. Check logs with `RUST_LOG=debug`
2. Review error messages in console
3. Check system requirements
4. Verify permissions
5. Review GitHub issues (if applicable)

---

## Performance Tuning

### Optimization Strategies

#### 1. Frame Rate Optimization

**Default**: 30 FPS

**Adjust in** `src-tauri/src/encoder.rs`:
```rust
// Target FPS
const TARGET_FPS: f64 = 30.0;
```

**Recommendations**:
- **High Quality**: 30 FPS (default)
- **Balanced**: 24 FPS
- **Low Resource**: 15 FPS

#### 2. Resolution Scaling

**Implementation**: Add resolution scaling in capture layer

**Benefits**:
- Reduced CPU usage
- Lower memory consumption
- Faster encoding

**Trade-offs**:
- Lower output quality
- May affect analytics accuracy

#### 3. Encoding Quality

**Current**: High quality (placeholder)

**Options**:
- **CRF 18**: High quality, larger files
- **CRF 23**: Balanced (recommended)
- **CRF 28**: Lower quality, smaller files

#### 4. Buffer Management

**Frame Buffer Size**: Currently unlimited (with 1000 entry limit for analytics)

**Optimization**:
- Implement ring buffer with fixed size
- Drop oldest frames if buffer full
- Monitor buffer fill level

#### 5. Parallel Processing

**Current**: Sequential (capture → analytics → encode)

**Optimization**:
- Run analytics in parallel with encoding
- Use separate threads for CPU-intensive tasks
- Consider GPU acceleration for encoding

### Resource Limits

#### Memory
- **Minimum**: 2 GB RAM
- **Recommended**: 4 GB RAM
- **Optimal**: 8 GB+ RAM

#### CPU
- **Minimum**: 2 cores
- **Recommended**: 4+ cores
- **Optimal**: 6+ cores with high clock speed

#### Disk
- **Per Minute**: ~50-100 MB (depending on quality)
- **1 Hour**: ~3-6 GB
- Ensure sufficient free space

---

## Deployment

### Build for Distribution

#### Windows

```bash
npm run tauri build
# Output: src-tauri/target/release/bundle/msi/screen-recorder_1.0.0_x64_en-US.msi
```

**Distribution**:
- Sign executable (recommended for Windows)
- Test on clean Windows 10/11 installation
- Package with installer

#### Linux

```bash
npm run tauri build
# Output: src-tauri/target/release/bundle/deb/screen-recorder_1.0.0_amd64.deb
```

**Distribution**:
- Test on target distributions
- Consider AppImage for universal compatibility
- Package dependencies if needed

#### macOS

```bash
npm run tauri build
# Output: src-tauri/target/release/bundle/dmg/screen-recorder_1.0.0_x64.dmg
```

**Distribution**:
- Code sign application (required for distribution)
- Notarize with Apple (required for Gatekeeper)
- Test on clean macOS installation

### Update Strategy

#### Version Management

1. **Semantic Versioning**: `MAJOR.MINOR.PATCH`
2. **Update `tauri.conf.json`**: Version number
3. **Update `Cargo.toml`**: Version number
4. **Update `package.json`**: Version number

#### Update Distribution

1. Build new version
2. Test thoroughly
3. Create release notes
4. Distribute via:
   - Direct download
   - Auto-update (if implemented)
   - Package managers (if applicable)

---

## Monitoring & Observability

### Metrics Collection

#### Available Metrics

1. **Capture Metrics**
   - `capture_fps`: Frames captured per second
   - `dropped_frames`: Frames dropped due to backpressure

2. **Encoding Metrics**
   - `encode_fps`: Frames encoded per second
   - `encode_latency`: Average encoding time per frame

3. **System Metrics**
   - `cpu_usage`: CPU usage percentage
   - `memory_usage`: Memory usage in bytes

4. **Session Metrics**
   - `recording_duration`: Total recording time
   - `frames_encoded`: Total frames processed

### Logging

#### Log Levels

- **ERROR**: Critical errors requiring attention
- **WARN**: Warnings that may indicate issues
- **INFO**: General information about operations
- **DEBUG**: Detailed debugging information
- **TRACE**: Very detailed tracing (performance impact)

#### Log Format

Structured JSON logs:
```json
{
  "timestamp": "2024-01-01T12:00:00Z",
  "level": "INFO",
  "target": "screen_recorder::session",
  "message": "Recording started",
  "fields": {
    "monitor_id": "primary",
    "output_path": "/path/to/output.mkv"
  }
}
```

### Prometheus Integration

#### Metrics Endpoint

When implemented, metrics available at:
```
http://localhost:9090/metrics
```

#### Example Queries

```promql
# Capture FPS
rate(capture_frames_total[5m])

# Dropped frames percentage
(dropped_frames_total / capture_frames_total) * 100

# Average encode latency
encode_latency_seconds
```

### Grafana Dashboard

#### Recommended Panels

1. **Capture Performance**
   - Capture FPS (line graph)
   - Dropped frames (bar chart)

2. **Encoding Performance**
   - Encode FPS (line graph)
   - Encode latency (histogram)

3. **System Resources**
   - CPU usage (gauge)
   - Memory usage (gauge)

4. **Session Information**
   - Active recordings (stat)
   - Total duration (stat)

---

## Appendix

### File Locations

#### Windows
- **Logs**: `%APPDATA%\screen-recorder\logs\`
- **Output**: `%USERPROFILE%\Videos\ScreenRecordings\`
- **Config**: `%APPDATA%\screen-recorder\config.json`

#### Linux
- **Logs**: `~/.local/share/screen-recorder/logs/`
- **Output**: `~/Videos/ScreenRecordings/`
- **Config**: `~/.config/screen-recorder/config.json`

#### macOS
- **Logs**: `~/Library/Logs/screen-recorder/`
- **Output**: `~/Movies/ScreenRecordings/`
- **Config**: `~/Library/Application Support/screen-recorder/config.json`

### Environment Variables

- `RUST_LOG`: Logging level (default: `info`)
- `SCREEN_RECORDER_OUTPUT_DIR`: Custom output directory
- `SCREEN_RECORDER_METRICS_PORT`: Prometheus metrics port (default: 9090)

### Support Contacts

- **Documentation**: See `README.md` and `QUICKSTART.md`
- **Issues**: GitHub Issues (if applicable)
- **Technical Questions**: Review `IMPLEMENTATION_NOTES.md`

---

**Last Updated**: 2024-12-28
**Version**: 1.0.0

