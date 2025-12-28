# Screen Recorder - One-Command Setup Script for Windows
# This script checks dependencies, installs if needed, and starts the application

$ErrorActionPreference = "Stop"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $ScriptDir

Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "Screen Recorder - Setup and Start" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host ""

# Run dependency checker
Write-Host "Step 1: Checking dependencies..." -ForegroundColor Yellow
& "$ScriptDir\check-dependencies.ps1"

if ($LASTEXITCODE -ne 0) {
    Write-Host ""
    Write-Host "Dependency check failed. Please install missing dependencies and try again." -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "Step 2: Installing npm dependencies..." -ForegroundColor Yellow
npm install

if ($LASTEXITCODE -ne 0) {
    Write-Host "Failed to install npm dependencies." -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "Step 3: Starting development server..." -ForegroundColor Yellow
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host ""

# Start the application
npm run tauri dev

