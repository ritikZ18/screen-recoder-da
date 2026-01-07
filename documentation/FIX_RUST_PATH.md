# Fix Rust PATH Issue

## Problem

Your system has two Rust installations:
- **System Rust**: `/usr/bin/rustc` (version 1.75.0) - **TOO OLD**
- **rustup Rust**: `~/.cargo/bin/rustc` (version 1.92.0) - **CORRECT**

The system Rust is being used first, causing build failures.

## Solution

### Option 1: Update PATH in Current Shell

```bash
export PATH="$HOME/.cargo/bin:$PATH"
rustc --version  # Should now show 1.92.0
```

### Option 2: Add to Shell Profile (Permanent)

Add to `~/.bashrc` or `~/.zshrc`:
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Then:
```bash
source ~/.bashrc  # or ~/.zshrc
rustc --version  # Verify
```

### Option 3: Use rustup directly

```bash
~/.cargo/bin/rustc --version
~/.cargo/bin/cargo check
```

## Then Build

```bash
cd screen-recorder/src-tauri
rm -f Cargo.lock
~/.cargo/bin/cargo clean
~/.cargo/bin/cargo check
```

If successful:
```bash
cd ..
npm run tauri dev
```

## Verify

```bash
which rustc  # Should show ~/.cargo/bin/rustc
rustc --version  # Should show 1.92.0
```

---

**All code is fixed. Just need to use the correct Rust version!**

