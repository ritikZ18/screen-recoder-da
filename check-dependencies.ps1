# Screen Recorder - Dependency Checker and Installer for Windows
# This script checks and installs all required dependencies for Windows systems

$ErrorActionPreference = "Stop"

# Colors for output
function Write-ColorOutput($ForegroundColor) {
    $fc = $host.UI.RawUI.ForegroundColor
    $host.UI.RawUI.ForegroundColor = $ForegroundColor
    if ($args) {
        Write-Output $args
    }
    $host.UI.RawUI.ForegroundColor = $fc
}

function Write-Success { Write-ColorOutput Green "✓ $args" }
function Write-Error { Write-ColorOutput Red "✗ $args" }
function Write-Warning { Write-ColorOutput Yellow "⚠ $args" }
function Write-Info { Write-ColorOutput Cyan "$args" }

# Check if command exists
function Test-Command {
    param([string]$Command)
    $null -ne (Get-Command $Command -ErrorAction SilentlyContinue)
}

# Check Rust installation
function Test-Rust {
    if (Test-Command "rustc") {
        $version = (rustc --version).Split(' ')[1]
        Write-Success "Rust installed: $version"
        return $true
    } else {
        Write-Error "Rust not installed"
        return $false
    }
}

# Install Rust
function Install-Rust {
    Write-Info "Installing Rust..."
    try {
        Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile "$env:TEMP\rustup-init.exe"
        Start-Process -FilePath "$env:TEMP\rustup-init.exe" -ArgumentList "-y" -Wait -NoNewWindow
        $env:Path = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "User")
        Write-Success "Rust installed successfully"
        Write-Warning "Please restart your terminal or run: refreshenv"
    } catch {
        Write-Error "Failed to install Rust: $_"
        throw
    }
}

# Check Node.js installation
function Test-NodeJS {
    if (Test-Command "node") {
        $version = (node --version).Substring(1).Split('.')[0]
        if ([int]$version -ge 18) {
            Write-Success "Node.js installed: $(node --version)"
            return $true
        } else {
            Write-Error "Node.js version too old: $(node --version) (requires 18+)"
            return $false
        }
    } else {
        Write-Error "Node.js not installed"
        return $false
    }
}

# Install Node.js
function Install-NodeJS {
    Write-Info "Installing Node.js 18..."
    try {
        # Download Node.js installer
        $nodeUrl = "https://nodejs.org/dist/v18.20.4/node-v18.20.4-x64.msi"
        $nodeInstaller = "$env:TEMP\nodejs-installer.msi"
        Invoke-WebRequest -Uri $nodeUrl -OutFile $nodeInstaller
        
        # Install silently
        Start-Process -FilePath "msiexec.exe" -ArgumentList "/i `"$nodeInstaller`" /quiet /norestart" -Wait -NoNewWindow
        
        # Refresh PATH
        $env:Path = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "User")
        
        Write-Success "Node.js installed successfully"
        Write-Warning "Please restart your terminal or run: refreshenv"
    } catch {
        Write-Error "Failed to install Node.js: $_"
        Write-Info "Please download and install Node.js manually from: https://nodejs.org/"
        throw
    }
}

# Check Visual Studio Build Tools
function Test-VSBuildTools {
    $vsPath = "${env:ProgramFiles}\Microsoft Visual Studio"
    $vsPathx86 = "${env:ProgramFiles(x86)}\Microsoft Visual Studio"
    
    if ((Test-Path $vsPath) -or (Test-Path $vsPathx86)) {
        Write-Success "Visual Studio found"
        return $true
    } else {
        # Check for Build Tools specifically
        $buildToolsPath = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\2019\BuildTools"
        if (Test-Path $buildToolsPath) {
            Write-Success "Visual Studio Build Tools found"
            return $true
        } else {
            Write-Error "Visual Studio Build Tools not found"
            return $false
        }
    }
}

# Install Visual Studio Build Tools
function Install-VSBuildTools {
    Write-Info "Installing Visual Studio Build Tools..."
    Write-Warning "This will download and install Visual Studio Build Tools."
    Write-Warning "The installer will open - please select 'Desktop development with C++' workload"
    
    try {
        $vsInstallerUrl = "https://aka.ms/vs/17/release/vs_buildtools.exe"
        $vsInstaller = "$env:TEMP\vs_buildtools.exe"
        
        Invoke-WebRequest -Uri $vsInstallerUrl -OutFile $vsInstaller
        
        Write-Info "Starting Visual Studio Build Tools installer..."
        Write-Info "Please select 'Desktop development with C++' workload in the installer"
        
        Start-Process -FilePath $vsInstaller -ArgumentList "--quiet", "--wait", "--add", "Microsoft.VisualStudio.Workload.VCTools", "--includeRecommended" -Wait
        
        Write-Success "Visual Studio Build Tools installation started"
        Write-Warning "This may take a while. Please wait for the installation to complete."
    } catch {
        Write-Error "Failed to download Visual Studio Build Tools installer"
        Write-Info "Please download and install manually from: https://visualstudio.microsoft.com/downloads/"
        Write-Info "Select 'Build Tools for Visual Studio' and install 'Desktop development with C++' workload"
    }
}

# Check Chocolatey (optional, for easier package management)
function Test-Chocolatey {
    if (Test-Command "choco") {
        Write-Success "Chocolatey found (optional)"
        return $true
    } else {
        return $false
    }
}

# Main function
function Main {
    Write-Info "========================================"
    Write-Info "Screen Recorder - Dependency Checker"
    Write-Info "========================================"
    Write-Output ""
    
    $needsInstall = $false
    
    # Check Rust
    if (-not (Test-Rust)) {
        $needsInstall = $true
    }
    
    # Check Node.js
    if (-not (Test-NodeJS)) {
        $needsInstall = $true
    }
    
    # Check Visual Studio Build Tools
    if (-not (Test-VSBuildTools)) {
        $needsInstall = $true
    }
    
    Write-Output ""
    
    if ($needsInstall) {
        Write-Warning "Some dependencies are missing."
        $response = Read-Host "Do you want to install them now? (y/n)"
        
        if ($response -eq 'y' -or $response -eq 'Y') {
            Write-Output ""
            
            # Install Rust
            if (-not (Test-Rust)) {
                Install-Rust
            }
            
            # Install Node.js
            if (-not (Test-NodeJS)) {
                Install-NodeJS
            }
            
            # Install Visual Studio Build Tools
            if (-not (Test-VSBuildTools)) {
                Install-VSBuildTools
            }
            
            Write-Output ""
            Write-Success "========================================"
            Write-Success "Installation process completed!"
            Write-Success "========================================"
            Write-Warning "Please restart your terminal/PowerShell for changes to take effect."
        } else {
            Write-Info "Installation cancelled."
            exit 1
        }
    } else {
        Write-Success "========================================"
        Write-Success "All dependencies are installed!"
        Write-Success "========================================"
    }
    
    Write-Output ""
    Write-Info "You can now run: npm run tauri dev"
}

# Run main function
Main

