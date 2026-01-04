use crate::capture::{CaptureSource, CaptureTrait, Frame};
use anyhow::Result;
use serde_json::Value;
use std::ffi::c_void;
use windows::Win32::{
    Foundation::*,
    Graphics::Gdi::*,
    System::Com::*,
    UI::WindowsAndMessaging::*,
};

// Windows handles are safe to send between threads on Windows
// They're just opaque pointers, not actual references
unsafe impl Send for WindowsCapture {}
unsafe impl Sync for WindowsCapture {}

pub struct WindowsCapture {
    source: CaptureSource,
    is_initialized: bool,
    hwnd: Option<HWND>,
    capture_width: u32,
    capture_height: u32,
    hdc_source: Option<HDC>,
    hdc_mem: Option<HDC>,
    hbitmap: Option<HBITMAP>,
    monitor_offset_x: Option<i32>,
    monitor_offset_y: Option<i32>,
}

impl WindowsCapture {
    pub async fn new(source: CaptureSource) -> Result<Self> {
        Ok(Self {
            source,
            is_initialized: false,
            hwnd: None,
            capture_width: 0,
            capture_height: 0,
            hdc_source: None,
            hdc_mem: None,
            hbitmap: None,
            monitor_offset_x: None,
            monitor_offset_y: None,
        })
    }

    fn get_monitor_hwnd() -> Result<HWND> {
        unsafe {
            let hwnd = GetDesktopWindow();
            if hwnd.is_invalid() {
                return Err(anyhow::anyhow!("Failed to get desktop window"));
            }
            Ok(hwnd)
        }
    }

    fn parse_window_handle(window_id: &str) -> Result<HWND> {
        // Window ID is format like "0x170c8a"
        let handle_str = window_id.trim_start_matches("0x");
        let handle_value = u64::from_str_radix(handle_str, 16)
            .map_err(|e| anyhow::anyhow!("Failed to parse window handle: {}", e))?;
        Ok(HWND(handle_value as *mut c_void))
    }

    fn parse_monitor_handle(monitor_id: &str) -> Result<HMONITOR> {
        use windows::Win32::Graphics::Gdi::*;
        // Monitor ID is format like "0x12345"
        let handle_str = monitor_id.trim_start_matches("0x");
        let handle_value = usize::from_str_radix(handle_str, 16)
            .map_err(|e| anyhow::anyhow!("Failed to parse monitor handle: {}", e))?;
        Ok(HMONITOR(handle_value as *mut c_void))
    }
}

#[async_trait::async_trait]
impl CaptureTrait for WindowsCapture {
    async fn initialize(&mut self) -> Result<()> {
        // Initialize COM
        unsafe {
            use windows::Win32::System::Com::COINIT_MULTITHREADED;
            let result = CoInitializeEx(None, COINIT_MULTITHREADED);
            if result.is_err() {
                return Err(anyhow::anyhow!("Failed to initialize COM: {:?}", result));
            }
        }

        // Determine HWND and dimensions based on source
        let (hwnd, width, height, _monitor_offset) = match &self.source {
            CaptureSource::Monitor(monitor_id) => {
                // Parse monitor handle from ID (format: "0x12345")
                let hmonitor = Self::parse_monitor_handle(monitor_id)?;
                let hwnd = Self::get_monitor_hwnd()?;
                unsafe {
                    use windows::Win32::Graphics::Gdi::*;
                    
                    let mut monitor_info = MONITORINFO {
                        cbSize: std::mem::size_of::<MONITORINFO>() as u32,
                        rcMonitor: RECT::default(),
                        rcWork: RECT::default(),
                        dwFlags: 0,
                    };
                    
                    if !GetMonitorInfoA(hmonitor, &mut monitor_info).as_bool() {
                        return Err(anyhow::anyhow!("Failed to get monitor info"));
                    }
                    
                    let width = (monitor_info.rcMonitor.right - monitor_info.rcMonitor.left) as u32;
                    let height = (monitor_info.rcMonitor.bottom - monitor_info.rcMonitor.top) as u32;
                    let offset_x = monitor_info.rcMonitor.left;
                    let offset_y = monitor_info.rcMonitor.top;
                    
                    // Store monitor offset for capture
                    self.monitor_offset_x = Some(offset_x);
                    self.monitor_offset_y = Some(offset_y);
                    
                    (hwnd, width, height, (offset_x, offset_y))
                }
            }
            CaptureSource::Window(window_id) => {
                let hwnd = Self::parse_window_handle(window_id)?;
                unsafe {
                    let mut rect = RECT::default();
                    if GetWindowRect(hwnd, &mut rect).is_err() {
                        return Err(anyhow::anyhow!("Failed to get window rectangle"));
                    }
                    let width = (rect.right - rect.left) as u32;
                    let height = (rect.bottom - rect.top) as u32;
                    self.monitor_offset_x = None;
                    self.monitor_offset_y = None;
                    (hwnd, width, height, (0, 0))
                }
            }
        };

        self.hwnd = Some(hwnd);
        self.capture_width = width;
        self.capture_height = height;

        // Set up GDI capture buffers
        unsafe {
            let hdc_source = GetDC(hwnd);
            if hdc_source.0.is_null() {
                return Err(anyhow::anyhow!("Failed to get source device context"));
            }

            let hdc_mem = CreateCompatibleDC(hdc_source);
            if hdc_mem.is_invalid() {
                ReleaseDC(hwnd, hdc_source);
                return Err(anyhow::anyhow!("Failed to create compatible DC"));
            }

            let hbitmap = CreateCompatibleBitmap(hdc_source, width as i32, height as i32);
            if hbitmap.is_invalid() {
                DeleteDC(hdc_mem);
                ReleaseDC(hwnd, hdc_source);
                return Err(anyhow::anyhow!("Failed to create compatible bitmap"));
            }

            let _old_bitmap = SelectObject(hdc_mem, hbitmap);

            self.hdc_source = Some(hdc_source);
            self.hdc_mem = Some(hdc_mem);
            self.hbitmap = Some(hbitmap);
        }

        self.is_initialized = true;
        tracing::info!(
            "Windows capture initialized for source: {:?}, size: {}x{}",
            self.source,
            width,
            height
        );
        Ok(())
    }

