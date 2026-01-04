# Video File Status

## Current Implementation Status

⚠️ **Important Note**: The video encoder is currently **stubbed/not implemented**. The application:
- ✅ Captures screen frames successfully
- ✅ Processes analytics and creates timeline data
- ✅ Saves metadata files (`.meta.json`)
- ❌ **Does NOT create actual video files (`.mkv`)**

## What You're Seeing

1. **`.meta.json` files**: These contain analytics metadata (brightness, color dominance, scene changes, etc.) but are NOT video files.

2. **No `.mkv` files**: The actual video files are not being created because the encoder needs FFmpeg integration.

## To Fix Video Recording

The encoder (`src-tauri/src/encoder.rs`) needs to be updated to:
1. Integrate FFmpeg library (e.g., `ffmpeg-next` crate)
2. Convert RGB frames to YUV420p format
3. Encode frames using H.264 codec
4. Write encoded packets to the `.mkv` file

Currently, the `encode_frame` function only tracks metrics but doesn't actually encode video.

## Metrics Explanation

- **Capture FPS**: Actual frames captured per second (should be ~30 FPS)
- **Encode FPS**: Currently same as capture FPS (will differ when real encoding is implemented)
- **Dropped Frames**: Frames that couldn't be processed in time
- **Encode Latency**: Time to process each frame (milliseconds)
- **CPU Usage**: Process CPU usage as percentage (capped at 100%)
- **Memory**: Process memory usage in MB

