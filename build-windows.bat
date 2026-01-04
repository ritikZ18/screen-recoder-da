@echo off
REM Windows Build Script for Screen Recorder
REM Run this in Command Prompt on Windows
REM Handles UNC paths (\\wsl$\...) by using pushd/popd

REM Handle UNC paths - pushd will map \\wsl$\... to a drive letter
set "ORIG_DIR=%CD%"
pushd "%~dp0" 2>nul || pushd "%CD%"

echo Building Screen Recorder for Windows...

REM Check Node.js
where node >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo ERROR: Node.js not found! Install from https://nodejs.org/
    pause
    exit /b 1
)
node --version

REM Check npm
where npm >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo ERROR: npm not found!
    pause
    exit /b 1
)
npm --version

REM Check Rust
where rustc >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo ERROR: Rust not found! Install from https://rustup.rs/
    pause
    exit /b 1
)
rustc --version

REM Install dependencies
echo.
echo Installing dependencies...
call npm install
if %ERRORLEVEL% NEQ 0 (
    echo ERROR: Failed to install dependencies!
    pause
    exit /b 1
)

REM Build
echo.
echo Building application...
call npm run tauri build
if %ERRORLEVEL% NEQ 0 (
    echo ERROR: Build failed!
    pause
    exit /b 1
)

REM Show output location
echo.
echo.
echo Build complete!
echo.
echo Executable location:
echo    src-tauri\target\release\screen-recorder.exe
echo.
echo TIP: Double-click screen-recorder.exe to run!
echo.

REM Restore original directory (if we used pushd)
popd 2>nul || cd /d "%ORIG_DIR%"

pause

