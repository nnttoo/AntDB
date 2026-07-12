@echo off
REM // Haryanto 12 July 2026
setlocal enabledelayedexpansion

:: Variables
set REPO=nnttoo/AntDB
set BINARY_NAME=

echo =============================================
echo       AntDB Windows Installer Script
echo =============================================

:: Detect Architecture
if "%PROCESSOR_ARCHITECTURE%"=="AMD64" (
    set BINARY_NAME=antdb-server_Win64.exe
) else if "%PROCESSOR_ARCHITEW6432%"=="AMD64" (
    set BINARY_NAME=antdb-server_Win64.exe
) else (
    set BINARY_NAME=antdb-server_Win32.exe
)

echo [*] Detected Architecture: %PROCESSOR_ARCHITECTURE%
echo [*] Target Binary: %BINARY_NAME%
echo [*] Downloading latest release from GitHub...

set DOWNLOAD_URL=https://github.com/%REPO%/releases/latest/download/%BINARY_NAME%

:: Download using PowerShell Invoke-WebRequest
powershell -Command "[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12; Invoke-WebRequest -Uri '%DOWNLOAD_URL%' -OutFile 'antdb-server.exe'"

if not exist "antdb-server.exe" (
    echo [!] Error: Download failed. Please check your internet connection.
    pause
    exit /b 1
)

echo [+] Download completed successfully.
echo =============================================

:menu
echo Choose an action for AntDB:
echo 1) Run AntDB immediately (Foreground)
echo 2) Install as a Windows Background Service (Task Scheduler)
echo 3) Exit
set /p CHOICE="Enter your choice [1-3]: "

if "%CHOICE%"=="1" goto run_foreground
if "%CHOICE%"=="2" goto install_service
if "%CHOICE%"=="3" goto exit_script
echo [!] Invalid choice, please try again.
goto menu

:run_foreground
echo [*] Starting AntDB server...
antdb-server.exe
goto exit_script

:install_service
:: Check for Administrator Rights
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo [!] Error: This action requires Administrator privileges.
    echo Please right-click this .bat file and choose 'Run as administrator'.
    echo.
    goto menu
)

set "INSTALL_DIR=C:\Program Files\AntDB"
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"

echo [*] Moving binary to %INSTALL_DIR%...
move /y "antdb-server.exe" "%INSTALL_DIR%\antdb-server.exe" >nul

echo [*] Registering background task via Task Scheduler...
schtasks /create /tn "AntDB_Server" /tr "\"%INSTALL_DIR%\antdb-server.exe\"" /sc onstart /ru "SYSTEM" /f >nul

if %errorLevel% eq 0 (
    echo [*] Starting AntDB service...
    schtasks /run /tn "AntDB_Server" >nul
    echo =============================================
    echo [+] AntDB service installed and started successfully!
    echo [*] It will run in the background automatically on system boot.
    echo [*] To stop it: schtasks /end /tn "AntDB_Server"
    echo =============================================
) else (
    echo [!] Failed to create Windows background task.
)
pause
goto exit_script

:exit_script
echo [*] Exiting installer.
endlocal