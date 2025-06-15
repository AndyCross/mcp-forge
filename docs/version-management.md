# Version Management

This document explains how version management works in MCP-Forge and how to maintain consistency across all packaging files.

## Overview

MCP-Forge uses semantic versioning and maintains version numbers in multiple places:
- `Cargo.toml` - The source of truth for the version
- `packaging/scoop/mcp-forge.json` - Scoop package manifest
- `packaging/homebrew/mcp-forge.rb` - Homebrew formula

## Automated Version Management

### Local Development

Use the provided tools to keep versions in sync:

```bash
# Check if all versions are consistent
make check-version

# Update all packaging files to match Cargo.toml
make update-version

# Full release workflow (build, test, update versions)
make release
```

### Manual Script

You can also run the version update script directly:

```bash
./scripts/update-version.sh
```

This script:
1. Extracts the version from `Cargo.toml`
2. Updates the Scoop manifest version and download URLs
3. Updates the Homebrew formula archive URL
4. Reports what was changed

### GitHub Actions (Automatic)

When you create a GitHub release, the `update-packaging.yml` workflow automatically:
1. Extracts the version from the release tag
2. Updates all packaging files
3. Creates a pull request with the changes

## Release Process

### 1. Update Version in Cargo.toml

```toml
[package]
version = "0.3.2"  # Update this
```

### 2. Update Packaging Files

```bash
make update-version
```

### 3. Verify Consistency

```bash
make check-version
```

### 4. Commit and Tag

```bash
git add -A
git commit -m "chore: bump version to 0.3.2"
git tag v0.3.2
git push origin master --tags
```

### 5. Create GitHub Release

Create a release on GitHub, which will:
- Trigger the CI/CD pipeline to build binaries
- Automatically create a PR to update packaging files (if needed)

## Manual Updates

If you need to update packaging files manually:

### Scoop Manifest

Edit `packaging/scoop/mcp-forge.json`:
- Update `version` field
- Update download URL in `architecture.64bit.url`
- Update hash after release (see below)

### Homebrew Formula

Edit `packaging/homebrew/mcp-forge.rb`:
- Update archive URL
- Update SHA256 hash after release (see below)

## Updating Hashes

After creating a GitHub release with binaries:

### Scoop Hash

```bash
# Download the Windows binary and get its hash
curl -L https://github.com/AndyCross/mcp-forge/releases/download/v0.3.1/mcp-forge-windows-x86_64.zip -o temp.zip
sha256sum temp.zip
# Update the hash in packaging/scoop/mcp-forge.json
```

### Homebrew SHA256

```bash
# Download the source archive and get its hash
curl -L https://github.com/AndyCross/mcp-forge/archive/v0.3.1.tar.gz -o temp.tar.gz
sha256sum temp.tar.gz
# Update the sha256 in packaging/homebrew/mcp-forge.rb
```

## Troubleshooting

### Version Mismatch

If `make check-version` fails:
1. Run `make update-version` to sync all files
2. Verify the changes look correct
3. Commit the updates

### Script Permissions

If the update script fails:
```bash
chmod +x scripts/update-version.sh
```

### GitHub Actions Not Running

Ensure the workflow file has the correct permissions and the release was created properly (not just a tag).

## Best Practices

1. **Always update Cargo.toml first** - It's the source of truth
2. **Use the automated tools** - Don't update packaging files manually unless necessary
3. **Check consistency** - Run `make check-version` before releases
4. **Update hashes after release** - SHA256 hashes can only be updated after binaries are built
5. **Test packaging** - Verify that package managers can install the new version

## Files Involved

- `Cargo.toml` - Source of truth for version
- `scripts/update-version.sh` - Version update script
- `Makefile` - Development commands
- `.github/workflows/update-packaging.yml` - Automated updates
- `packaging/scoop/mcp-forge.json` - Scoop manifest
- `packaging/homebrew/mcp-forge.rb` - Homebrew formula 