    async fn capture_frame(&mut self) -> Result<Option<Frame>> {
        if !self.is_initialized {
            return Err(anyhow::anyhow!("Capture not initialized"));
        }

        let _hwnd = self.hwnd.ok_or_else(|| anyhow::anyhow!("HWND not set"))?;
        let hdc_source = self
            .hdc_source
            .ok_or_else(|| anyhow::anyhow!("Source DC not set"))?;
        let hdc_mem = self
            .hdc_mem
            .ok_or_else(|| anyhow::anyhow!("Memory DC not set"))?;
        let hbitmap = self
            .hbitmap
            .ok_or_else(|| anyhow::anyhow!("Bitmap not set"))?;

        // Capture screen using BitBlt
        // For monitors, use offset to capture the correct region
        let src_x = self.monitor_offset_x.unwrap_or(0);
        let src_y = self.monitor_offset_y.unwrap_or(0);
        
        unsafe {
            if BitBlt(
                hdc_mem,
                0,
                0,
                self.capture_width as i32,
                self.capture_height as i32,
                hdc_source,
                src_x,
                src_y,
                SRCCOPY,
            )
            .is_err()
            {
                return Err(anyhow::anyhow!("BitBlt failed"));
            }

            // Get bitmap info
            let mut bitmap_info = BITMAPINFO {
                bmiHeader: BITMAPINFOHEADER {
                    biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth: self.capture_width as i32,
                    biHeight: -(self.capture_height as i32), // Negative for top-down DIB
                    biPlanes: 1,
                    biBitCount: 32, // BGRA
                    biCompression: 0,
                    biSizeImage: 0,
                    biXPelsPerMeter: 0,
                    biYPelsPerMeter: 0,
                    biClrUsed: 0,
                    biClrImportant: 0,
                },
                bmiColors: std::array::from_fn(|_| RGBQUAD::default()),
            };

            // Calculate buffer size
            let stride = ((self.capture_width * 4 + 3) / 4) * 4; // 4-byte aligned
            let buffer_size = (stride * self.capture_height) as usize;

            // Allocate buffer for bitmap data
            let mut bitmap_data = vec![0u8; buffer_size];

            // Get bitmap bits - First call to fill bitmap info
            let result1 = GetDIBits(
                hdc_mem,
                hbitmap,
                0,
                0,
                None,
                &mut bitmap_info,
                DIB_RGB_COLORS,
            );

            if result1 == 0 {
                return Err(anyhow::anyhow!("GetDIBits (info) failed"));
            }

            // Second call to get pixel data
            let bitmap_ptr: *mut c_void = bitmap_data.as_mut_ptr() as *mut c_void;
            let result = GetDIBits(
                hdc_mem,
                hbitmap,
                0,
                self.capture_height as u32,
                Some(bitmap_ptr),
                &mut bitmap_info,
                DIB_RGB_COLORS,
            );

            if result == 0 {
                return Err(anyhow::anyhow!("GetDIBits failed"));
            }

            // Convert BGRA to RGB
            let mut rgb_data = Vec::with_capacity((self.capture_width * self.capture_height * 3) as usize);
            for chunk in bitmap_data.chunks(4) {
                if chunk.len() == 4 {
                    // BGRA format from GetDIBits
                    let b = chunk[0];
                    let g = chunk[1];
                    let r = chunk[2];
                    // Skip alpha channel
                    rgb_data.push(r);
                    rgb_data.push(g);
                    rgb_data.push(b);
                }
            }

            // Get current timestamp
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;

            Ok(Some(Frame {
                data: rgb_data,
                width: self.capture_width,
                height: self.capture_height,
                timestamp,
            }))
        }
    }

