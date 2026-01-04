import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./DevicePicker.css";

// Check if running in Tauri
const isTauri = typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

interface Monitor {
  id: string;
  name: string;
  width: number;
  height: number;
}

interface Window {
  id: string;
  title: string;
}

interface DevicePickerProps {
  selectedMonitor: string | null;
  selectedWindow: string | null;
  onMonitorSelect: (id: string | null) => void;
  onWindowSelect: (id: string | null) => void;
}

function DevicePicker({
  selectedMonitor,
  selectedWindow,
  onMonitorSelect,
  onWindowSelect,
}: DevicePickerProps) {
  const [monitors, setMonitors] = useState<Monitor[]>([]);
  const [windows, setWindows] = useState<Window[]>([]);
  const [loading, setLoading] = useState(true);
  const [captureMode, setCaptureMode] = useState<"monitor" | "window">("monitor");

  // Debug: Log state changes
  useEffect(() => {
    console.log("Monitors state updated:", monitors.length, monitors);
  }, [monitors]);

  useEffect(() => {
    console.log("Windows state updated:", windows.length, windows);
  }, [windows]);

  useEffect(() => {
    loadDevices();
  }, []);

  const loadDevices = async () => {
    if (!isTauri) {
      console.warn("Tauri APIs not available - running in web mode");
      setLoading(false);
      return;
    }
    try {
      setLoading(true);
      console.log("Loading monitors and windows...");
      
      const monitorsResult = await invoke<any>("list_monitors").catch((e) => {
        console.error("Error loading monitors:", e);
        return [];
      });
      
      const windowsResult = await invoke<any>("list_windows").catch((e) => {
        console.error("Error loading windows:", e);
        return [];
      });
      
      console.log("Monitors loaded:", monitorsResult);
      console.log("Windows loaded:", windowsResult);
      console.log("Monitors type:", typeof monitorsResult, "Is array:", Array.isArray(monitorsResult));
      console.log("Windows type:", typeof windowsResult, "Is array:", Array.isArray(windowsResult));
      
      // Ensure we have arrays
      const monitorsData = Array.isArray(monitorsResult) ? monitorsResult : [];
      const windowsData = Array.isArray(windowsResult) ? windowsResult : [];
      
      console.log("Setting monitors:", monitorsData.length, "monitors");
      console.log("Setting windows:", windowsData.length, "windows");
      
      setMonitors(monitorsData);
      setWindows(windowsData);
      
      // Log state after setting
      setTimeout(() => {
        console.log("State after update - monitors:", monitorsData);
        console.log("State after update - windows:", windowsData);
      }, 100);
      
      if (monitorsData.length === 0 && windowsData.length === 0) {
        console.warn("No monitors or windows found. Check backend implementation.");
      }
    } catch (error) {
      console.error("Failed to load devices:", error);
      // Show error to user
      alert(`Failed to load devices: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="device-picker">
      <h2>Capture Source</h2>

      <div className="mode-selector">
        <button
          className={captureMode === "monitor" ? "active" : ""}
          onClick={() => {
            setCaptureMode("monitor");
            onWindowSelect(null);
          }}
        >
          Monitor
        </button>
        <button
          className={captureMode === "window" ? "active" : ""}
          onClick={() => {
            setCaptureMode("window");
            onMonitorSelect(null);
          }}
        >
          Window
        </button>
      </div>

      {loading ? (
        <div className="loading">Loading devices...</div>
      ) : (
        <>
          {captureMode === "monitor" ? (
            <div className="device-list">
              {loading ? (
                <div className="loading">Loading monitors...</div>
              ) : monitors.length === 0 ? (
                <div className="device-empty">
                  <p>No monitors found.</p>
                  {isTauri ? (
                    <p className="info-text">
                      Try clicking "Refresh" or check the browser console (F12) for errors.
                    </p>
                  ) : (
                    <p className="info-text">
                      Run <code>npm run tauri dev</code> to use the desktop app.
                    </p>
                  )}
                </div>
              ) : (
                <>
                  <div style={{ color: '#fff', marginBottom: '0.5rem', fontSize: '0.9rem', padding: '0.5rem' }}>
                    Found {monitors.length} monitor{monitors.length !== 1 ? 's' : ''}
                  </div>
                  {monitors.map((monitor) => {
                    console.log("Rendering monitor:", monitor);
                    return (
                      <div
                        key={monitor.id}
                        className={`device-item ${
                          selectedMonitor === monitor.id ? "selected" : ""
                        }`}
                        onClick={() => {
                          console.log("Monitor clicked:", monitor.id);
                          onMonitorSelect(monitor.id);
                        }}
                        style={{ marginBottom: '0.5rem' }}
                      >
                        <div className="device-name">{monitor.name || 'Unnamed Monitor'}</div>
                        <div className="device-info">
                          {monitor.width} × {monitor.height}
                        </div>
                      </div>
                    );
                  })}
                </>
              )}
            </div>
          ) : (
            <div className="device-list">
              {loading ? (
                <div className="loading">Loading windows...</div>
              ) : windows.length === 0 ? (
                <div className="device-empty">
                  <p>No windows found.</p>
                  {isTauri ? (
                    <p className="info-text">
                      Try clicking "Refresh" or check the browser console (F12) for errors.
                    </p>
                  ) : (
                    <p className="info-text">
                      Run <code>npm run tauri dev</code> to use the desktop app.
                    </p>
                  )}
                </div>
              ) : (
                <>
                  <div style={{ color: '#fff', marginBottom: '0.5rem', fontSize: '0.9rem', padding: '0.5rem' }}>
                    Found {windows.length} window{windows.length !== 1 ? 's' : ''}
                  </div>
                  {windows.slice(0, 50).map((window) => {
                    console.log("Rendering window:", window);
                    return (
                      <div
                        key={window.id}
                        className={`device-item ${
                          selectedWindow === window.id ? "selected" : ""
                        }`}
                        onClick={() => {
                          console.log("Window clicked:", window.id);
                          onWindowSelect(window.id);
                        }}
                        style={{ marginBottom: '0.5rem' }}
                      >
                        <div className="device-name">{window.title || `Window ${window.id}`}</div>
                      </div>
                    );
                  })}
                  {windows.length > 50 && (
                    <div style={{ color: '#a0a0a0', padding: '0.5rem', fontSize: '0.875rem' }}>
                      ... and {windows.length - 50} more windows
                    </div>
                  )}
                </>
              )}
            </div>
          )}
        </>
      )}

      <button className="refresh-btn" onClick={loadDevices}>
        Refresh
      </button>
    </div>
  );
}

export default DevicePicker;

