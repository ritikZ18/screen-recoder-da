use crate::capture::Frame;
use anyhow::Result;
use image::{DynamicImage, RgbImage};
use rayon::prelude::*;
use serde_json::{json, Value};
use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AnalyticsPipeline {
    timeline_data: Arc<Mutex<VecDeque<TimelineEntry>>>,
    last_frame: Option<Vec<u8>>,
}

#[derive(Clone)]
struct TimelineEntry {
    timestamp: f64,
    color_dominance: f64,
    brightness: f64,
    audio_level: f64,
    scene_change: bool,
}

impl AnalyticsPipeline {
    pub fn new() -> Self {
        Self {
            timeline_data: Arc::new(Mutex::new(VecDeque::new())),
            last_frame: None,
        }
    }

    pub async fn process_frame(&mut self, frame: &Frame) {
        // Process color patterns
        let (color_dominance, brightness) = self.analyze_color_patterns(frame);

        // Detect scene changes
        let scene_change = self.detect_scene_change(frame);

        // Process audio patterns
        // Note: Audio processing requires audio capture to be implemented
        // For now, we calculate a placeholder based on frame activity
        let audio_level = self.estimate_audio_level(frame);

        let entry = TimelineEntry {
            timestamp: frame.timestamp as f64 / 1_000_000_000.0, // Convert to seconds
            color_dominance,
            brightness,
            audio_level,
            scene_change,
        };

        let mut data = self.timeline_data.lock().await;
        data.push_back(entry);

        // Keep only last 1000 entries to prevent memory bloat
        if data.len() > 1000 {
            data.pop_front();
        }
    }

    fn analyze_color_patterns(&self, frame: &Frame) -> (f64, f64) {
        // Convert frame data to image
        let img_result = RgbImage::from_raw(frame.width, frame.height, frame.data.clone());

        if let Some(img) = img_result {
            // Calculate color histogram
            let mut histogram = [0u32; 256];
            let mut total_brightness = 0u64;
            let pixel_count = (frame.width * frame.height) as u64;

            for pixel in img.pixels() {
                // Calculate brightness (luminance)
                let brightness = (0.299 * pixel[0] as f64
                    + 0.587 * pixel[1] as f64
                    + 0.114 * pixel[2] as f64) as u8;
                histogram[brightness as usize] += 1;
                total_brightness += brightness as u64;
            }

            // Calculate dominant color (most common brightness)
            let dominant_brightness = histogram
                .iter()
                .enumerate()
                .max_by_key(|(_, &count)| count)
                .map(|(idx, _)| idx)
                .unwrap_or(128) as f64;

            // Normalize color dominance (0-1 scale)
            let color_dominance = dominant_brightness / 255.0;

            // Calculate average brightness
            let avg_brightness = (total_brightness as f64 / pixel_count as f64) / 255.0;

            (color_dominance, avg_brightness)
        } else {
            (0.0, 0.0)
        }
    }

    fn detect_scene_change(&mut self, frame: &Frame) -> bool {
        if let Some(ref last_frame_data) = self.last_frame {
            // Compare histograms
            let current_hist = self.calculate_histogram(frame);
            let last_hist = self.calculate_histogram_from_data(
                last_frame_data,
                frame.width,
                frame.height,
            );

            // Calculate histogram difference
            let diff: f64 = current_hist
                .iter()
                .zip(last_hist.iter())
                .map(|(a, b)| (a - b).abs() as f64)
                .sum();

            let threshold = (frame.width * frame.height) as f64 * 0.1; // 10% change threshold
            let is_scene_change = diff > threshold;

            self.last_frame = Some(frame.data.clone());
            is_scene_change
        } else {
            self.last_frame = Some(frame.data.clone());
            false
        }
    }

