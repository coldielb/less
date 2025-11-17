#!/bin/bash

# Package the game for distribution

set -e

VERSION="1.0.0"
PACKAGE_NAME="code-golf-v${VERSION}"

# Colors
GREEN='\033[0;32m'
CYAN='\033[0;36m'
NC='\033[0m'

echo ""
echo "================================================================="
echo "           Creating Distribution Package                        "
echo "================================================================="
echo ""

# Create package directory
echo -n "Creating package directory... "
rm -rf dist
mkdir -p "dist/${PACKAGE_NAME}"
echo -e "${GREEN}DONE${NC}"

# Copy necessary files
echo -n "Copying distribution files... "
cp install.sh "dist/${PACKAGE_NAME}/"
cp uninstall.sh "dist/${PACKAGE_NAME}/"
cp README.md "dist/${PACKAGE_NAME}/"
cp QUICKSTART.md "dist/${PACKAGE_NAME}/"
cp INSTALL.md "dist/${PACKAGE_NAME}/"
cp Cargo.toml "dist/${PACKAGE_NAME}/"
cp Cargo.lock "dist/${PACKAGE_NAME}/"

# Copy source files (needed for installation)
cp -r src "dist/${PACKAGE_NAME}/"

echo -e "${GREEN}DONE${NC}"

# Create archive
echo -n "Creating tarball... "
cd dist
tar -czf "${PACKAGE_NAME}.tar.gz" "${PACKAGE_NAME}"
cd ..
echo -e "${GREEN}DONE${NC}"

# Get size
SIZE=$(du -h "dist/${PACKAGE_NAME}.tar.gz" | cut -f1)

echo ""
echo "================================================================="
echo -e "${GREEN}Package created successfully!${NC}"
echo "================================================================="
echo ""
echo "Package: ${CYAN}dist/${PACKAGE_NAME}.tar.gz${NC}"
echo "Size:    ${CYAN}${SIZE}${NC}"
echo ""
echo "Users can extract and run:"
echo "  tar -xzf ${PACKAGE_NAME}.tar.gz"
echo "  cd ${PACKAGE_NAME}"
echo "  ./install.sh"
echo ""
