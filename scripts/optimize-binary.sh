#!/bin/bash
set -euo pipefail

# MCP-Forge Binary Optimization Script
# This script optimizes the binary for size and performance

echo "🔧 MCP-Forge Binary Optimization"
echo "================================="

# Check if we're in the right directory
if [[ ! -f "Cargo.toml" ]]; then
    echo "❌ Error: Must be run from the project root directory"
    exit 1
fi

# Create optimized Cargo.toml profile if it doesn't exist
echo "📝 Setting up optimized build profile..."

# Check if [profile.release-optimized] exists
if ! grep -q "\[profile\.release-optimized\]" Cargo.toml; then
    cat >> Cargo.toml << 'EOF'

# Optimized release profile for distribution
[profile.release-optimized]
inherits = "release"
opt-level = "z"          # Optimize for size
lto = true               # Enable Link Time Optimization
codegen-units = 1        # Better optimization
panic = "abort"          # Smaller binary size
strip = true             # Strip symbols
EOF
    echo "✅ Added optimized release profile to Cargo.toml"
else
    echo "✅ Optimized release profile already exists"
fi

# Build with optimizations
echo "🔨 Building optimized binary..."
cargo build --profile release-optimized

# Get binary path
BINARY_PATH="target/release-optimized/mcp-forge"
if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    BINARY_PATH="${BINARY_PATH}.exe"
fi

# Check if binary exists
if [[ ! -f "$BINARY_PATH" ]]; then
    echo "❌ Error: Binary not found at $BINARY_PATH"
    exit 1
fi

# Show binary size
BINARY_SIZE=$(du -h "$BINARY_PATH" | cut -f1)
echo "📊 Optimized binary size: $BINARY_SIZE"

# Additional optimizations with UPX if available
if command -v upx &> /dev/null; then
    echo "🗜️  Compressing binary with UPX..."
    cp "$BINARY_PATH" "${BINARY_PATH}.backup"
    
    if upx --best --lzma "$BINARY_PATH" 2>/dev/null; then
        COMPRESSED_SIZE=$(du -h "$BINARY_PATH" | cut -f1)
        echo "✅ UPX compression successful"
        echo "📊 Compressed binary size: $COMPRESSED_SIZE"
        
        # Test that compressed binary still works
        echo "🧪 Testing compressed binary..."
        if "$BINARY_PATH" --version &>/dev/null; then
            echo "✅ Compressed binary works correctly"
            rm -f "${BINARY_PATH}.backup"
        else
            echo "⚠️  Compressed binary failed test, reverting..."
            mv "${BINARY_PATH}.backup" "$BINARY_PATH"
        fi
    else
        echo "⚠️  UPX compression failed, keeping uncompressed binary"
        mv "${BINARY_PATH}.backup" "$BINARY_PATH"
    fi
else
    echo "ℹ️  UPX not available, skipping compression"
    echo "   Install UPX for additional size reduction: https://upx.github.io/"
fi

# Performance test
echo "⚡ Running performance tests..."
echo "Startup time test:"
time "$BINARY_PATH" --version > /dev/null

echo "Help command test:"
time "$BINARY_PATH" --help > /dev/null

# Final binary info
echo ""
echo "🎉 Optimization complete!"
echo "📍 Optimized binary location: $BINARY_PATH"
echo "📊 Final binary size: $(du -h "$BINARY_PATH" | cut -f1)"

# Copy to standard release location for compatibility
RELEASE_PATH="target/release/mcp-forge"
if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    RELEASE_PATH="${RELEASE_PATH}.exe"
fi

cp "$BINARY_PATH" "$RELEASE_PATH"
echo "📋 Copied optimized binary to: $RELEASE_PATH"

echo ""
echo "✨ Binary optimization complete!"
echo "   The optimized binary is ready for distribution." 