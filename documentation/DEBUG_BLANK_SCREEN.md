# Debugging Blank Screen Issue

## Symptoms
- Tauri window opens but shows blank/beige screen
- Terminal shows app compiled and started successfully
- Vite dev server is running on port 1420

## Debugging Steps

### 1. Check Browser Console in Tauri Window
The Tauri window has a developer console. To access it:
- Right-click in the window → "Inspect Element" or "Inspect"
- Or check if there's a menu option for Developer Tools
- Look for JavaScript errors in the Console tab

### 2. Verify Vite Server is Accessible
```bash
curl http://localhost:1420
```

Should return the HTML with the root div.

### 3. Check for JavaScript Errors
In the Tauri window console, look for:
- Module loading errors
- React rendering errors
- Tauri API errors
- Network errors (failed to load scripts/CSS)

### 4. Test with Simple HTML
Replace the content temporarily to verify the window loads content:

Create `test.html`:
```html
<!DOCTYPE html>
<html>
<head><title>Test</title></head>
<body style="background: red; color: white; padding: 2rem;">
  <h1>If you see this, Tauri is loading content!</h1>
</body>
</html>
```

Then test if the window can load local files.

### 5. Check Network Tab
In developer tools, check the Network tab:
- Is `main.tsx` loading?
- Are CSS files loading?
- Are there 404 errors?

### 6. Verify React is Mounting
Check the console logs we added:
- Should see "main.tsx: Starting app initialization..."
- Should see "App.tsx: Component rendering..."
- If you don't see these, React isn't mounting

### 7. Common Issues

#### Issue: CSP blocking scripts
**Fix:** The CSP is set to `null` which should allow everything, but verify in dev tools.

#### Issue: Module type not supported
**Fix:** Ensure Vite is serving modules correctly. Check `vite.config.ts`.

#### Issue: React not mounting
**Fix:** Check if there's an error in `main.tsx` or `App.tsx` that's preventing render.

#### Issue: CSS not loading
**Fix:** Check Network tab for failed CSS requests. Verify paths are correct.

#### Issue: Webview not initialized
**Fix:** Ensure Tauri webview features are enabled in `Cargo.toml`.

## Next Steps After Debugging

Once you identify the error from the console:
1. Note the exact error message
2. Check which file/line is causing the issue
3. Look for missing imports or incorrect paths
4. Verify all dependencies are installed: `npm install`

## Quick Test

Run this in browser console (when window is open):
```javascript
console.log("Window object:", window);
console.log("Root element:", document.getElementById("root"));
console.log("React loaded:", typeof React !== "undefined");
```

