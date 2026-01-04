#!/bin/bash
# Verification script to check all fixes are applied

set -e

echo "🔍 Verifying all fixes..."

cd "$(dirname "$0")/src-tauri"

# Check Rust version
RUST_VERSION=$(rustc --version 2>/dev/null | cut -d' ' -f2 || echo "not installed")
echo "📦 Rust version: $RUST_VERSION"

if [[ "$RUST_VERSION" < "1.70" ]]; then
    echo "❌ Rust version too old! Need 1.70.0+"
    echo "   Run: rustup update stable"
    exit 1
fi

# Check Windows module is conditional
if grep -q "^pub mod windows;" src/capture/mod.rs && ! grep -B1 "^pub mod windows;" src/capture/mod.rs | grep -q "#\[cfg(windows)\]"; then
    echo "❌ Windows module not conditional!"
    exit 1
fi
if ! grep -q "#\[cfg(windows)\]" src/capture/mod.rs; then
    echo "❌ Windows module missing cfg attribute!"
    exit 1
fi
echo "✅ Windows module is conditional"

# Check OpenTelemetry has metrics feature
if ! grep -q 'opentelemetry.*features.*metrics' Cargo.toml; then
    echo "❌ OpenTelemetry missing metrics feature!"
    exit 1
fi
echo "✅ OpenTelemetry has metrics feature"

# Check icons exist
if [ ! -f "icons/32x32.png" ] || [ ! -f "icons/128x128.png" ]; then
    echo "❌ Icon files missing!"
    exit 1
fi
echo "✅ Icon files exist"

# Check Capture enum exists
if ! grep -q "pub enum Capture" src/capture/mod.rs; then
    echo "❌ Capture enum not found!"
    exit 1
fi
echo "✅ Capture enum exists"

# Try to compile
echo "🔨 Attempting compilation..."
if cargo check 2>&1 | grep -q "error\[E"; then
    echo "❌ Compilation errors found!"
    cargo check 2>&1 | grep "error\[E" | head -5
    exit 1
fi

echo "✅ All checks passed! Build should succeed."
echo ""
echo "Next step: npm run tauri dev"

