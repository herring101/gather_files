#!/bin/sh

REPO="herring101/gather_files"
RELEASES_URL="https://api.github.com/repos/$REPO/releases"

case "$(uname -sm)" in
    "Darwin x86_64")
        PLATFORM="macos"
        ARCH="amd64"
        FILE="gather_files-macos-amd64"
        ;;
    "Darwin arm64")
        PLATFORM="macos"
        ARCH="arm64"
        FILE="gather_files-macos-arm64"
        ;;
    "Linux x86_64")
        PLATFORM="linux"
        ARCH="amd64"
        FILE="gather_files-linux-musl-amd64"
        ;;
    "Linux aarch64")
        PLATFORM="linux"
        ARCH="arm64"
        FILE="gather_files-linux-arm64"
        ;;
    *)
        echo "Unsupported platform: $(uname -sm)"
        exit 1
        ;;
esac

echo "Detected platform: $PLATFORM-$ARCH"

if ! TAG=$(curl -fsSL $RELEASES_URL | grep -o '"tag_name": "[^"]*' | head -n 1 | cut -d'"' -f4); then
    echo "Failed to fetch latest release information"
    exit 1
fi

DOWNLOAD_URL="https://github.com/$REPO/releases/download/$TAG/$FILE"
INSTALL_DIR="$HOME/.local/bin"

mkdir -p "$INSTALL_DIR"

echo "Downloading gather_files..."
if curl -fsSL "$DOWNLOAD_URL" -o "$INSTALL_DIR/gather_files"; then
    chmod +x "$INSTALL_DIR/gather_files"
    
    if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
        SHELL_CONFIG=""
        if [ -f "$HOME/.zshrc" ]; then
            SHELL_CONFIG="$HOME/.zshrc"
        elif [ -f "$HOME/.bashrc" ]; then
            SHELL_CONFIG="$HOME/.bashrc"
        fi
        
        if [ -n "$SHELL_CONFIG" ]; then
            echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$SHELL_CONFIG"
            echo "Added $INSTALL_DIR to PATH in $SHELL_CONFIG"
            echo "Please restart your terminal or run: source $SHELL_CONFIG"
        else
            echo "Please add $INSTALL_DIR to your PATH manually"
        fi
    fi
    
    echo "Installation completed!"
    echo "You can now run 'gather_files' from your terminal."
else
    echo "Download failed"
    exit 1
fi
