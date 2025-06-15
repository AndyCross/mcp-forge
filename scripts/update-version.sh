#!/bin/bash

# Update version script for MCP-Forge
# This script updates version numbers across all packaging files

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Get the current version from Cargo.toml
VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')

if [ -z "$VERSION" ]; then
    echo -e "${RED}Error: Could not extract version from Cargo.toml${NC}"
    exit 1
fi

echo -e "${GREEN}Updating version to: ${VERSION}${NC}"

# Update Scoop package
SCOOP_FILE="packaging/scoop/mcp-forge.json"
if [ -f "$SCOOP_FILE" ]; then
    echo -e "${YELLOW}Updating Scoop package...${NC}"
    # Update version field
    sed -i.bak "s/\"version\": \"[^\"]*\"/\"version\": \"$VERSION\"/" "$SCOOP_FILE"
    # Update URL (more specific pattern for the download URL)
    sed -i.bak "s|releases/download/v[0-9]\+\.[0-9]\+\.[0-9]\+/|releases/download/v$VERSION/|g" "$SCOOP_FILE"
    rm "$SCOOP_FILE.bak"
    echo -e "${GREEN}✓ Updated $SCOOP_FILE${NC}"
else
    echo -e "${YELLOW}Warning: $SCOOP_FILE not found${NC}"
fi

# Update Homebrew formula
HOMEBREW_FILE="packaging/homebrew/mcp-forge.rb"
if [ -f "$HOMEBREW_FILE" ]; then
    echo -e "${YELLOW}Updating Homebrew formula...${NC}"
    # Update URL with version
    sed -i.bak "s/v[0-9]\+\.[0-9]\+\.[0-9]\+/v$VERSION/g" "$HOMEBREW_FILE"
    rm "$HOMEBREW_FILE.bak"
    echo -e "${GREEN}✓ Updated $HOMEBREW_FILE${NC}"
    echo -e "${YELLOW}Note: SHA256 hash will need to be updated after release${NC}"
else
    echo -e "${YELLOW}Warning: $HOMEBREW_FILE not found${NC}"
fi

# Update any other version references
echo -e "${YELLOW}Checking for other version references...${NC}"

# Check README.md for version badges or references
if grep -q "v0\.[0-9]\+\.[0-9]\+" README.md; then
    echo -e "${YELLOW}Found version references in README.md - you may want to update these manually${NC}"
fi

echo -e "${GREEN}Version update complete!${NC}"
echo -e "${YELLOW}Don't forget to:${NC}"
echo -e "  1. Update SHA256 hash in Homebrew formula after release"
echo -e "  2. Test the packaging files"
echo -e "  3. Commit the changes" 