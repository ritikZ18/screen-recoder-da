import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer } from "recharts";
import "./Timeline.css";

// Check if running in Tauri
const isTauri = typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

interface TimelineProps {
  duration: number;
  isRecording: boolean;
}

interface TimelineData {
  time: number;
  colorDominance: number;
  brightness: number;
  audioLevel: number;
  sceneChange: boolean;
}

function Timeline({ isRecording }: TimelineProps) {
  const [timelineData, setTimelineData] = useState<TimelineData[]>([]);

  useEffect(() => {
    if (!isTauri) return;
    
    // Poll timeline data while recording
    let pollInterval: NodeJS.Timeout | null = null;
    
    const pollTimelineData = async () => {
      try {
        const data = await invoke<TimelineData[]>("get_timeline_data");
        if (data.length > 0) {
          setTimelineData(data);
        }
      } catch (error) {
        console.error("Failed to poll timeline data:", error);
      }
    };
    
    if (isRecording) {
      // Poll every second while recording
      pollInterval = setInterval(pollTimelineData, 1000);
      pollTimelineData(); // Initial load
    } else {
      // Load once when recording stops
      loadTimelineData();
    }

    return () => {
      if (pollInterval) {
        clearInterval(pollInterval);
      }
    };
  }, [isRecording, isTauri]);


  const loadTimelineData = async () => {
    if (!isTauri) return;
    try {
      const data = await invoke<TimelineData[]>("get_timeline_data");
      setTimelineData(data);
    } catch (error) {
      console.error("Failed to load timeline data:", error);
    }
  };

  return (
    <div className="timeline">
      <h2>Analytics Timeline</h2>
      <div className="timeline-content">
        {timelineData.length > 0 ? (
          <ResponsiveContainer width="100%" height={400}>
            <LineChart data={timelineData}>
              <CartesianGrid strokeDasharray="3 3" stroke="rgba(255,255,255,0.1)" />
              <XAxis
                dataKey="time"
                stroke="#a0a0a0"
                label={{ value: "Time (s)", position: "insideBottom", offset: -5 }}
              />
              <YAxis stroke="#a0a0a0" />
              <Tooltip
                contentStyle={{
                  backgroundColor: "#2d2d44",
                  border: "1px solid rgba(255,255,255,0.2)",
                  borderRadius: "8px",
                }}
              />
              <Legend />
              <Line
                type="monotone"
                dataKey="brightness"
                stroke="#667eea"
                strokeWidth={2}
                name="Brightness"
                dot={false}
              />
              <Line
                type="monotone"
                dataKey="audioLevel"
                stroke="#10b981"
                strokeWidth={2}
                name="Audio Level"
                dot={false}
              />
              <Line
                type="monotone"
                dataKey="colorDominance"
                stroke="#f59e0b"
                strokeWidth={2}
                name="Color Dominance"
                dot={false}
              />
            </LineChart>
          </ResponsiveContainer>
        ) : (
          <div className="empty-timeline">
            {isRecording
              ? "Recording... Analytics will appear here"
              : "No timeline data available. Start a recording to see analytics."}
          </div>
        )}
      </div>
    </div>
  );
}

export default Timeline;

