@echo off
chcp 65001 >nul
setlocal enabledelayedexpansion

set "FB_ROOT=%~dp0"
cd /d "%FB_ROOT%"

echo ============================================
echo  FairyBench - Setup
echo ============================================
echo.

:: ---------- check prerequisites ----------
echo [1/5] Prerequisites check...
echo.

set "HAS_RUST=no"
where rustc >nul 2>&1 && set "HAS_RUST=yes"

set "HAS_CARGO=no"
where cargo >nul 2>&1 && set "HAS_CARGO=yes"

set "HAS_NODE=no"
where node >nul 2>&1 && set "HAS_NODE=yes"

set "HAS_NPM=no"
where npm >nul 2>&1 && set "HAS_NPM=yes"

if "%HAS_RUST%"=="yes" (
  for /f "tokens=2" %%v in ('rustc --version 2^^^>nul') do echo [OK] Rust %%v
) else (
  echo [--] Rust not found
)
if "%HAS_NODE%"=="yes" (
  for /f "tokens=1 delims=v" %%v in ('node --version 2^^^>nul') do echo [OK] Node %%v
) else (
  echo [--] Node not found
)

:: ---------- Rust install ----------
if "%HAS_RUST%"=="no" (
  echo.
  echo [2/5] Rust is not installed. Installing via rustup...
  echo.
  where curl >nul 2>&1
  if !errorlevel! neq 0 (
    echo [ERROR] curl not found. Please install manually from:
    echo         https://rustup.rs/
    pause
    exit /b 1
  )
  curl -# -o "%TEMP%\rustup-init.exe" "https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe"
  if !errorlevel! neq 0 (
    echo [ERROR] Failed to download rustup-init.exe
    pause
    exit /b 1
  )
  "%TEMP%\rustup-init.exe" -y --default-host x86_64-pc-windows-msvc --default-toolchain stable
  if !errorlevel! neq 0 (
    echo [ERROR] Rustup installation failed
    pause
    exit /b 1
  )
  set "PATH=%USERPROFILE%\.cargo\bin;%PATH%"
  echo [OK] Rust installed
) else (
  echo.
  echo [2/5] Rust already installed ^(skipped^)
)

:: ---------- Node.js check ----------
if "%HAS_NODE%"=="no" (
  echo.
  echo [ERROR] Node.js is not installed.
  echo         Please download and install from: https://nodejs.org/
  echo         ^(LTS version recommended^)
  pause
  exit /b 1
)

:: ---------- npm install ----------
echo.
echo [3/5] Installing Node.js dependencies...
echo.
cd /d "%FB_ROOT%"
call npm install
if %errorlevel% neq 0 (
  echo [ERROR] npm install failed
  pause
  exit /b 1
)
echo [OK] npm install completed

:: ---------- cargo build ----------
echo.
echo [4/5] Building Rust backend ^(this may take a few minutes^)...
echo.
cd /d "%FB_ROOT%"
set "PATH=%USERPROFILE%\.cargo\bin;%PATH%"
call cargo build --manifest-path src-tauri\Cargo.toml
if %errorlevel% neq 0 (
  echo [ERROR] cargo build failed
  pause
  exit /b 1
)
echo [OK] Rust build completed

:: ---------- run tests ----------
echo.
echo [5/5] Running tests...
echo.
call cargo test --manifest-path src-tauri\Cargo.toml --lib
if %errorlevel% neq 0 (
  echo [WARNING] Some tests failed
) else (
  echo [OK] All tests passed
)

:: ---------- frontend build ----------
echo.
echo [+] Building frontend...
call npm run build
if %errorlevel% neq 0 (
  echo [WARNING] Frontend build failed
) else (
  echo [OK] Frontend built successfully
)

echo.
echo ============================================
echo  Setup complete!
echo ============================================
echo.
echo How to run:
echo.
echo   Development mode ^(hot-reload^):
echo     npm run tauri dev
echo.
echo   Frontend only ^(browser preview^):
echo     npm run dev
echo.
echo   Production build:
echo     npm run tauri build
echo.
echo   Run Rust tests:
echo     cargo test --manifest-path src-tauri\Cargo.toml --lib
echo.
echo   Run frontend type-check:
echo     npm run check
echo.
pause
