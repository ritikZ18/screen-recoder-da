use anyhow::Result;

#[cfg(windows)]
pub mod windows;

#[derive(Clone, Debug)]
pub enum CaptureSource {
    Monitor(String),
    Window(String),
}

#[async_trait::async_trait]
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

// Use enum instead of dyn trait for async compatibility
// Capture is Send + Sync because WindowsCapture implements Send + Sync
#[allow(clippy::large_enum_variant)]
pub enum Capture {
    #[cfg(windows)]
    Windows(windows::WindowsCapture),
}

impl Capture {
    pub async fn new(_source: CaptureSource) -> Result<Self> {
        #[cfg(windows)]
        {
            Ok(Capture::Windows(windows::WindowsCapture::new(_source).await?))
        }
        #[cfg(not(windows))]
        {
            Err(anyhow::anyhow!("Capture not implemented for this platform"))
        }
    }
}

#[async_trait::async_trait]
impl CaptureTrait for Capture {
    async fn initialize(&mut self) -> Result<()> {
        #[cfg(windows)]
        {
            match self {
                Capture::Windows(c) => c.initialize().await,
            }
        }
        #[cfg(not(windows))]
        {
            Err(anyhow::anyhow!("Capture not implemented for this platform"))
        }
    }

    async fn capture_frame(&mut self) -> Result<Option<Frame>> {
        #[cfg(windows)]
        {
            match self {
                Capture::Windows(c) => c.capture_frame().await,
            }
        }
        #[cfg(not(windows))]
        {
            Err(anyhow::anyhow!("Capture not implemented for this platform"))
        }
    }

    async fn stop(&mut self) -> Result<()> {
        #[cfg(windows)]
        {
            match self {
                Capture::Windows(c) => c.stop().await,
            }
        }
        #[cfg(not(windows))]
        {
            Err(anyhow::anyhow!("Capture not implemented for this platform"))
        }
    }
}

pub async fn create_capture(source: CaptureSource) -> Result<Capture> {
    Capture::new(source).await
}

