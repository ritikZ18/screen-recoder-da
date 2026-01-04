import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import DevicePicker from "./components/DevicePicker";
import RecordingControls from "./components/RecordingControls";
import Timeline from "./components/Timeline";
import MetricsPanel from "./components/MetricsPanel";
import "./App.css";

// Check if running in Tauri
const isTauri = typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

interface RecordingState {
  isRecording: boolean;
  isPaused: boolean;
  duration: number;
}

interface Metrics {
  captureFps: number;
  encodeFps: number;
  droppedFrames: number;
  encodeLatency: number;
  cpuUsage: number;
  memoryUsage: number;
}

function App() {
  console.log("App.tsx: Component rendering, isTauri:", typeof window !== "undefined" && "__TAURI_INTERNALS__" in window);
  
  const [recordingState, setRecordingState] = useState<RecordingState>({
    isRecording: false,
    isPaused: false,
    duration: 0,
  });
  const [metrics, setMetrics] = useState<Metrics>({
    captureFps: 0,
    encodeFps: 0,
    droppedFrames: 0,
    encodeLatency: 0,
    cpuUsage: 0,
    memoryUsage: 0,
  });
  const [selectedMonitor, setSelectedMonitor] = useState<string | null>(null);
  const [selectedWindow, setSelectedWindow] = useState<string | null>(null);
  
  console.log("App.tsx: State initialized");

  useEffect(() => {
    if (!isTauri) return;
    
    // Sync recording status on mount
    const syncStatus = async () => {
      try {
        const status = await invoke<any>("get_recording_status");
        console.log("Synced recording status:", status);
        setRecordingState({
          isRecording: status.is_recording || false,
          isPaused: status.is_paused || false,
          duration: status.duration || 0,
        });
      } catch (error) {
        console.error("Failed to sync recording status:", error);
      }
    };
    
    syncStatus();
    
    // Listen for recording events
    const unlistenPromise = listen("recording-update", (event) => {
      const data = event.payload as any;
      console.log("Recording state update:", data);
      setRecordingState({
        isRecording: data.is_recording || false,
        isPaused: data.is_paused || false,
        duration: data.duration || 0,
      });
      
      // Show notification when recording stops with file path
      if (data.output_path && !data.is_recording) {
        const path = data.output_path as string;
        alert(`Recording saved to:\n${path}`);
        console.log("Recording saved to:", path);
      }
      
      // Show notification when recording starts
      if (data.is_recording && data.duration === 0) {
        console.log("Recording started!");
      }
    }).catch((error) => {
      console.warn("Failed to listen to recording-update events:", error);
      // If events fail, periodically poll status instead
      const pollInterval = setInterval(syncStatus, 1000);
      return () => clearInterval(pollInterval);
    });

    // Listen for metrics updates
    const unlistenMetricsPromise = listen("metrics-update", (event) => {
      const data = event.payload as any;
      setMetrics({
        captureFps: data.capture_fps || 0,
        encodeFps: data.encode_fps || 0,
        droppedFrames: data.dropped_frames || 0,
        encodeLatency: data.encode_latency || 0,
        cpuUsage: data.cpu_usage || 0,
        memoryUsage: data.memory_usage || 0,
      });
    }).catch((error) => {
      console.warn("Failed to listen to metrics-update events:", error);
    });

    return () => {
      unlistenPromise.then((fn) => fn && fn()).catch(() => {});
      unlistenMetricsPromise.then((fn) => fn && fn()).catch(() => {});
    };
  }, [isTauri]);

  const handleStartRecording = async () => {
    if (!isTauri) {
      alert("This app must be run as a desktop application. Use 'npm run tauri dev' to start the desktop app.");
      return;
    }
    
    if (!selectedMonitor && !selectedWindow) {
      alert("Please select a monitor or window to record.");
      return;
    }
    
    try {
      await invoke("start_recording", {
        monitorId: selectedMonitor,
        windowId: selectedWindow,
      });
      // Optimistically update UI state even if events fail
      setRecordingState(prev => ({
        ...prev,
        isRecording: true,
        isPaused: false,
        duration: 0,
      }));
    } catch (error) {
      console.error("Failed to start recording:", error);
      const errorMsg = error instanceof Error ? error.message : String(error);
      if (errorMsg.includes("already in progress")) {
        // Try to sync status to fix stuck state
        try {
          const status = await invoke<any>("get_recording_status");
          setRecordingState({
            isRecording: status.is_recording || false,
            isPaused: status.is_paused || false,
            duration: status.duration || 0,
          });
        } catch (syncError) {
          console.error("Failed to sync status:", syncError);
        }
        alert(`Recording is already in progress. Use the Stop button to end it first.`);
      } else if (errorMsg.includes("not implemented") || errorMsg.includes("not supported")) {
        alert(`Screen capture is not yet supported on Linux. Windows support is available.`);
      } else {
        alert(`Failed to start recording: ${errorMsg}`);
      }
    }
  };

  const handleStopRecording = async () => {
    if (!isTauri) {
      alert("This app must be run as a desktop application.");
      return;
    }
    try {
      await invoke("stop_recording");
    } catch (error) {
      console.error("Failed to stop recording:", error);
      alert(`Failed to stop recording: ${error}`);
    }
  };

  const handlePauseRecording = async () => {
    if (!isTauri) {
      alert("This app must be run as a desktop application.");
      return;
    }
    try {
      await invoke("pause_recording");
    } catch (error) {
      console.error("Failed to pause recording:", error);
      alert(`Failed to pause recording: ${error}`);
    }
  };

  console.log("App.tsx: About to render JSX");
  
  return (
    <div className="app" style={{ minHeight: "100vh", backgroundColor: "#1e1e2e" }}>
      <header className="app-header">
        <h1>Screen Recorder</h1>
        <p className="subtitle">Professional Desktop Recording with Analytics</p>
        {!isTauri && (
          <div style={{ 
            background: "#ff4444", 
            color: "white", 
            padding: "10px", 
            marginTop: "10px", 
            borderRadius: "5px" 
          }}>
            ⚠️ Running in web mode. Tauri APIs are not available. 
            Run <code>npm run tauri dev</code> for the desktop app.
          </div>
        )}
      </header>

      <main className="app-main">
        <div className="left-panel">
          <DevicePicker
            selectedMonitor={selectedMonitor}
            selectedWindow={selectedWindow}
            onMonitorSelect={setSelectedMonitor}
            onWindowSelect={setSelectedWindow}
          />

          <RecordingControls
            isRecording={recordingState.isRecording}
            isPaused={recordingState.isPaused}
            duration={recordingState.duration}
            onStart={handleStartRecording}
            onStop={handleStopRecording}
            onPause={handlePauseRecording}
          />

          <MetricsPanel metrics={metrics} />
        </div>

        <div className="right-panel">
          <Timeline
            duration={recordingState.duration}
            isRecording={recordingState.isRecording}
          />
        </div>
      </main>
    </div>
  );
}

export default App;

