#!/bin/sh

REPO="herring101/gather_files"
RELEASES_URL="https://api.github.com/repos/$REPO/releases"

case "$(uname -sm)" in
    "Darwin x86_64")   FILE="gather-macos-amd64"      ;;
    "Darwin arm64")    FILE="gather-macos-arm64"      ;;
    "Linux x86_64")    FILE="gather-linux-musl-amd64" ;;
    "Linux aarch64")   FILE="gather-linux-arm64"      ;;
    *) echo "Unsupported platform: $(uname -sm)"; exit 1 ;;
esac

echo "Detected binary: $FILE"

TAG=$(curl -fsSL $RELEASES_URL | grep -o '"tag_name": "[^"]*' | head -n 1 | cut -d'"' -f4)
DOWNLOAD_URL="https://github.com/$REPO/releases/download/$TAG/$FILE"
INSTALL_DIR="$HOME/.local/bin"

mkdir -p "$INSTALL_DIR"
echo "Downloading gather..."
curl -fsSL "$DOWNLOAD_URL" -o "$INSTALL_DIR/gather" && chmod +x "$INSTALL_DIR/gather"

# PATH 追加（なければ）
case ":$PATH:" in
    *":$INSTALL_DIR:"*) ;;
    *) echo "export PATH=\"$PATH:$INSTALL_DIR\"" >> "$HOME/.profile";;
esac

echo "Installation completed!  Run 'gather' from any terminal."
