# Troubleshooting Blank Screen Issue

## Current Status
- ✅ Rust backend compiles and starts
- ✅ Vite process is running
- ⚠️ Port 1420 may not be accessible
- ⚠️ Window opens but shows blank screen

## Debugging Steps

### 1. Check if Vite is Actually Serving

Open a browser and go to: `http://localhost:1420`

If you see the React app in the browser but not in the Tauri window, it's a Tauri webview issue.
If you see nothing, Vite isn't serving correctly.

### 2. Check Browser Console in Tauri Window

**Right-click in the Tauri window → "Inspect" or press F12**

Look for:
- Console logs starting with "main.tsx:"
- JavaScript errors
- Network errors
- Failed module loads

### 3. Check Tauri Window is Loading URL

In the console, run:
```javascript
console.log("Current URL:", window.location.href);
console.log("Tauri available:", typeof window !== "undefined" && "__TAURI_INTERNALS__" in window);
```

### 4. Verify Files Are Loading

In DevTools Network tab, check:
- Is `main.tsx` loading? (should show 200 status)
- Are CSS files loading?
- Are there any 404 errors?

### 5. Common Issues

#### Issue: Vite not binding to localhost
**Fix:** Updated `vite.config.ts` to explicitly bind to `127.0.0.1`

#### Issue: Window loads but React doesn't render
**Fix:** Check console for React errors. ErrorBoundary should catch them.

#### Issue: libEGL warnings
**Note:** These are harmless on WSL/Linux - just graphics driver warnings.

#### Issue: CSP blocking scripts
**Fix:** CSP is set to `null` which should allow everything. If still blocking, check DevTools Console.

### 6. Quick Test

Replace `index.html` temporarily with:
```html
<!DOCTYPE html>
<html>
<body style="background: red; padding: 2rem; color: white;">
  <h1>If you see this, Tauri is loading HTML!</h1>
  <script>
    console.log("Script executed!");
    document.body.innerHTML += "<p>Script worked!</p>";
  </script>
</body>
</html>
```

If you see red background with text, HTML/JS loading works - issue is with React.
If still blank, Tauri webview isn't loading content at all.

## Next Steps

1. **Restart the dev server** with the updated Vite config
2. **Check browser at http://localhost:1420** - does it show the app?
3. **Check Tauri window console** - what errors/logs do you see?
4. **Share the console output** so we can identify the exact issue