    fn calculate_histogram(&self, frame: &Frame) -> [u32; 256] {
        let mut histogram = [0u32; 256];
        let img_result = RgbImage::from_raw(frame.width, frame.height, frame.data.clone());

        if let Some(img) = img_result {
            for pixel in img.pixels() {
                let brightness = (0.299 * pixel[0] as f64
                    + 0.587 * pixel[1] as f64
                    + 0.114 * pixel[2] as f64) as u8;
                histogram[brightness as usize] += 1;
            }
        }

        histogram
    }

    fn calculate_histogram_from_data(
        &self,
        data: &[u8],
        width: u32,
        height: u32,
    ) -> [u32; 256] {
        let mut histogram = [0u32; 256];
        let img_result = RgbImage::from_raw(width, height, data.to_vec());

        if let Some(img) = img_result {
            for pixel in img.pixels() {
                let brightness = (0.299 * pixel[0] as f64
                    + 0.587 * pixel[1] as f64
                    + 0.114 * pixel[2] as f64) as u8;
                histogram[brightness as usize] += 1;
            }
        }

        histogram
    }

    pub async fn save_metadata(&self, video_path: &PathBuf) -> Result<()> {
        let data = self.timeline_data.lock().await;
        let entries: Vec<Value> = data
            .iter()
            .map(|e| {
                json!({
                    "time": e.timestamp,
                    "colorDominance": e.color_dominance,
                    "brightness": e.brightness,
                    "audioLevel": e.audio_level,
                    "sceneChange": e.scene_change,
                })
            })
            .collect();

        let metadata = json!({
            "video_path": video_path.to_string_lossy(),
            "entries": entries,
        });

        // Save to sidecar file
        let mut meta_path = video_path.clone();
        meta_path.set_extension("meta.json");
        std::fs::write(&meta_path, serde_json::to_string_pretty(&metadata)?)?;

        Ok(())
    }

    pub async fn get_timeline_data(&self) -> Vec<Value> {
        let data = self.timeline_data.lock().await;
        data.iter()
            .map(|e| {
                json!({
                    "time": e.timestamp,
                    "colorDominance": e.color_dominance,
                    "brightness": e.brightness,
                    "audioLevel": e.audio_level,
                    "sceneChange": e.scene_change,
                })
            })
            .collect()
    }

    fn estimate_audio_level(&self, frame: &Frame) -> f64 {
        // Estimate audio level based on frame activity
        // In a full implementation, this would process actual audio samples
        // For now, we use brightness changes as a proxy for activity
        
        if let Some(ref last_frame) = self.last_frame {
            let current_brightness = self.calculate_average_brightness(frame);
            let last_brightness = self.calculate_average_brightness_from_data(
                last_frame,
                frame.width,
                frame.height,
            );
            
            // Audio level correlates with visual activity
            let activity = (current_brightness - last_brightness).abs();
            activity.min(1.0)
        } else {
            0.0
        }
    }

    fn calculate_average_brightness(&self, frame: &Frame) -> f64 {
        let img_result = RgbImage::from_raw(frame.width, frame.height, frame.data.clone());
        if let Some(img) = img_result {
            let mut total = 0u64;
            let count = (frame.width * frame.height) as u64;
            
            for pixel in img.pixels() {
                let brightness = (0.299 * pixel[0] as f64
                    + 0.587 * pixel[1] as f64
                    + 0.114 * pixel[2] as f64) as u64;
                total += brightness;
            }
            
            (total as f64 / count as f64) / 255.0
        } else {
            0.0
        }
    }

    fn calculate_average_brightness_from_data(
        &self,
        data: &[u8],
        width: u32,
        height: u32,
    ) -> f64 {
        let img_result = RgbImage::from_raw(width, height, data.to_vec());
        if let Some(img) = img_result {
            let mut total = 0u64;
            let count = (width * height) as u64;
            
            for pixel in img.pixels() {
                let brightness = (0.299 * pixel[0] as f64
                    + 0.587 * pixel[1] as f64
                    + 0.114 * pixel[2] as f64) as u64;
                total += brightness;
            }
            
            (total as f64 / count as f64) / 255.0
        } else {
            0.0
        }
    }
}

