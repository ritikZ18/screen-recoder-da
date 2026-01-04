#!/usr/bin/env bash
# Wrapper script to run Tauri dev with software rendering (for WSL compatibility)
# Usage: ./run-tauri-dev.sh

set -e

export LIBGL_ALWAYS_SOFTWARE=1
export GALLIUM_DRIVER=llvmpipe

echo "🔧 Using software rendering (WSL compatibility mode)"
echo ""

cd "$(dirname "$0")"
npm run tauri dev

