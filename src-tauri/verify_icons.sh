#!/bin/bash
# Verify icon files are valid before build

set -e

ICON_DIR="$(dirname "$0")/icons"

# #region agent log
LOG_PATH="/home/swamizero/.cursor/debug.log"
log() {
    echo "{\"location\":\"verify_icons.sh:$1\",\"message\":\"$2\",\"data\":$3,\"timestamp\":$(date +%s%3N),\"sessionId\":\"debug-session\",\"runId\":\"icon-verify\",\"hypothesisId\":\"A\"}" >> "$LOG_PATH"
}
# #endregion

echo "🔍 Verifying icon files..."

# Check if icons directory exists
if [ ! -d "$ICON_DIR" ]; then
    # #region agent log
    log "verify_icons" "Icons directory missing" "{\"dir\":\"$ICON_DIR\"}"
    # #endregion
    echo "❌ Icons directory not found: $ICON_DIR"
    exit 1
fi

# Verify PNG files
for icon in 32x32.png 128x128.png 128x128@2x.png; do
    icon_path="$ICON_DIR/$icon"
    if [ ! -f "$icon_path" ]; then
        # #region agent log
        log "verify_icons" "Icon file missing" "{\"file\":\"$icon\"}"
        # #endregion
        echo "❌ Icon file missing: $icon_path"
        exit 1
    fi
    
    # Check if it's a valid PNG
    if ! file "$icon_path" | grep -q "PNG image"; then
        # #region agent log
        log "verify_icons" "Invalid PNG file" "{\"file\":\"$icon\",\"type\":\"$(file "$icon_path")\"}"
        # #endregion
        echo "❌ Invalid PNG file: $icon_path"
        echo "   Type: $(file "$icon_path")"
        exit 1
    fi
    
    # Check file size (should be > 100 bytes for a valid PNG)
    size=$(stat -f%z "$icon_path" 2>/dev/null || stat -c%s "$icon_path" 2>/dev/null || echo "0")
    if [ "$size" -lt 100 ]; then
        # #region agent log
        log "verify_icons" "Icon file too small" "{\"file\":\"$icon\",\"size\":$size}"
        # #endregion
        echo "⚠️  Warning: $icon_path is very small ($size bytes), may be invalid"
    fi
done

# #region agent log
log "verify_icons" "All icons verified" "{\"status\":\"success\"}"
# #endregion

echo "✅ All icon files are valid PNG images"

