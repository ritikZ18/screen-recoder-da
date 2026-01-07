# Implementation Notes

## Current Status

This is a **production-ready architecture** with a **working foundation**. The following components are implemented:

### ✅ Completed
- Project structure (Tauri 2 + React + TypeScript)
- UI components (DevicePicker, RecordingControls, Timeline, MetricsPanel)
- Session Manager architecture
- Analytics Pipeline (color patterns, scene detection)
- Observability setup (OpenTelemetry, Prometheus, structured logging)
- Encoder structure (FFmpeg integration points)
- Cross-platform capture trait design

### 🔄 In Progress / TODO

#### Windows Capture (High Priority)
The Windows capture implementation in `src-tauri/src/capture/windows.rs` needs to be completed. This requires:

1. **Direct3D Device Setup**
   ```rust
   // Create ID3D11Device and ID3D11DeviceContext
   // Use D3D11CreateDevice
   ```

2. **Graphics Capture Item Creation**
   ```rust
   // For monitors:
   let item = GraphicsCaptureItem::CreateFromMonitorId(monitor_id)?;
   
   // For windows:
   let item = GraphicsCaptureItem::CreateFromWindowId(window_id)?;
   ```

3. **Frame Pool Creation**
   ```rust
   let frame_pool = Direct3D11CaptureFramePool::CreateFreeThreaded(
       device,
       DirectXPixelFormat::B8G8R8A8UIntNormalized,
       2, // buffer count
       size
   )?;
   ```

4. **Frame Extraction**
   ```rust
   // Get frame from pool
   let frame = frame_pool.TryGetNextFrame()?;
   // Convert to RGB using Direct3D or CPU fallback
   ```

**Reference**: [Windows.Graphics.Capture Documentation](https://learn.microsoft.com/en-us/uwp/api/windows.graphics.capture)

#### FFmpeg Integration
The encoder in `src-tauri/src/encoder.rs` needs FFmpeg bindings:

1. **Initialize FFmpeg Context**
   ```rust
   use ffmpeg_next as ffmpeg;
   
   ffmpeg::init()?;
   let mut output = ffmpeg::format::output(&output_path)?;
   ```

2. **Setup Video Stream**
   ```rust
   let mut stream = output.add_stream(ffmpeg::encoder::find(ffmpeg::encoder::Id::H264))?;
   // Configure codec parameters
   ```

3. **Encode Frames**
   ```rust
   // Convert frame to AVFrame
   // Encode with codec context
   // Write packet to output
   ```

**Alternative**: Use `ffmpeg-next` crate or spawn FFmpeg process with pipes.

#### Audio Capture
Audio capture is not yet implemented. Options:

1. **Windows**: Use `Windows.Media.Audio` or `WASAPI`
2. **Cross-platform**: Use `cpal` crate (Rust audio library)
3. **FFmpeg**: Use `ffmpeg` to capture system audio directly

## Next Steps

### Milestone 1 Completion
1. Complete Windows capture implementation
2. Integrate FFmpeg encoding
3. Test end-to-end recording

### Milestone 2
1. Add audio capture
2. Implement multi-monitor support
3. Add window enumeration

### Milestone 3
1. Complete audio analytics (RMS, silence detection)
2. Add spectral analysis
3. Improve timeline visualization

## Building & Testing

```bash
# Install dependencies
npm install

# Development (requires Rust + Tauri)
npm run tauri dev

# Build (will fail until Windows capture is implemented)
npm run tauri build
```

## Architecture Decisions

### Why Tauri 2?
- Small binary size
- Native performance
- Cross-platform support
- Active ecosystem

### Why Rust?
- Memory safety
- Performance (zero-cost abstractions)
- Excellent FFI for native APIs
- Strong async support (Tokio)

### Why FFmpeg?
- Industry standard
- Supports all codecs/containers
- Well-documented
- Cross-platform

### Why OpenTelemetry?
- Vendor-neutral
- Standard observability
- Easy integration with Prometheus/Grafana
- Production-ready

## Performance Considerations

1. **Frame Buffering**: Use ring buffers to prevent memory bloat
2. **Backpressure**: Drop frames if encoder falls behind (configurable)
3. **Parallel Processing**: Analytics runs in parallel with encoding
4. **Segmented Writing**: Write in chunks for crash resilience

## Security Considerations

1. **Permissions**: Screen capture requires user permission on all platforms
2. **Data Storage**: Recordings stored in user's video directory
3. **No Network**: All processing is local (no cloud upload)

## Known Limitations

1. Windows-only capture (macOS/Linux pending)
2. No audio capture yet
3. FFmpeg integration incomplete
4. Metrics endpoint not exposed (needs HTTP server)

## Contributing

When implementing Windows capture:
1. Test with different monitor configurations
2. Handle window resizing during capture
3. Implement proper error handling
4. Add unit tests for frame conversion

When implementing FFmpeg:
1. Support multiple codecs (H.264, H.265, AV1)
2. Handle encoding errors gracefully
3. Implement proper muxing for multi-track audio
4. Add progress callbacks

