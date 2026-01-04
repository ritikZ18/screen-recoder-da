#!/bin/bash

# Screen Recorder - One-Command Setup Script
# This script checks dependencies, installs if needed, and starts the application

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "=========================================="
echo "Screen Recorder - Setup and Start"
echo "=========================================="
echo ""

# Run dependency checker
echo "Step 1: Checking dependencies..."
bash "$SCRIPT_DIR/check-dependencies.sh"

# Check if dependencies check passed
if [ $? -ne 0 ]; then
    echo ""
    echo "Dependency check failed. Please install missing dependencies and try again."
    exit 1
fi

echo ""
echo "Step 2: Installing npm dependencies..."
npm install

echo ""
echo "Step 3: Starting development server..."
echo "=========================================="
echo ""

# Start the application
npm run tauri dev

