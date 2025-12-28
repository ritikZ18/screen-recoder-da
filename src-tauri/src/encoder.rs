use crate::capture::Frame;
use anyhow::{Context, Result};
use serde_json::json;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::Instant;
use std::collections::VecDeque;

pub struct Encoder {
    output_path: PathBuf,
    width: u32,
    height: u32,
    frame_count: u64,
    encode_time: std::time::Duration,
    dropped_frames: u64,
    frame_timestamps: VecDeque<Instant>,
    fps_calculator: FpsCalculator,
}

struct FpsCalculator {
    timestamps: VecDeque<Instant>,
    max_samples: usize,
}

impl FpsCalculator {
    fn new() -> Self {
        Self {
            timestamps: VecDeque::with_capacity(60),
            max_samples: 60,
        }
    }

    fn add_frame(&mut self, timestamp: Instant) -> f64 {
        self.timestamps.push_back(timestamp);
        
        // Keep only last N samples
        while self.timestamps.len() > self.max_samples {
            self.timestamps.pop_front();
        }

        // Calculate FPS from time span
        if self.timestamps.len() >= 2 {
            let first = self.timestamps.front().unwrap();
            let last = self.timestamps.back().unwrap();
            let duration = last.duration_since(*first).as_secs_f64();
            
            if duration > 0.0 {
                (self.timestamps.len() - 1) as f64 / duration
            } else {
                0.0
            }
        } else {
            0.0
        }
    }
}

impl Encoder {
    pub async fn new(output_path: PathBuf) -> Result<Self> {
        Ok(Self {
            output_path,
            width: 0,
            height: 0,
            frame_count: 0,
            encode_time: std::time::Duration::ZERO,
            dropped_frames: 0,
            frame_timestamps: VecDeque::with_capacity(60),
            fps_calculator: FpsCalculator::new(),
        })
    }

    pub async fn initialize(&mut self) -> Result<()> {
        // Initialize FFmpeg encoder
        // Note: For production, you would use ffmpeg-next or spawn FFmpeg process
        // This is a simplified implementation that prepares the structure
        
        // In a full implementation, you would:
        // 1. Initialize FFmpeg: ffmpeg::init()
        // 2. Create output format context
        // 3. Add video stream with H.264 codec
        // 4. Add audio stream with AAC codec (if audio capture is enabled)
        // 5. Open output file
        
        // For now, we'll use a file-based approach that can be extended
        tracing::info!("Encoder initialized for output: {:?}", self.output_path);
        Ok(())
    }

    pub async fn encode_frame(&mut self, frame: &Frame) -> Result<()> {
        let start = Instant::now();

        // Set dimensions on first frame
        if self.width == 0 {
            self.width = frame.width;
            self.height = frame.height;
            tracing::info!("Encoder configured for {}x{}", self.width, self.height);
        }

        // Check for frame drops (if encoding takes too long)
        if self.frame_timestamps.len() > 0 {
            let last_timestamp = self.frame_timestamps.back().unwrap();
            let time_since_last = start.duration_since(*last_timestamp);
            
            // If more than 2 frame intervals have passed, consider frames dropped
            let expected_interval = std::time::Duration::from_secs_f64(1.0 / 30.0); // 30 FPS
            if time_since_last > expected_interval * 2 {
                let dropped = ((time_since_last.as_secs_f64() / expected_interval.as_secs_f64()) - 1.0) as u64;
                self.dropped_frames += dropped;
            }
        }

        // TODO: Encode frame using FFmpeg
        // In production implementation:
        // 1. Convert RGB frame to YUV420p format
        // 2. Create AVFrame from frame data
        // 3. Encode frame using codec context
        // 4. Write encoded packet to output file
        
        // For now, we simulate encoding by just tracking metrics
        // In a real implementation, you would write the frame data to FFmpeg
        
        self.frame_count += 1;
        self.frame_timestamps.push_back(start);
        self.encode_time += start.elapsed();
        
        // Keep only last 60 timestamps for FPS calculation
        if self.frame_timestamps.len() > 60 {
            self.frame_timestamps.pop_front();
        }

        Ok(())
    }

    pub async fn finalize(&mut self) -> Result<()> {
        // Finalize FFmpeg encoder
        // In production implementation:
        // 1. Flush encoder buffers (encode remaining frames)
        // 2. Write file trailer
        // 3. Close output file
        // 4. Clean up codec contexts
        
        tracing::info!(
            "Encoder finalized: {} frames encoded, {} dropped",
            self.frame_count,
            self.dropped_frames
        );
        
        Ok(())
    }

    pub fn get_metrics(&self) -> serde_json::Value {
        let avg_encode_time = if self.frame_count > 0 {
            self.encode_time.as_millis() as f64 / self.frame_count as f64
        } else {
            0.0
        };

        // Calculate capture FPS from timestamps
        let capture_fps = if self.frame_timestamps.len() >= 2 {
            let first = self.frame_timestamps.front().unwrap();
            let last = self.frame_timestamps.back().unwrap();
            let duration = last.duration_since(*first).as_secs_f64();
            if duration > 0.0 {
                (self.frame_timestamps.len() - 1) as f64 / duration
            } else {
                0.0
            }
        } else {
            0.0
        };

        // Calculate encode FPS
        let encode_fps = if avg_encode_time > 0.0 {
            1000.0 / avg_encode_time
        } else {
            0.0
        };

        json!({
            "capture_fps": capture_fps,
            "encode_fps": encode_fps,
            "dropped_frames": self.dropped_frames,
            "encode_latency": avg_encode_time,
            // CPU and memory will be added by session manager
            "cpu_usage": 0.0,
            "memory_usage": 0,
        })
    }
}

