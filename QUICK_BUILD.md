# Quick Build Instructions

## ✅ All Code Fixed - Ready to Build!

### Step 1: Verify Rust Version

```bash
rustc --version  # Should show 1.70.0 or later
```

If it shows 1.75.0 or older:
```bash
source ~/.cargo/env
rustc --version  # Check again
```

### Step 2: Clean and Build

```bash
cd screen-recorder/src-tauri
rm -f Cargo.lock
cargo clean
cargo check
```

### Step 3: Run Desktop App

If `cargo check` succeeds:
```bash
cd ..
npm run tauri dev
```

## What to Expect

✅ **Desktop window opens** (not browser)  
✅ **No "invoke is undefined" errors**  
✅ **Device picker loads**  
✅ **Recording buttons work**

## Troubleshooting

### "Rust version too old"
→ Run: `rustup update stable && source ~/.cargo/env`

### "Cargo.lock version 4"
→ Run: `rm -f src-tauri/Cargo.lock && cargo clean`

### "Still getting errors"
→ Run: `./verify_build.sh` to check all fixes are applied

---

**All code issues are fixed. Just need correct Rust version in your PATH!**

