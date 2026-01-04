# How to Run the Desktop App

## ⚠️ CRITICAL: This is NOT a Web App!

**DO NOT** run `npm run dev` - that's just the web frontend!

## ✅ Correct Way to Run Desktop App

```bash
cd screen-recorder
npm run tauri dev
```

This will:
1. ✅ Open a **desktop window** (not browser)
2. ✅ Enable Tauri APIs
3. ✅ Make recording buttons work

## Build for Production

```bash
npm run tauri build
```

Output: `src-tauri/target/release/bundle/`

## Troubleshooting

### "invoke is undefined"
→ You're running `npm run dev` instead of `npm run tauri dev`

### "Rust not found"
→ Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### Build fails
→ Update Rust: `rustup update stable`

---

**Always use `npm run tauri dev` for the desktop app!**
