@echo off
REM Screen Recorder - One-Command Setup Script for Windows
REM This script checks dependencies, installs if needed, and starts the application

cd /d "%~dp0"

echo ==========================================
echo Screen Recorder - Setup and Start
echo ==========================================
echo.

REM Check if PowerShell is available
where powershell >nul 2>&1
if %ERRORLEVEL% EQU 0 (
    echo Running PowerShell setup script...
    powershell -ExecutionPolicy Bypass -File "%~dp0setup.ps1"
    exit /b %ERRORLEVEL%
) else (
    echo PowerShell is required to run the setup script.
    echo Please install PowerShell or run setup.ps1 manually.
    exit /b 1
)

