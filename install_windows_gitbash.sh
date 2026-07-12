#!/bin/bash
# // Haryanto 12 July 2026

REPO="nnttoo/AntDB"
BINARY_NAME=""
ARCH=$(uname -m)

echo "============================================="
echo "   AntDB Windows Installer (via Git Bash)    "
echo "============================================="

# Detect Architecture
if [ "$ARCH" = "x86_64" ]; then
    BINARY_NAME="antdb-server_Win64.exe"
else
    BINARY_NAME="antdb-server_Win32.exe"
fi

echo "[*] Detected Architecture: $ARCH"
echo "[*] Target Binary: $BINARY_NAME"
echo "[*] Downloading latest release from GitHub..."

# Download using native curl inside Git Bash
DOWNLOAD_URL="https://github.com/$REPO/releases/latest/download/$BINARY_NAME"
curl -L -o antdb-server.exe "$DOWNLOAD_URL"

if [ ! -f "antdb-server.exe" ]; then
    echo "[!] Error: Download failed. Please check your internet connection."
    exit 1
fi

echo "[+] Download completed successfully."
echo "============================================="

echo "Choose an action for AntDB:"
echo "1) Run AntDB immediately (Foreground)"
echo "2) Install as a Windows Background Service (Task Scheduler)"
echo "3) Exit"
read -p "Enter your choice [1-3]: " CHOICE

case $CHOICE in
    1)
        echo "[*] Starting AntDB server..."
        ./antdb-server.exe
        ;;
    2)
        # Check for Administrator Rights in Git Bash
        if ! net session > /dev/null 2>&1; then
            echo "[!] Error: This action requires Administrator privileges."
            echo "Please close Git Bash, right-click 'Git Bash' icon, and choose 'Run as administrator'."
            exit 1
        fi

        INSTALL_DIR="/c/Program Files/AntDB"
        mkdir -p "$INSTALL_DIR"

        echo "[*] Moving binary to $INSTALL_DIR..."
        mv -f antdb-server.exe "$INSTALL_DIR/antdb-server.exe"

        # Convert POSIX path (/c/...) to Windows path (C:\...) for schtasks
        WIN_EXEC_PATH=$(cygpath -w "$INSTALL_DIR/antdb-server.exe")

        echo "[*] Registering background task via Task Scheduler..."
        schtasks.exe /create /tn "AntDB_Server" /tr "\"$WIN_EXEC_PATH\"" /sc onstart /ru "SYSTEM" /f

        if [ $? -eq 0 ]; then
            echo "[*] Starting AntDB service..."
            schtasks.exe /run /tn "AntDB_Server"
            echo "============================================="
            echo "[+] AntDB service installed and started successfully!"
            echo "[*] It will run in the background automatically on system boot."
            echo "[*] To stop it: schtasks.exe /end /tn AntDB_Server"
            echo "============================================="
        else
            echo "[!] Failed to create Windows background task."
        fi
        ;;
    *)
        echo "[*] Exiting installer."
        exit 0
        ;;
esac