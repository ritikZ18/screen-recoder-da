# Windows Build Script for Screen Recorder
# Run this in PowerShell on Windows (not WSL)

Write-Host "Building Screen Recorder for Windows..." -ForegroundColor Cyan

# Check Node.js
if (-not (Get-Command node -ErrorAction SilentlyContinue)) {
    Write-Host "ERROR: Node.js not found! Install from https://nodejs.org/" -ForegroundColor Red
    exit 1
}
Write-Host "OK: Node.js: $(node --version)" -ForegroundColor Green

# Check npm
if (-not (Get-Command npm -ErrorAction SilentlyContinue)) {
    Write-Host "ERROR: npm not found!" -ForegroundColor Red
    exit 1
}
Write-Host "OK: npm: $(npm --version)" -ForegroundColor Green

# Check Rust
if (-not (Get-Command rustc -ErrorAction SilentlyContinue)) {
    Write-Host "ERROR: Rust not found! Install from https://rustup.rs/" -ForegroundColor Red
    Write-Host "   Or run: winget install Rustlang.Rustup" -ForegroundColor Yellow
    exit 1
}
Write-Host "OK: Rust: $(rustc --version)" -ForegroundColor Green

# Check Tauri CLI
if (-not (Get-Command tauri -ErrorAction SilentlyContinue)) {
    Write-Host "WARNING: Tauri CLI not found globally, installing..." -ForegroundColor Yellow
    npm install -g @tauri-apps/cli
}

# Install dependencies
Write-Host "`nInstalling dependencies..." -ForegroundColor Cyan
npm install
if ($LASTEXITCODE -ne 0) {
    Write-Host "ERROR: Failed to install dependencies!" -ForegroundColor Red
    exit 1
}

# Build
Write-Host "`nBuilding application..." -ForegroundColor Cyan
npm run tauri build
if ($LASTEXITCODE -ne 0) {
    Write-Host "ERROR: Build failed!" -ForegroundColor Red
    exit 1
}

# Show output location
Write-Host "`nBuild complete!" -ForegroundColor Green
Write-Host "`nExecutable location:" -ForegroundColor Cyan
Write-Host "   src-tauri\target\release\screen-recorder.exe" -ForegroundColor White

Write-Host "`nInstaller location:" -ForegroundColor Cyan
$msiPath = "src-tauri\target\release\bundle\msi\Screen Recorder_1.0.0_x64_en-US.msi"
if (Test-Path $msiPath) {
    Write-Host "   $msiPath" -ForegroundColor White
    Write-Host "`nTIP: Double-click the .msi file to install, or run screen-recorder.exe directly!" -ForegroundColor Yellow
} else {
    Write-Host "   (MSI installer not found, but .exe should be available)" -ForegroundColor Yellow
}

