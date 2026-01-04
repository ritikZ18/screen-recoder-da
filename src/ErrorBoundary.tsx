import React from "react";

interface ErrorBoundaryState {
  hasError: boolean;
  error: Error | null;
}

export class ErrorBoundary extends React.Component<
  { children: React.ReactNode },
  ErrorBoundaryState
> {
  constructor(props: { children: React.ReactNode }) {
    super(props);
    this.state = { hasError: false, error: null };
  }

  static getDerivedStateFromError(error: Error): ErrorBoundaryState {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: React.ErrorInfo) {
    console.error("ErrorBoundary caught an error:", error, errorInfo);
  }

  render() {
    if (this.state.hasError) {
      return (
        <div
          style={{
            padding: "2rem",
            minHeight: "100vh",
            backgroundColor: "#1e1e2e",
            color: "#e0e0e0",
            fontFamily: "system-ui, sans-serif",
          }}
        >
          <h1 style={{ color: "#ff4444", marginBottom: "1rem" }}>
            ⚠️ Error Loading Application
          </h1>
          <div
            style={{
              background: "rgba(255, 68, 68, 0.2)",
              padding: "1rem",
              borderRadius: "8px",
              marginBottom: "1rem",
            }}
          >
            <p style={{ marginBottom: "0.5rem" }}>
              <strong>Error:</strong> {this.state.error?.message || "Unknown error"}
            </p>
            {this.state.error?.stack && (
              <pre
                style={{
                  background: "rgba(0, 0, 0, 0.3)",
                  padding: "1rem",
                  borderRadius: "4px",
                  overflow: "auto",
                  fontSize: "0.875rem",
                }}
              >
                {this.state.error.stack}
              </pre>
            )}
          </div>
          <button
            onClick={() => window.location.reload()}
            style={{
              padding: "0.75rem 1.5rem",
              background: "#667eea",
              color: "white",
              border: "none",
              borderRadius: "8px",
              cursor: "pointer",
              fontSize: "1rem",
            }}
          >
            Reload Application
          </button>
        </div>
      );
    }

    return this.props.children;
  }
}

