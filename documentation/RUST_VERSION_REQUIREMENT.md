# Rust Version Requirement

## Issue

If you encounter an error like:
```
feature `edition2024` is required
The package requires the Cargo feature called `edition2024`, but that feature is not stabilized in this version of Cargo (1.75.0).
```

This indicates your Rust toolchain needs to be updated.

## Solution

### Update Rust Toolchain

**Linux/macOS:**
```bash
# If rustup is installed
rustup update stable

# If rustup is not installed, install it first:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
rustup update stable
```

**Windows:**
```powershell
# Update Rust
rustup update stable

# Or reinstall if needed
Invoke-WebRequest https://win.rustup.rs/x86_64 -OutFile rustup-init.exe
.\rustup-init.exe
```

### Verify Installation

```bash
rustc --version  # Should be 1.70.0 or later
cargo --version
```

### Minimum Requirements

- **Rust**: 1.70.0 or later (recommended: latest stable)
- **Cargo**: Comes with Rust installation

### Alternative: Clean Cargo Cache

If updating Rust doesn't work, try cleaning the Cargo cache:

```bash
# Remove corrupted registry cache
rm -rf ~/.cargo/registry/src/index.crates.io-*

# Update dependencies
cd src-tauri
cargo update
cargo check
```

---

**Note**: The screen-recorder project requires Rust 1.70+ for Tauri 2.0 compatibility.

