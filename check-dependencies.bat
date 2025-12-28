@echo off
REM Screen Recorder - Dependency Checker and Installer for Windows (Batch)
REM This script checks and installs all required dependencies for Windows systems

echo ========================================
echo Screen Recorder - Dependency Checker
echo ========================================
echo.

REM Check if PowerShell is available
where powershell >nul 2>&1
if %ERRORLEVEL% EQU 0 (
    echo Running PowerShell script...
    powershell -ExecutionPolicy Bypass -File "%~dp0check-dependencies.ps1"
    exit /b %ERRORLEVEL%
) else (
    echo PowerShell is required to run the dependency checker.
    echo Please install PowerShell or run check-dependencies.ps1 manually.
    exit /b 1
)

