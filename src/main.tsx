import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { ErrorBoundary } from "./ErrorBoundary";
import "./styles.css";

console.log("main.tsx: Starting app initialization...");
console.log("main.tsx: React version:", React.version);
console.log("main.tsx: Document ready state:", document.readyState);

const rootElement = document.getElementById("root");
if (!rootElement) {
  console.error("ERROR: Root element not found! Check index.html");
  document.body.innerHTML = `
    <div style="padding: 2rem; font-family: sans-serif; background: #1e1e2e; color: #e0e0e0; min-height: 100vh;">
      <h1 style="color: #ff4444;">Error: Root element not found!</h1>
      <p>Check index.html for &lt;div id="root"&gt;&lt;/div&gt;</p>
    </div>
  `;
  throw new Error("Root element not found! Check index.html");
}

console.log("main.tsx: Root element found, rendering App...");

try {
  const root = ReactDOM.createRoot(rootElement);
  root.render(
    <React.StrictMode>
      <ErrorBoundary>
        <App />
      </ErrorBoundary>
    </React.StrictMode>,
  );
  console.log("main.tsx: App rendered successfully");
} catch (error) {
  console.error("main.tsx: Error rendering app:", error);
  // Fallback rendering
  rootElement.innerHTML = `
    <div style="padding: 2rem; font-family: sans-serif; background: #1e1e2e; color: #e0e0e0; min-height: 100vh;">
      <h1 style="color: #ff4444;">Error Loading App</h1>
      <p>Check console (F12) for details.</p>
      <pre style="background: rgba(0,0,0,0.3); padding: 1rem; border-radius: 4px; overflow: auto;">
${error instanceof Error ? error.message + "\n\n" + error.stack : String(error)}
      </pre>
      <button onclick="window.location.reload()" style="margin-top: 1rem; padding: 0.75rem 1.5rem; background: #667eea; color: white; border: none; border-radius: 8px; cursor: pointer;">
        Reload
      </button>
    </div>
  `;
}

