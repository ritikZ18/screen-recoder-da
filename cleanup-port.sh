#!/bin/bash
# Cleanup script to free port 1420 and remove stale processes/locks

set -e

echo "🧹 Cleaning up port 1420 and stale processes..."

cd "$(dirname "$0")"

# Kill any processes using port 1420
if command -v lsof >/dev/null 2>&1; then
    PIDS=$(lsof -ti :1420 2>/dev/null || true)
    if [ -n "$PIDS" ]; then
        echo "Killing processes on port 1420: $PIDS"
        kill -9 $PIDS 2>/dev/null || true
        sleep 1
    fi
fi

# Kill any stale vite/node processes in this directory
pkill -f "vite.*1420" 2>/dev/null || true
pkill -f "node.*vite" 2>/dev/null || true
pkill -f "npm.*dev" 2>/dev/null || true

# Clean Vite cache
rm -rf node_modules/.vite 2>/dev/null || true
rm -rf dist 2>/dev/null || true

# Clean Rust build artifacts (optional, can be slow)
# rm -rf src-tauri/target/debug/incremental 2>/dev/null || true

# Verify port is free
if command -v node >/dev/null 2>&1; then
    node check-port.js 1420 || echo "⚠️  Port may still be in use"
fi

echo "✅ Cleanup complete! Port 1420 should now be available."
echo ""
echo "You can now run: npm run tauri dev"

