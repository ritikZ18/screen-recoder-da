# Icon Files

## Requirements

Tauri requires **RGBA PNG** format for icon files. The icons must be:
- Valid PNG format with RGBA color space
- Correct dimensions: 32x32, 128x128, 256x256 (for @2x)

## Creating Icons

### Option 1: Using Python PIL (Recommended)

```bash
python3 create_valid_icons.py
```

This creates proper RGBA PNG files using PIL/Pillow.

### Option 2: Using ImageMagick

```bash
convert -size 32x32 xc:'rgba(70,130,180,255)' -alpha on PNG32:icons/32x32.png
convert -size 128x128 xc:'rgba(70,130,180,255)' -alpha on PNG32:icons/128x128.png
convert -size 256x256 xc:'rgba(70,130,180,255)' -alpha on PNG32:icons/128x128@2x.png
```

### Option 3: Manual Creation

Use any image editor that can export RGBA PNG files:
- GIMP
- Inkscape
- Photoshop
- Online tools (ensure RGBA output)

## Verification

Run the verification script before building:

```bash
./verify_icons.sh
```

This checks:
- All required icon files exist
- Files are valid PNG format
- Files have reasonable sizes (>100 bytes)

## Build Error: "icon is not RGBA"

If you see this error during `tauri build`:
1. Run `python3 create_valid_icons.py` to regenerate icons
2. Or use ImageMagick/PIL to create proper RGBA PNGs
3. Verify with `./verify_icons.sh`
4. Rebuild: `npm run tauri build`

