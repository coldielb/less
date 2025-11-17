#!/bin/bash

# Code Golf Game Installer
# Installs the game binary to ~/.local/bin

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Spinner function
spinner() {
    local pid=$1
    local delay=0.1
    local spinstr='|/-\'
    while [ "$(ps a | awk '{print $1}' | grep $pid)" ]; do
        local temp=${spinstr#?}
        printf " [%c]  " "$spinstr"
        local spinstr=$temp${spinstr%"$temp"}
        sleep $delay
        printf "\b\b\b\b\b\b"
    done
    printf "    \b\b\b\b"
}

# Progress bar
progress_bar() {
    local duration=$1
    local width=50
    local progress=0

    while [ $progress -le 100 ]; do
        local filled=$((progress * width / 100))
        local empty=$((width - filled))

        printf "\r["
        printf "%${filled}s" | tr ' ' '='
        printf "%${empty}s" | tr ' ' ' '
        printf "] %3d%%" $progress

        progress=$((progress + 2))
        sleep $(echo "$duration / 50" | bc -l)
    done
    echo ""
}

echo ""
echo "================================================================="
echo "           CODE GOLF - Functional Language Edition              "
echo "================================================================="
echo ""

# Check for Rust
echo -n "Checking for Rust installation... "
if ! command -v rustc &> /dev/null; then
    echo -e "${RED}NOT FOUND${NC}"
    echo ""
    echo -e "${YELLOW}Rust is required to build this project.${NC}"
    echo "Would you like to install Rust now? (y/n)"
    read -r response
    if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
        echo ""
        echo "Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
        echo -e "${GREEN}Rust installed successfully!${NC}"
    else
        echo -e "${RED}Installation cancelled.${NC}"
        exit 1
    fi
else
    echo -e "${GREEN}FOUND${NC}"
fi

# Check Rust version
RUST_VERSION=$(rustc --version | awk '{print $2}')
echo "Using Rust version: ${CYAN}${RUST_VERSION}${NC}"
echo ""

# Create install directory if it doesn't exist
INSTALL_DIR="$HOME/.local/bin"
echo "Installation directory: ${CYAN}${INSTALL_DIR}${NC}"

if [ ! -d "$INSTALL_DIR" ]; then
    echo -n "Creating installation directory... "
    mkdir -p "$INSTALL_DIR"
    echo -e "${GREEN}DONE${NC}"
fi

# Check if directory is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo ""
    echo -e "${YELLOW}WARNING: ${INSTALL_DIR} is not in your PATH${NC}"
    echo "Add this line to your ~/.bashrc, ~/.zshrc, or ~/.profile:"
    echo ""
    echo -e "    ${CYAN}export PATH=\"\$HOME/.local/bin:\$PATH\"${NC}"
    echo ""
    echo "Then restart your terminal or run: source ~/.bashrc (or ~/.zshrc)"
    echo ""
fi

# Build the project
echo "Building project in release mode..."
echo -e "${BLUE}This may take a few minutes...${NC}"
echo ""

# Start build
cargo build --release > /tmp/build.log 2>&1 &
BUILD_PID=$!

# Show spinner while building
spinner $BUILD_PID

wait $BUILD_PID
BUILD_RESULT=$?

if [ $BUILD_RESULT -ne 0 ]; then
    echo -e "${RED}Build failed!${NC}"
    echo "Check /tmp/build.log for details"
    tail -20 /tmp/build.log
    exit 1
fi

echo -e "${GREEN}Build completed successfully!${NC}"
echo ""

# Get binary size
BINARY_SIZE=$(du -h target/release/less | cut -f1)
echo "Binary size: ${CYAN}${BINARY_SIZE}${NC}"
echo ""

# Copy binary
echo -n "Installing binary to ${INSTALL_DIR}... "
cp target/release/less "$INSTALL_DIR/less"
chmod +x "$INSTALL_DIR/less"
echo -e "${GREEN}DONE${NC}"
echo ""

# Verify installation
if [ -x "$INSTALL_DIR/less" ]; then
    echo "================================================================="
    echo -e "${GREEN}Installation successful!${NC}"
    echo "================================================================="
    echo ""
    echo "You can now run the game by typing:"
    echo ""
    echo -e "    ${CYAN}less${NC}"
    echo ""
    echo "Game features:"
    echo "  - 25 coding challenges (5 tutorials + 20 advanced)"
    echo "  - Custom functional programming language"
    echo "  - Interactive REPL for experimentation"
    echo "  - Built-in language reference (press H in menu)"
    echo "  - Personal best tracking with SQLite"
    echo ""
    echo "Controls:"
    echo "  - Main Menu: Up/Down or j/k to navigate, Enter to select"
    echo "  - Editor: Ctrl+R to run tests, Esc to go back"
    echo "  - REPL: Type expressions and press Enter"
    echo ""
    echo "Happy coding!"
    echo ""
else
    echo -e "${RED}Installation failed!${NC}"
    exit 1
fi
