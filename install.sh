#!/bin/bash
# // Haryanto 12 July 2026

# Variables
REPO="nnttoo/AntDB"
BINARY_NAME=""
OS=$(uname -s)
ARCH=$(uname -m)

echo "============================================="
echo "       AntDB Automated Installer Script      "
echo "============================================="

# Detect Platform and Architecture
if [ "$OS" = "Linux" ]; then
    if [ "$ARCH" = "x86_64" ]; then
        BINARY_NAME="antdb-server_Linux64"
    elif [[ "$ARCH" =~ i.86 ]]; then
        BINARY_NAME="antdb-server_Linux32"
    elif [ "$ARCH" = "aarch64" ]; then
        BINARY_NAME="antdb-server_LinuxArm64"
    elif [[ "$ARCH" =~ arm.* ]]; then
        BINARY_NAME="antdb-server_LinuxArm32"
    fi
elif [ "$OS" = "Darwin" ]; then
    if [ "$ARCH" = "x86_64" ]; then
        BINARY_NAME="antdb-server_Mac64"
    elif [ "$ARCH" = "arm64" ]; then
        BINARY_NAME="antdb-server_MacArm64"
    fi
fi

# Validate if platform is supported
if [ -z "$BINARY_NAME" ]; then
    echo "Error: Unsupported platform or architecture ($OS $ARCH)"
    exit 1
fi

echo "[*] Detected Platform: $OS ($ARCH)"
echo "[*] Target Binary: $BINARY_NAME"
echo "[*] Fetching latest release from GitHub..."

# Download the specific binary asset from GitHub Releases
DOWNLOAD_URL="https://github.com/$REPO/releases/latest/download/$BINARY_NAME"
curl -L -o antdb-server "$DOWNLOAD_URL"

if [ ! -f "antdb-server" ]; then
    echo "Error: Download failed. Please check your internet connection or repository permissions."
    exit 1
fi

# Make binary executable
chmod +x antdb-server
echo "[+] Download completed and executable permissions granted."
echo "============================================="

# Interactive Menu
echo "Choose an action for AntDB:"
echo "1) Run AntDB immediately (Foreground)"
echo "2) Install as a system service (Linux Systemd)"
echo "3) Exit installer"
read -p "Enter your choice [1-3]: " CHOICE

case $CHOICE in
    1)
        echo "[*] Starting AntDB server..."
        ./antdb-server
        ;;
    2)
        if [ "$OS" != "Linux" ]; then
            echo "Error: Systemd services are only supported on Linux distributions."
            exit 1
        fi
        
        echo "[*] Installing AntDB as a systemd service..."
        INSTALL_DIR="/usr/local/bin"
        SERVICE_FILE="/etc/systemd/system/antdb.service"
        
        # Move binary to a global binary directory
        echo "[*] Moving binary to $INSTALL_DIR (requires sudo)..."
        sudo mv antdb-server "$INSTALL_DIR/antdb-server"
        
        # Create systemd service configuration file
        echo "[*] Creating systemd service file at $SERVICE_FILE..."
        sudo bash -c "cat <<EOF > $SERVICE_FILE
[Unit]
Description=AntDB In-Memory Data Store
After=network.target

[Service]
Type=simple
ExecStart=$INSTALL_DIR/antdb-server
Restart=always
RestartSec=5
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
EOF"

        # Reload, enable, and start the service
        echo "[*] Reloading systemd daemon and starting service..."
        sudo systemctl daemon-reload
        sudo systemctl enable antdb
        sudo systemctl start antdb
        
        echo "============================================="
        echo "[+] AntDB service installed and started successfully!"
        echo "[*] To check status: sudo systemctl status antdb"
        echo "[*] To view logs   : journalctl -u antdb -f"
        echo "============================================="
        ;;
    *)
        echo "[*] Installation script finished. The binary 'antdb-server' is saved in your current folder."
        exit 0
        ;;
esac