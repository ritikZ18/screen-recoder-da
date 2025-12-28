import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import DevicePicker from "./components/DevicePicker";
import RecordingControls from "./components/RecordingControls";
import Timeline from "./components/Timeline";
import MetricsPanel from "./components/MetricsPanel";
import "./App.css";

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

  useEffect(() => {
    // Listen for recording events
    const unlisten = listen("recording-update", (event) => {
      const data = event.payload as any;
      setRecordingState({
        isRecording: data.is_recording || false,
        isPaused: data.is_paused || false,
        duration: data.duration || 0,
      });
    });

    // Listen for metrics updates
    const unlistenMetrics = listen("metrics-update", (event) => {
      const data = event.payload as any;
      setMetrics({
        captureFps: data.capture_fps || 0,
        encodeFps: data.encode_fps || 0,
        droppedFrames: data.dropped_frames || 0,
        encodeLatency: data.encode_latency || 0,
        cpuUsage: data.cpu_usage || 0,
        memoryUsage: data.memory_usage || 0,
      });
    });

    return () => {
      unlisten.then((fn) => fn());
      unlistenMetrics.then((fn) => fn());
    };
  }, []);

  const handleStartRecording = async () => {
    try {
      await invoke("start_recording", {
        monitorId: selectedMonitor,
        windowId: selectedWindow,
      });
    } catch (error) {
      console.error("Failed to start recording:", error);
    }
  };

  const handleStopRecording = async () => {
    try {
      await invoke("stop_recording");
    } catch (error) {
      console.error("Failed to stop recording:", error);
    }
  };

  const handlePauseRecording = async () => {
    try {
      await invoke("pause_recording");
    } catch (error) {
      console.error("Failed to pause recording:", error);
    }
  };

  return (
    <div className="app">
      <header className="app-header">
        <h1>Screen Recorder</h1>
        <p className="subtitle">Professional Desktop Recording with Analytics</p>
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

