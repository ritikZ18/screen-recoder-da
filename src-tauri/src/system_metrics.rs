use serde_json::json;
use std::sync::Arc;
use sysinfo::{System, Pid};
use tokio::sync::Mutex;
use tokio::time::{Duration, Instant};

pub struct SystemMetrics {
    system: Arc<Mutex<System>>,
    last_update: Arc<Mutex<Instant>>,
    process_id: Option<u32>,
}

impl SystemMetrics {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        
        Self {
            system: Arc::new(Mutex::new(system)),
            last_update: Arc::new(Mutex::new(Instant::now())),
            process_id: Some(std::process::id()),
        }
    }

    pub async fn get_metrics(&self) -> serde_json::Value {
        let mut system: tokio::sync::MutexGuard<'_, System> = self.system.lock().await;
        let mut last_update = self.last_update.lock().await;

        // Refresh every 500ms to avoid excessive CPU usage
        if last_update.elapsed() > Duration::from_millis(500) {
            system.refresh_cpu();
            system.refresh_memory();
            *last_update = Instant::now();
        }

        // Get CPU usage (as percentage, capped at 100%)
        let cpu_usage = if let Some(process_id) = self.process_id {
            if let Some(process) = system.process(Pid::from_u32(process_id)) {
                // cpu_usage() returns percentage, clamp to 0-100
                (process.cpu_usage() as f64).min(100.0).max(0.0)
            } else {
                0.0
            }
        } else {
            // Global CPU usage - average across all CPUs
            let cpus = system.cpus();
            if !cpus.is_empty() {
                let total: f32 = cpus.iter().map(|cpu: &sysinfo::Cpu| cpu.cpu_usage()).sum();
                ((total / cpus.len() as f32) as f64).min(100.0).max(0.0)
            } else {
                0.0
            }
        };

        // Get memory usage (in MB)
        let memory_usage_mb = if let Some(process_id) = self.process_id {
            if let Some(process) = system.process(Pid::from_u32(process_id)) {
                // memory() returns bytes, convert to MB
                (process.memory() as f64 / 1024.0 / 1024.0)
            } else {
                0.0
            }
        } else {
            // Convert bytes to MB
            (system.used_memory() as f64 / 1024.0 / 1024.0)
        };

        json!({
            "cpu_usage": cpu_usage,
            "memory_usage": memory_usage_mb,
        })
    }
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self::new()
    }
}

