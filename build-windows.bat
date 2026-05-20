@echo off
REM ─────────────────────────────────────────────────────────────────────────
REM  build-windows.bat
REM  Compiles the release binary and creates the Windows installer (.exe)
REM  Requirements:
REM    - Rust (https://rustup.rs)
REM    - Inno Setup 6 (https://jrsoftware.org/isinfo.php)
REM ─────────────────────────────────────────────────────────────────────────

echo ==========================================================
echo   IAM Business - Windows Build Script
echo ==========================================================
echo.

REM Step 1 – Compile release binary
echo [1/3] Compiling release binary...
cargo build --release
IF %ERRORLEVEL% NEQ 0 (
    echo ERROR: Cargo build failed.
    pause
    exit /b 1
)
echo       Done.
echo.

REM Step 2 – Create output directory
echo [2/3] Preparing output directory...
if not exist "dist\windows" mkdir "dist\windows"
echo       Done.
echo.

REM Step 3 – Run Inno Setup compiler
echo [3/3] Creating installer with Inno Setup...
set ISCC="C:\Program Files (x86)\Inno Setup 6\ISCC.exe"
if not exist %ISCC% (
    set ISCC="C:\Program Files\Inno Setup 6\ISCC.exe"
)
if not exist %ISCC% (
    echo ERROR: Inno Setup not found.
    echo        Download from https://jrsoftware.org/isinfo.php
    echo        Or copy the .exe from target\release\ manually.
    echo.
    echo The binary is ready at: target\release\iam-business.exe
    pause
    exit /b 0
)

%ISCC% "installer\windows\setup.iss"
IF %ERRORLEVEL% NEQ 0 (
    echo ERROR: Inno Setup failed.
    pause
    exit /b 1
)

echo.
echo ==========================================================
echo   BUILD COMPLETE
echo   Installer: dist\windows\IAMBusiness-Setup.exe
echo ==========================================================
pause
