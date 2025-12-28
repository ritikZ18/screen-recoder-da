import "./MetricsPanel.css";

interface Metrics {
  captureFps: number;
  encodeFps: number;
  droppedFrames: number;
  encodeLatency: number;
  cpuUsage: number;
  memoryUsage: number;
}

interface MetricsPanelProps {
  metrics: Metrics;
}

function MetricsPanel({ metrics }: MetricsPanelProps) {
  return (
    <div className="metrics-panel">
      <h2>Performance Metrics</h2>
      <div className="metrics-grid">
        <div className="metric-item">
          <div className="metric-label">Capture FPS</div>
          <div className="metric-value">{metrics.captureFps.toFixed(1)}</div>
        </div>
        <div className="metric-item">
          <div className="metric-label">Encode FPS</div>
          <div className="metric-value">{metrics.encodeFps.toFixed(1)}</div>
        </div>
        <div className="metric-item">
          <div className="metric-label">Dropped Frames</div>
          <div className="metric-value">{metrics.droppedFrames}</div>
        </div>
        <div className="metric-item">
          <div className="metric-label">Encode Latency</div>
          <div className="metric-value">{metrics.encodeLatency.toFixed(1)}ms</div>
        </div>
        <div className="metric-item">
          <div className="metric-label">CPU Usage</div>
          <div className="metric-value">{metrics.cpuUsage.toFixed(1)}%</div>
        </div>
        <div className="metric-item">
          <div className="metric-label">Memory</div>
          <div className="metric-value">
            {(metrics.memoryUsage / 1024 / 1024).toFixed(1)} MB
          </div>
        </div>
      </div>
    </div>
  );
}

export default MetricsPanel;

