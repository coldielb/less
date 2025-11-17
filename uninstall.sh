#!/bin/bash

# Code Golf Game Uninstaller

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

INSTALL_DIR="$HOME/.local/bin"
BINARY="$INSTALL_DIR/less"
DATA_DIR="$HOME/.code_golf_game"

echo ""
echo "================================================================="
echo "           CODE GOLF - Uninstaller                              "
echo "================================================================="
echo ""

# Check if binary exists
if [ ! -f "$BINARY" ]; then
    echo -e "${YELLOW}Binary not found at ${BINARY}${NC}"
    echo "It may have already been uninstalled."
else
    echo -n "Removing binary... "
    rm -f "$BINARY"
    echo -e "${GREEN}DONE${NC}"
fi

# Ask about game data
if [ -d "$DATA_DIR" ]; then
    echo ""
    echo "Game data directory found: ${CYAN}${DATA_DIR}${NC}"
    echo "This contains your solutions and personal bests."
    echo ""
    echo -n "Do you want to remove game data? (y/n): "
    read -r response
    if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
        echo -n "Removing game data... "
        rm -rf "$DATA_DIR"
        echo -e "${GREEN}DONE${NC}"
    else
        echo "Game data preserved."
    fi
fi

echo ""
echo "================================================================="
echo -e "${GREEN}Uninstallation complete!${NC}"
echo "================================================================="
echo ""
