#!/bin/bash
set -euo pipefail

# MCP-Forge Release Script
# Automates the release process including version bumping, tagging, and publishing

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

echo "🚀 MCP-Forge Release Script"
echo "==========================="

# Check if we're in a clean git state
if [[ -n "$(git status --porcelain)" ]]; then
    echo "❌ Error: Working directory is not clean. Please commit or stash changes."
    git status --short
    exit 1
fi

# Check if we're on main/master branch
CURRENT_BRANCH=$(git branch --show-current)
if [[ "$CURRENT_BRANCH" != "main" && "$CURRENT_BRANCH" != "master" ]]; then
    echo "❌ Error: Must be on main or master branch for release. Currently on: $CURRENT_BRANCH"
    exit 1
fi

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
echo "📋 Current version: $CURRENT_VERSION"

# Ask for new version
echo ""
echo "Version bump options:"
echo "1. Patch (bug fixes): $CURRENT_VERSION -> $(echo $CURRENT_VERSION | awk -F. '{$3++; print $1"."$2"."$3}')"
echo "2. Minor (new features): $CURRENT_VERSION -> $(echo $CURRENT_VERSION | awk -F. '{$2++; $3=0; print $1"."$2"."$3}')"
echo "3. Major (breaking changes): $CURRENT_VERSION -> $(echo $CURRENT_VERSION | awk -F. '{$1++; $2=0; $3=0; print $1"."$2"."$3}')"
echo "4. Custom version"

read -p "Select version bump (1-4): " VERSION_CHOICE

case $VERSION_CHOICE in
    1)
        NEW_VERSION=$(echo $CURRENT_VERSION | awk -F. '{$3++; print $1"."$2"."$3}')
        ;;
    2)
        NEW_VERSION=$(echo $CURRENT_VERSION | awk -F. '{$2++; $3=0; print $1"."$2"."$3}')
        ;;
    3)
        NEW_VERSION=$(echo $CURRENT_VERSION | awk -F. '{$1++; $2=0; $3=0; print $1"."$2"."$3}')
        ;;
    4)
        read -p "Enter custom version: " NEW_VERSION
        ;;
    *)
        echo "❌ Invalid choice"
        exit 1
        ;;
esac

echo "📝 New version will be: $NEW_VERSION"
read -p "Continue with release? (y/N): " CONFIRM

if [[ "$CONFIRM" != "y" && "$CONFIRM" != "Y" ]]; then
    echo "❌ Release cancelled"
    exit 1
fi

# Update version in Cargo.toml
echo "📝 Updating version in Cargo.toml..."
sed -i.bak "s/^version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" Cargo.toml
rm Cargo.toml.bak

# Update Cargo.lock
echo "📝 Updating Cargo.lock..."
cargo check --quiet

# Update CHANGELOG.md
echo "📝 Updating CHANGELOG.md..."
TODAY=$(date +%Y-%m-%d)

# Create new changelog entry
TEMP_CHANGELOG=$(mktemp)
cat > "$TEMP_CHANGELOG" << EOF
# Changelog

All notable changes to MCP-Forge will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [$NEW_VERSION] - $TODAY

### Added
- Release $NEW_VERSION

### Changed
- Version bump to $NEW_VERSION

### Fixed
- Various bug fixes and improvements

EOF

# Append existing changelog (skip the header)
tail -n +8 CHANGELOG.md >> "$TEMP_CHANGELOG"
mv "$TEMP_CHANGELOG" CHANGELOG.md

echo "✅ Updated CHANGELOG.md (please edit to add specific changes)"

# Run tests
echo "🧪 Running tests..."
cargo test --quiet

# Build optimized binary
echo "🔨 Building optimized release binary..."
if [[ -f "scripts/optimize-binary.sh" ]]; then
    ./scripts/optimize-binary.sh
else
    cargo build --release
fi

# Commit changes
echo "📝 Committing release changes..."
git add Cargo.toml Cargo.lock CHANGELOG.md
git commit -m "chore: release v$NEW_VERSION"

# Create and push tag
echo "🏷️  Creating release tag..."
git tag -a "v$NEW_VERSION" -m "Release v$NEW_VERSION"

echo "📤 Pushing changes and tag..."
git push origin "$CURRENT_BRANCH"
git push origin "v$NEW_VERSION"

# Create GitHub release (if gh CLI is available)
if command -v gh &> /dev/null; then
    echo "🎉 Creating GitHub release..."
    
    # Extract changelog for this version
    RELEASE_NOTES=$(awk "/## \[$NEW_VERSION\]/,/## \[/{if(/## \[/ && !/## \[$NEW_VERSION\]/) exit; if(!/## \[$NEW_VERSION\]/) print}" CHANGELOG.md)
    
    gh release create "v$NEW_VERSION" \
        --title "Release v$NEW_VERSION" \
        --notes "$RELEASE_NOTES" \
        --draft
    
    echo "✅ GitHub release created as draft"
    echo "   Edit the release notes and publish when ready"
else
    echo "ℹ️  GitHub CLI not available, skipping GitHub release creation"
    echo "   Create release manually at: https://github.com/AndyCross/mcp-forge/releases/new"
fi

# Final instructions
echo ""
echo "🎉 Release v$NEW_VERSION completed successfully!"
echo ""
echo "Next steps:"
echo "1. Edit CHANGELOG.md to add specific changes for this release"
echo "2. Review and publish the GitHub release"
echo "3. Update package manager configurations with new version"
echo "4. Announce the release to the community"
echo ""
echo "Release artifacts:"
echo "- Git tag: v$NEW_VERSION"
echo "- Binary: target/release/mcp-forge"
echo "- GitHub release: https://github.com/AndyCross/mcp-forge/releases/tag/v$NEW_VERSION" 