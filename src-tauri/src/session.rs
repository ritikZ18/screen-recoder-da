use crate::analytics::AnalyticsPipeline;
use crate::capture::{CaptureSource, CaptureTrait};
use crate::encoder::Encoder;
use crate::observability;
use crate::system_metrics::SystemMetrics;
use anyhow::{Context, Result};
use serde_json::Value;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;
use tokio::time::{Duration, Instant};


#[derive(Clone, Copy, PartialEq)]
pub enum RecordingState {
    Stopped,
    Recording,
    Paused,
}

pub struct SessionManager {
    state: Arc<Mutex<RecordingState>>,
    capture_source: Option<Arc<Mutex<crate::capture::Capture>>>,
    encoder: Option<Arc<Mutex<Encoder>>>,
    analytics: Option<Arc<Mutex<AnalyticsPipeline>>>,
    system_metrics: Arc<SystemMetrics>,
    start_time: Option<Instant>,
    paused_duration: Arc<Mutex<Duration>>,
    output_path: Option<PathBuf>,
    capture_task: Option<tokio::task::JoinHandle<()>>,
}

impl SessionManager {
    pub async fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(RecordingState::Stopped)),
            capture_source: None,
            encoder: None,
            analytics: None,
            system_metrics: Arc::new(SystemMetrics::new()),
            start_time: None,
            paused_duration: Arc::new(Mutex::new(Duration::ZERO)),
            output_path: None,
            capture_task: None,
        }
    }

    pub async fn list_monitors(&self) -> Result<Vec<Value>, String> {
        #[cfg(windows)]
        {
            crate::capture::windows::list_monitors()
                .await
                .map_err(|e| e.to_string())
        }
        #[cfg(not(windows))]
        {
            Ok(vec![])
        }
    }

    pub async fn list_windows(&self) -> Result<Vec<Value>, String> {
        #[cfg(windows)]
        {
            crate::capture::windows::list_windows()
                .await
                .map_err(|e| e.to_string())
        }
        #[cfg(not(windows))]
        {
            Ok(vec![])
        }
    }

    pub async fn start_recording(
        &mut self,
        monitor_id: Option<String>,
        window_id: Option<String>,
        app: AppHandle,
    ) -> Result<()> {
        let current_state = *self.state.lock().await;
        if current_state == RecordingState::Recording {
            return Err(anyhow::anyhow!("Recording already in progress").into());
        }

        observability::record_event("recording_started", &[]);

        // Determine capture source
        let source = if let Some(mon_id) = monitor_id {
            CaptureSource::Monitor(mon_id)
        } else if let Some(win_id) = window_id {
            CaptureSource::Window(win_id)
        } else {
            return Err(anyhow::anyhow!("No capture source specified").into());
        };

        // Initialize capture
        let mut capture = crate::capture::create_capture(source)
            .await
            .context("Failed to create capture source")?;

        capture
            .initialize()
            .await
            .context("Failed to initialize capture")?;

        // Generate output path
        let output_path = self.generate_output_path()?;

        // Initialize encoder
        let mut encoder = Encoder::new(output_path.clone())
            .await
            .context("Failed to create encoder")?;

        encoder
            .initialize()
            .await
            .context("Failed to initialize encoder")?;

        // Initialize analytics
        let analytics = AnalyticsPipeline::new();

        // Wrap in Arc<Mutex> for shared access
        let capture_arc = Arc::new(Mutex::new(capture));
        let encoder_arc = Arc::new(Mutex::new(encoder));
        let analytics_arc = Arc::new(Mutex::new(analytics));
        let metrics_arc = self.system_metrics.clone();

        self.capture_source = Some(capture_arc.clone());
        self.encoder = Some(encoder_arc.clone());
        self.analytics = Some(analytics_arc.clone());
        *self.state.lock().await = RecordingState::Recording;
        self.start_time = Some(Instant::now());
        *self.paused_duration.lock().await = Duration::ZERO;
        self.output_path = Some(output_path.clone());

        // Start capture loop in background task
        let app_clone = app.clone();
        let start_time_clone = self.start_time.unwrap();
        let paused_duration_arc = self.paused_duration.clone();
        let state_arc_clone = self.state.clone();
        
        let task = tokio::spawn(async move {
            Self::capture_loop_task(
                capture_arc,
                encoder_arc,
                analytics_arc,
                state_arc_clone,
                metrics_arc,
                app_clone,
                start_time_clone,
                paused_duration_arc,
            )
            .await;
        });

        self.capture_task = Some(task);
        
        // Store state reference for pause/resume
        // Note: In a production system, you'd use channels or shared state properly

        // Emit started event
        let _ = app.emit(
            "recording-update",
            serde_json::json!({
                "is_recording": true,
                "is_paused": false,
                "duration": 0.0,
            }),
        );

        Ok(())
    }

    pub async fn stop_recording(&mut self) -> Result<String> {
        let current_state = *self.state.lock().await;
        if current_state == RecordingState::Stopped {
            return Err(anyhow::anyhow!("No recording in progress").into());
        }

        observability::record_event("recording_stopped", &[]);

        // Signal capture loop to stop
        *self.state.lock().await = RecordingState::Stopped;

        // Wait for capture task to finish
        if let Some(task) = self.capture_task.take() {
            let _ = task.await;
        }

        // Stop capture
        if let Some(capture) = self.capture_source.take() {
            let mut capture_guard = capture.lock().await;
            capture_guard.stop().await?;
        }

        // Finalize encoder
        if let Some(encoder) = self.encoder.take() {
            let mut encoder_guard = encoder.lock().await;
            encoder_guard.finalize().await?;
        }

        // Save analytics
        if let Some(analytics) = self.analytics.take() {
            let analytics_guard = analytics.lock().await;
            if let Some(ref path) = self.output_path {
                analytics_guard.save_metadata(path).await?;
            }
        }

        let output = self
            .output_path
            .as_ref()
            .and_then(|p| p.to_str())
            .unwrap_or("")
            .to_string();

        self.start_time = None;
        *self.paused_duration.lock().await = Duration::ZERO;

        Ok(output)
    }

    pub async fn pause_recording(&mut self) -> Result<()> {
        let mut state = self.state.lock().await;
        match *state {
            RecordingState::Recording => {
                *state = RecordingState::Paused;
                observability::record_event("recording_paused", &[]);
                Ok(())
            }
            RecordingState::Paused => {
                *state = RecordingState::Recording;
                observability::record_event("recording_resumed", &[]);
                Ok(())
            }
            RecordingState::Stopped => Err(anyhow::anyhow!("No recording in progress").into()),
        }
    }

    pub async fn get_timeline_data(&self) -> Result<Vec<Value>, String> {
        if let Some(analytics) = &self.analytics {
            let analytics_guard = analytics.lock().await;
            Ok(analytics_guard.get_timeline_data().await)
        } else {
            Ok(vec![])
        }
    }

    async fn capture_loop_task(
        capture: Arc<Mutex<crate::capture::Capture>>,
        encoder: Arc<Mutex<Encoder>>,
        analytics: Arc<Mutex<AnalyticsPipeline>>,
        state: Arc<Mutex<RecordingState>>,
        system_metrics: Arc<SystemMetrics>,
        app: AppHandle,
        start_time: Instant,
        paused_duration: Arc<Mutex<Duration>>,
    ) {
        let mut frame_count = 0u64;
        let mut last_metrics_update = Instant::now();
        let mut last_state_update = Instant::now();
        let mut paused_start: Option<Instant> = None;

        loop {
            // Check if recording is paused or stopped
            let current_state = *state.lock().await;
            if current_state == RecordingState::Stopped {
                break;
            }

            if current_state == RecordingState::Paused {
                if paused_start.is_none() {
                    paused_start = Some(Instant::now());
                }
                tokio::time::sleep(Duration::from_millis(100)).await;
                continue;
            }

            // Resume from pause
            if let Some(pause_start_time) = paused_start.take() {
                let mut paused_dur = paused_duration.lock().await;
                *paused_dur += pause_start_time.elapsed();
            }

            // Capture frame
            let frame_result = {
                let mut capture_guard = capture.lock().await;
                capture_guard.capture_frame().await
            };

            match frame_result {
                Ok(Some(frame)) => {
                    frame_count += 1;

                    // Process analytics
                    {
                        let mut analytics_guard = analytics.lock().await;
                        analytics_guard.process_frame(&frame).await;
                    }

                    // Encode frame
                    {
                        let mut encoder_guard = encoder.lock().await;
                        if let Err(e) = encoder_guard.encode_frame(&frame).await {
                            tracing::error!("Encoding error: {}", e);
                            break;
                        }
                    }

                    // Update metrics periodically
                    if last_metrics_update.elapsed() > Duration::from_secs(1) {
                        let mut metrics = {
                            let encoder_guard = encoder.lock().await;
                            encoder_guard.get_metrics()
                        };

                        // Add system metrics
                        let sys_metrics = system_metrics.get_metrics().await;
                        metrics["cpu_usage"] = sys_metrics["cpu_usage"].clone();
                        metrics["memory_usage"] = sys_metrics["memory_usage"].clone();

                        let _ = app.emit("metrics-update", metrics);
                        last_metrics_update = Instant::now();
                    }
                }
                Ok(None) => {
                    // No frame available, continue
                    tokio::time::sleep(Duration::from_millis(16)).await;
                }
                Err(e) => {
                    tracing::error!("Capture error: {}", e);
                    // Don't break on capture errors, just log and continue
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            }

            // Emit state update periodically
            if last_state_update.elapsed() > Duration::from_millis(500) {
                let paused_dur = *paused_duration.lock().await;
                let duration = if paused_start.is_some() {
                    // Currently paused
                    start_time.elapsed().as_secs_f64() - paused_dur.as_secs_f64()
                } else {
                    start_time.elapsed().as_secs_f64() - paused_dur.as_secs_f64()
                };

                let _ = app.emit(
                    "recording-update",
                    serde_json::json!({
                        "is_recording": current_state == RecordingState::Recording,
                        "is_paused": current_state == RecordingState::Paused,
                        "duration": duration,
                    }),
                );
                last_state_update = Instant::now();
            }
        }

        tracing::info!("Capture loop finished after {} frames", frame_count);
    }

    fn generate_output_path(&self) -> Result<PathBuf> {
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let filename = format!("recording_{}.mkv", timestamp);
        let mut path = dirs::video_dir()
            .or_else(|| dirs::home_dir())
            .ok_or_else(|| anyhow::anyhow!("Could not determine output directory"))?;
        path.push("ScreenRecordings");
        std::fs::create_dir_all(&path)?;
        path.push(filename);
        Ok(path)
    }
}

