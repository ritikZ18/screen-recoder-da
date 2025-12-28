use anyhow::Result;
use async_trait::async_trait;

pub use crate::capture::Frame;

pub mod windows;

#[derive(Clone, Debug)]
pub enum CaptureSource {
    Monitor(String),
    Window(String),
}

#[async_trait]
pub trait CaptureTrait: Send + Sync {
    async fn initialize(&mut self) -> Result<()>;
    async fn capture_frame(&mut self) -> Result<Option<Frame>>;
    async fn stop(&mut self) -> Result<()>;
}

pub struct Frame {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub timestamp: u64,
}

pub async fn create_capture(source: CaptureSource) -> Result<Box<dyn CaptureTrait>> {
    #[cfg(windows)]
    {
        Ok(Box::new(windows::WindowsCapture::new(source).await?))
    }
    #[cfg(not(windows))]
    {
        Err(anyhow::anyhow!("Capture not implemented for this platform"))
    }
}

