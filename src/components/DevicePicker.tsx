import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./DevicePicker.css";

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

  useEffect(() => {
    loadDevices();
  }, []);

  const loadDevices = async () => {
    try {
      setLoading(true);
      const [monitorsData, windowsData] = await Promise.all([
        invoke<Monitor[]>("list_monitors"),
        invoke<Window[]>("list_windows"),
      ]);
      setMonitors(monitorsData);
      setWindows(windowsData);
    } catch (error) {
      console.error("Failed to load devices:", error);
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
              {monitors.map((monitor) => (
                <div
                  key={monitor.id}
                  className={`device-item ${
                    selectedMonitor === monitor.id ? "selected" : ""
                  }`}
                  onClick={() => onMonitorSelect(monitor.id)}
                >
                  <div className="device-name">{monitor.name}</div>
                  <div className="device-info">
                    {monitor.width} × {monitor.height}
                  </div>
                </div>
              ))}
            </div>
          ) : (
            <div className="device-list">
              {windows.map((window) => (
                <div
                  key={window.id}
                  className={`device-item ${
                    selectedWindow === window.id ? "selected" : ""
                  }`}
                  onClick={() => onWindowSelect(window.id)}
                >
                  <div className="device-name">{window.title}</div>
                </div>
              ))}
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

