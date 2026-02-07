#!/bin/bash
# Extract just the minimal-browser binary to a standalone location
# Usage: ./copy-binary.sh [destination]

BINARY="./target/release/minimal-browser"
DEST="${1:-.}"

if [ ! -f "$BINARY" ]; then
    echo "Error: Binary not found at $BINARY"
    exit 1
fi

SIZE=$(ls -lh "$BINARY" | awk '{print $5}')
echo "Copying minimal-browser ($SIZE) to $DEST/"

cp "$BINARY" "$DEST/"
chmod +x "$DEST/minimal-browser"

echo "✓ Binary copied! You can now run: $DEST/minimal-browser"
echo ""
echo "Note: The standalone binary works independently."
echo "      You can delete the 'target/' folder after copying if you wish."
