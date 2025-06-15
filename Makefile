# MCP-Forge Makefile
# Common development tasks

.PHONY: help build test clean release update-version check-version

# Default target
help:
	@echo "MCP-Forge Development Commands"
	@echo "=============================="
	@echo ""
	@echo "  build           Build the project"
	@echo "  test            Run all tests"
	@echo "  clean           Clean build artifacts"
	@echo "  check           Run clippy and formatting checks"
	@echo "  update-version  Update version in all packaging files"
	@echo "  check-version   Check version consistency across files"
	@echo "  release         Build release version and update packaging"
	@echo ""

# Build the project
build:
	cargo build

# Build release version
build-release:
	cargo build --release

# Run tests
test:
	cargo test

# Run clippy and formatting checks
check:
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings

# Clean build artifacts
clean:
	cargo clean

# Update version in packaging files
update-version:
	@echo "Updating version in packaging files..."
	@./scripts/update-version.sh

# Check version consistency
check-version:
	@echo "Checking version consistency..."
	@VERSION=$$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/'); \
	echo "Cargo.toml version: $$VERSION"; \
	SCOOP_VERSION=$$(grep '"version":' packaging/scoop/mcp-forge.json | sed 's/.*"version": "\([^"]*\)".*/\1/'); \
	echo "Scoop version: $$SCOOP_VERSION"; \
	if [ "$$VERSION" != "$$SCOOP_VERSION" ]; then \
		echo "❌ Version mismatch detected!"; \
		echo "Run 'make update-version' to fix"; \
		exit 1; \
	else \
		echo "✅ All versions are consistent"; \
	fi

# Full release process
release: check test build-release update-version
	@echo "✅ Release build complete!"
	@echo "Next steps:"
	@echo "  1. Commit the version updates"
	@echo "  2. Create and push a git tag"
	@echo "  3. Create a GitHub release"

# Development workflow
dev: build test check
	@echo "✅ Development checks passed!" 