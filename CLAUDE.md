# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

MCP-Forge is a Rust CLI tool for managing Model Context Protocol (MCP) server configurations in Claude Desktop. It provides server management, templates, validation, bulk operations, backup/restore, and profile management capabilities.

## Key Commands

### Building and Development
```bash
# Build debug version
cargo build

# Build optimized release version
cargo build --release

# Run the CLI tool in development
cargo run -- [command]

# Run with verbose/debug logging
RUST_LOG=debug cargo run -- [command]
```

### Testing
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test module
cargo test backup::tests
cargo test bulk::tests
```

### Quality Checks
```bash
# Format code
cargo fmt

# Run clippy linter
cargo clippy --all-targets --all-features -- -D warnings

# Full check (format + clippy)
make check

# Development workflow (build + test + check)
make dev
```

### Version Management
```bash
# Check version consistency across packaging files
make check-version

# Update packaging files after changing Cargo.toml version
make update-version

# Full release process
make release
```

## Architecture

### Module Structure

The codebase is organized into focused modules in `src/`:

- **main.rs**: Entry point, CLI argument parsing, command routing
- **cli.rs**: Command execution logic, orchestrates operations
- **config.rs**: Configuration file management (Claude Desktop config)
- **profiles.rs**: Profile system for multiple environments
- **templates.rs**: Template fetching from GitHub, caching, variable substitution
- **backup.rs**: Backup creation, restoration, cleanup
- **bulk.rs**: Pattern-based bulk operations on multiple servers
- **search.rs**: Server filtering, searching, ranking
- **validation.rs**: Configuration validation, health checks
- **github.rs**: GitHub API client for fetching templates
- **utils.rs**: Shared utilities, error handling, formatting

### Key Design Patterns

1. **Profile System**: Profiles are stored as separate config files in the profiles directory. The default profile is the main Claude Desktop config. Named profiles allow isolated configurations.

2. **Template Architecture**: Templates are fetched from a separate GitHub repository (mcp-forge-templates) and cached locally. Templates support variable substitution using Handlebars.

3. **Security**: Environment variables containing credentials (CLIENT_ID, CLIENT_SECRET, API_KEY, TOKEN, etc.) are automatically masked in all output to prevent leaks.

4. **Error Handling**: Uses anyhow for error propagation with context. Custom error types in utils.rs for specific error cases.

### Configuration Paths

- **macOS**: `~/Library/Application Support/Claude/`
- **Windows**: `%APPDATA%\Claude\`
- **Linux**: `~/.config/claude/`

Files:
- `claude_desktop_config.json`: Main configuration
- `profiles/`: Named profile configurations
- `.mcp-forge-cache/`: Template cache

## Testing Strategy

Tests are implemented as inline unit tests in each module using `#[cfg(test)]` blocks. Key test areas:

- Backup operations (backup.rs)
- Bulk operations and pattern matching (bulk.rs)
- Profile management (profiles.rs)
- Template parsing and validation (templates.rs)
- Configuration validation (validation.rs)

## Dependencies

Major dependencies (see Cargo.toml):
- `clap`: CLI argument parsing
- `serde`/`serde_json`: JSON serialization
- `tokio`/`reqwest`: Async HTTP for GitHub API
- `handlebars`: Template engine
- `inquire`: Interactive prompts
- `colored`: Terminal colors
- `regex`: Pattern matching for bulk operations

## Common Development Tasks

### Adding a New Command

1. Add command variant to `Commands` enum in main.rs
2. Add command handler in cli.rs `execute()` method
3. Implement logic in appropriate module or create new module
4. Add tests in the module's test section

### Modifying Templates

Templates are in the separate mcp-forge-templates repository. To test template changes locally:
1. Modify cached templates in `~/.mcp-forge-cache/`
2. Or set up local template directory for testing

### Debugging Profile Issues

Profiles can be debugged by:
- Checking profile files in the profiles directory
- Using `--profile` flag to test specific profiles
- Running with `RUST_LOG=debug` for detailed logging

## Security Considerations

- Never log or display raw environment variables
- All credential patterns are masked in output
- Template variables marked as sensitive are handled specially
- Configuration files may contain secrets - handle with care