import "./RecordingControls.css";

interface RecordingControlsProps {
  isRecording: boolean;
  isPaused: boolean;
  duration: number;
  onStart: () => void;
  onStop: () => void;
  onPause: () => void;
}

function RecordingControls({
  isRecording,
  isPaused,
  duration,
  onStart,
  onStop,
  onPause,
}: RecordingControlsProps) {
  const formatTime = (seconds: number): string => {
    const hrs = Math.floor(seconds / 3600);
    const mins = Math.floor((seconds % 3600) / 60);
    const secs = Math.floor(seconds % 60);
    return `${hrs.toString().padStart(2, "0")}:${mins
      .toString()
      .padStart(2, "0")}:${secs.toString().padStart(2, "0")}`;
  };

  return (
    <div className="recording-controls">
      <div className="timer">
        <div className="timer-display">{formatTime(duration)}</div>
        {isRecording && (
          <div className={`recording-indicator ${isPaused ? "paused" : ""}`}>
            {isPaused ? "⏸ PAUSED" : "● REC"}
          </div>
        )}
      </div>

      <div className="controls">
        {!isRecording ? (
          <button className="start-btn" onClick={onStart}>
            Start Recording
          </button>
        ) : (
          <>
            <button className="pause-btn" onClick={onPause}>
              {isPaused ? "Resume" : "Pause"}
            </button>
            <button className="stop-btn" onClick={onStop}>
              Stop
            </button>
          </>
        )}
      </div>
    </div>
  );
}

export default RecordingControls;

