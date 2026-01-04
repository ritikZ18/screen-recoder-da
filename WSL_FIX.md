# WSL Graphics Rendering Fix

## Problem
On WSL, you may see libEGL warnings like:
```
libEGL warning: MESA-LOADER: failed to open vgem: ...
```

This is a WSLg GPU/EGL path issue. The app may still work, but these warnings are annoying.

## Solution
Force software rendering using `LIBGL_ALWAYS_SOFTWARE=1`.

## Implementation

### ✅ Already Fixed in package.json

The fix is now automatic! Just run:

```bash
npm run tauri:dev
```

The environment variable is set automatically in the npm script.

### Alternative: Use the wrapper script

You can also use the helper script:

```bash
./run-tauri-dev.sh
```

### Manual override (if needed)

If you want to set it manually:

```bash
LIBGL_ALWAYS_SOFTWARE=1 GALLIUM_DRIVER=llvmpipe npm run tauri dev
```

## What Changed

Updated `package.json` scripts:
- `tauri` - Now includes software rendering flags
- `tauri:dev` - Automatically uses software rendering
- `tauri:build` - Build also uses software rendering

## Performance Note

Software rendering may be slightly slower than hardware acceleration, but it's more compatible and eliminates the warnings. This is perfectly fine for development and most use cases.

## For Production Builds

The built app will also use software rendering. If you want to disable it later (once GPU path is fixed), remove the environment variables from the build script.

## Remove Warnings Permanently (Optional)

If you want this for all apps (not just this project), add to your `~/.bashrc`:

```bash
echo 'export LIBGL_ALWAYS_SOFTWARE=1' >> ~/.bashrc
echo 'export GALLIUM_DRIVER=llvmpipe' >> ~/.bashrc
source ~/.bashrc
```

But the package.json solution is better for project portability.

