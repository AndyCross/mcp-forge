# MCP-Forge Distribution Guide

This guide covers how to distribute MCP-Forge across different package managers.

## 📦 Package Managers

### 1. Cargo (crates.io) - ✅ Ready

**Status**: Ready to publish  
**Command**: `cargo install mcp-forge`

#### To Publish:
```bash
# Login to crates.io (one-time setup)
cargo login

# Publish the package
cargo publish
```

#### Verification:
```bash
cargo install mcp-forge
mcp-forge --version
```

### 2. Homebrew - 🔄 Setup Required

**Status**: Formula ready, tap needed  
**Command**: `brew install andycross/tap/mcp-forge`

#### Setup Steps:
1. **Create Homebrew Tap Repository**:
   ```bash
   # Create new repo: homebrew-tap
   gh repo create homebrew-tap --public --description "Homebrew tap for AndyCross tools"
   ```

2. **Add Formula**:
   ```bash
   # Clone the tap repo
   git clone https://github.com/AndyCross/homebrew-tap.git
   cd homebrew-tap
   
   # Copy our formula
   cp ../mcp-forge/packaging/homebrew/mcp-forge.rb Formula/
   
   # Update SHA256 hash
   # Get the hash from the GitHub release
   curl -sL https://github.com/AndyCross/mcp-forge/archive/v0.3.0.tar.gz | shasum -a 256
   
   # Edit Formula/mcp-forge.rb and replace REPLACE_WITH_ACTUAL_SHA256
   
   # Commit and push
   git add Formula/mcp-forge.rb
   git commit -m "Add mcp-forge formula"
   git push
   ```

3. **Test Installation**:
   ```bash
   brew tap andycross/tap
   brew install mcp-forge
   ```

### 3. Scoop (Windows) - 🔄 Setup Required

**Status**: Manifest ready, bucket needed  
**Command**: `scoop install mcp-forge`

#### Setup Steps:
1. **Create Scoop Bucket Repository**:
   ```bash
   # Create new repo: scoop-bucket
   gh repo create scoop-bucket --public --description "Scoop bucket for AndyCross tools"
   ```

2. **Add Manifest**:
   ```bash
   # Clone the bucket repo
   git clone https://github.com/AndyCross/scoop-bucket.git
   cd scoop-bucket
   
   # Copy our manifest
   cp ../mcp-forge/packaging/scoop/mcp-forge.json bucket/
   
   # Update hash
   # Get the hash from the Windows release binary
   
   # Commit and push
   git add bucket/mcp-forge.json
   git commit -m "Add mcp-forge manifest"
   git push
   ```

3. **Test Installation**:
   ```bash
   scoop bucket add andycross https://github.com/AndyCross/scoop-bucket
   scoop install mcp-forge
   ```

### 4. GitHub Releases - ✅ Active

**Status**: Active with automated CI/CD  
**Download**: Direct binary downloads from releases

#### Current Setup:
- Automated multi-platform builds
- Release assets for Windows, macOS, Linux
- Security auditing and performance benchmarks

## 🚀 Automated Release Process

Our GitHub Actions CI/CD pipeline automatically:

1. **On Tag Push** (`v*`):
   - Builds for all platforms
   - Runs security audits
   - Creates optimized binaries
   - Generates release assets
   - Updates package manager configurations

2. **On Pull Request**:
   - Runs full test suite
   - Performance benchmarks
   - Security scanning
   - Multi-platform compatibility checks

## 📋 Release Checklist

### For New Releases:

1. **Update Version**:
   ```bash
   # Use our automated script
   ./scripts/release.sh
   ```

2. **Verify CI/CD**:
   - Check GitHub Actions pass
   - Verify all platform builds
   - Test release binaries

3. **Update Package Managers**:
   - **Cargo**: Automatic via `cargo publish`
   - **Homebrew**: Update SHA256 in formula
   - **Scoop**: Update hash in manifest

4. **Community Announcement**:
   - Update README with new version
   - Post to relevant communities
   - Update documentation

## 🔧 Maintenance

### Homebrew Formula Updates:
```bash
# Get new release SHA256
curl -sL https://github.com/AndyCross/mcp-forge/archive/v{VERSION}.tar.gz | shasum -a 256

# Update Formula/mcp-forge.rb with new version and hash
# Commit and push to homebrew-tap repo
```

### Scoop Manifest Updates:
```bash
# Get new release hash
# Update bucket/mcp-forge.json with new version and hash
# Commit and push to scoop-bucket repo
```

## 📊 Distribution Analytics

Track adoption across platforms:
- **Cargo**: crates.io download stats
- **Homebrew**: Analytics via tap repository
- **Scoop**: Download metrics
- **GitHub**: Release download counts

## 🎯 Next Steps

1. **Immediate**: Set up Homebrew tap and Scoop bucket
2. **Short-term**: Publish to crates.io
3. **Medium-term**: Submit to official package repositories
4. **Long-term**: Consider additional package managers (apt, yum, etc.) 