    async fn stop(&mut self) -> Result<()> {
        // Clean up GDI resources
        unsafe {
            if let Some(hbitmap) = self.hbitmap {
                if !hbitmap.is_invalid() {
                    DeleteObject(hbitmap);
                }
            }
            if let Some(hdc_mem) = self.hdc_mem {
                if !hdc_mem.is_invalid() {
                    DeleteDC(hdc_mem);
                }
            }
            if let Some(hwnd) = self.hwnd {
                if let Some(hdc_source) = self.hdc_source {
                    if !hdc_source.0.is_null() {
                        ReleaseDC(hwnd, hdc_source);
                    }
                }
            }
        }

        self.hdc_source = None;
        self.hdc_mem = None;
        self.hbitmap = None;
        self.hwnd = None;
        self.is_initialized = false;
        tracing::info!("Windows capture stopped");
        Ok(())
    }
}

pub async fn list_monitors() -> Result<Vec<Value>> {
    use windows::Win32::Graphics::Gdi::*;

    let mut monitors = Vec::new();

    unsafe extern "system" fn enum_proc(
        hmonitor: HMONITOR,
        _hdc: HDC,
        _lprect: *mut RECT,
        lparam: LPARAM,
    ) -> BOOL {
        let monitors_ptr = lparam.0 as *mut Vec<serde_json::Value>;
        let monitors = unsafe { &mut *monitors_ptr };

        let mut monitor_info = MONITORINFO {
            cbSize: std::mem::size_of::<MONITORINFO>() as u32,
            rcMonitor: RECT::default(),
            rcWork: RECT::default(),
            dwFlags: 0,
        };

        if GetMonitorInfoA(hmonitor, &mut monitor_info).as_bool() {
            let width = (monitor_info.rcMonitor.right - monitor_info.rcMonitor.left) as u32;
            let height = (monitor_info.rcMonitor.bottom - monitor_info.rcMonitor.top) as u32;
            let is_primary = (monitor_info.dwFlags & MONITORINFOF_PRIMARY) != 0;

            let monitor_name = if is_primary {
                "Primary Monitor".to_string()
            } else {
                format!("Monitor {}", monitors.len() + 1)
            };
            
            // Use monitor handle as ID - format as hex string for consistency
            let monitor_id = format!("0x{:x}", hmonitor.0 as usize);
            
            monitors.push(serde_json::json!({
                "id": monitor_id,
                "name": monitor_name,
                "width": width,
                "height": height,
            }));
        }

        TRUE
    }

    unsafe {
        EnumDisplayMonitors(None, None, Some(enum_proc), LPARAM(&mut monitors as *mut _ as isize));
    }

    tracing::info!("Found {} monitors", monitors.len());
    Ok(monitors)
}

pub async fn list_windows() -> Result<Vec<Value>> {
    use windows::Win32::{
        Foundation::*,
        UI::WindowsAndMessaging::*,
    };

    let mut windows = Vec::new();

    unsafe extern "system" fn enum_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let windows_ptr = lparam.0 as *mut Vec<serde_json::Value>;
        let windows = unsafe { &mut *windows_ptr };

        // Check if window is visible and has a title
        if IsWindowVisible(hwnd).as_bool() {
            let mut title = [0u16; 256];
            let len = GetWindowTextW(hwnd, &mut title);

            if len > 0 {
                use std::ffi::OsString;
                use std::os::windows::ffi::OsStringExt;
                let title_str: OsString = OsStringExt::from_wide(&title[..len as usize]);
                let title_str = title_str.to_string_lossy().to_string();

                // Get window dimensions
                let mut rect = RECT::default();
                if GetWindowRect(hwnd, &mut rect).is_ok() {
                    let width = (rect.right - rect.left) as u32;
                    let height = (rect.bottom - rect.top) as u32;

                    // Create window ID (using handle as string)
                    let window_id = format!("0x{:x}", hwnd.0 as u64);

                    windows.push(serde_json::json!({
                        "id": window_id,
                        "title": title_str,
                        "width": width,
                        "height": height,
                    }));
                }
            }
        }

        TRUE
    }

    unsafe {
        EnumWindows(
            Some(enum_proc),
            LPARAM(&mut windows as *mut _ as isize),
        )?;
    }

    tracing::info!("Found {} windows", windows.len());
    Ok(windows)
}
