# 🔧 MCP-Forge

**A powerful CLI tool for managing Claude Desktop MCP server configurations**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)

MCP-Forge is a comprehensive command-line tool designed to streamline the management of Model Context Protocol (MCP) servers in Claude Desktop. From basic server management to advanced enterprise features like bulk operations, configuration validation, and profile management.

## ✨ Features

### 🚀 **Core Management**
- **Server Management**: Add, remove, edit, and update MCP servers
- **Template System**: Pre-built templates for popular MCP servers
- **Configuration Validation**: Deep validation with health checks
- **Interactive Setup**: Guided server configuration with variable prompts

### 🔍 **Advanced Search & Filtering**
- **Smart Filtering**: Filter servers by name, type, status, and custom criteria
- **Semantic Search**: Find servers using natural language queries
- **Ranking System**: Intelligent sorting and relevance scoring
- **Multiple Output Formats**: JSON, table, and custom formatting

### 📦 **Bulk Operations**
- **Pattern Matching**: Operate on multiple servers using glob patterns
- **Batch Processing**: Update, validate, or manage multiple servers at once
- **Progress Tracking**: Real-time progress for bulk operations
- **Dry-Run Mode**: Preview changes before applying them

### 💾 **Backup & Restore**
- **Automated Backups**: Scheduled and manual backup creation
- **Point-in-Time Recovery**: Restore configurations from any backup
- **Incremental Backups**: Efficient storage with change tracking
- **Cross-Profile Backups**: Backup and restore across different profiles

### 👤 **Profile Management**
- **Multi-Environment Support**: Separate configurations for dev, staging, prod
- **Profile Switching**: Easy switching between different environments
- **Isolated Configurations**: Keep environments completely separate
- **Profile-Specific Templates**: Custom templates per environment

### 🔧 **Enterprise Features**
- **Health Monitoring**: Comprehensive system health checks
- **Configuration Import/Export**: Migrate configurations between systems
- **Validation Rules**: Custom validation rules and requirements
- **Audit Logging**: Track all configuration changes

## 🚀 Quick Start

### Installation

#### From Source (Recommended)
```bash
git clone https://github.com/yourusername/mcp-forge.git
cd mcp-forge
cargo build --release
./target/release/mcp-forge --help
```

#### Using Cargo
```bash
cargo install mcp-forge
```

### Basic Usage

```bash
# List all configured servers
mcp-forge list

# Add a new server from template
mcp-forge add my-filesystem filesystem

# Add server with custom variables
mcp-forge add my-search brave-search --vars "api_key=your_key_here"

# Validate all configurations
mcp-forge validate-all

# Create a backup
mcp-forge backup create
```

## 📚 Command Reference

### Server Management

#### `list` - List MCP servers
```bash
# Basic listing
mcp-forge list

# Filter by name pattern
mcp-forge list --filter "filesystem*"

# Show only active servers
mcp-forge list --status active

# JSON output for scripting
mcp-forge list --json

# Advanced filtering
mcp-forge list --type filesystem --sort name --limit 10
```

#### `add` - Add new server
```bash
# Add from template
mcp-forge add server-name template-name

# Add with variables
mcp-forge add my-fs filesystem --vars "path=/home/user/docs"

# Interactive mode (prompts for variables)
mcp-forge add my-server template-name --interactive
```

#### `remove` - Remove servers
```bash
# Remove specific server
mcp-forge remove server-name

# Remove multiple servers
mcp-forge remove --pattern "test-*"

# Remove all servers (with confirmation)
mcp-forge remove --all
```

#### `edit` - Edit server configuration
```bash
# Edit server in default editor
mcp-forge edit server-name

# Edit with specific editor
EDITOR=vim mcp-forge edit server-name
```

#### `update` - Update server configuration
```bash
# Update server arguments
mcp-forge update server-name --args "new_arg=value"

# Update from template
mcp-forge update server-name --template new-template
```

### Validation & Health

#### `validate` - Validate configurations
```bash
# Validate specific server
mcp-forge validate server-name

# Deep validation with system checks
mcp-forge validate --deep --requirements

# Validate all servers
mcp-forge validate-all

# Validate with custom profile
mcp-forge --profile production validate-all
```

#### `health` - System health check
```bash
# Basic health check
mcp-forge health

# Detailed health report
mcp-forge health --detailed

# Health check for specific profile
mcp-forge --profile staging health
```

#### `doctor` - System diagnostic
```bash
# Run full diagnostic
mcp-forge doctor

# Quick diagnostic
mcp-forge doctor --quick

# Fix common issues automatically
mcp-forge doctor --fix
```

### Bulk Operations

#### `bulk` - Bulk operations
```bash
# Update multiple servers
mcp-forge bulk update --pattern "api-*" --args "timeout=30"

# Validate multiple servers
mcp-forge bulk validate --pattern "prod-*"

# Remove multiple servers
mcp-forge bulk remove --pattern "test-*" --dry-run

# Bulk operations with confirmation
mcp-forge bulk update --pattern "*" --interactive
```

### Backup & Restore

#### `backup` - Backup operations
```bash
# Create manual backup
mcp-forge backup create

# Create named backup
mcp-forge backup create --name "before-migration"

# List all backups
mcp-forge backup list

# Restore from backup
mcp-forge backup restore backup-2024-01-15.json

# Auto-cleanup old backups
mcp-forge backup cleanup --keep 10
```

### Profile Management

#### `profile` - Profile operations
```bash
# List all profiles
mcp-forge profile list

# Create new profile
mcp-forge profile create development

# Switch to profile
mcp-forge profile use development

# Copy profile
mcp-forge profile copy production staging

# Delete profile
mcp-forge profile delete old-profile
```

### Template Management

#### `template` - Template operations
```bash
# List available templates
mcp-forge template list

# Show template details
mcp-forge template show filesystem

# Validate template
mcp-forge template validate custom-template.json

# Create new template
mcp-forge template create my-template
```

### Configuration Management

#### `config` - Configuration operations
```bash
# Show current configuration
mcp-forge config show

# Show configuration file path
mcp-forge config path

# Initialize new configuration
mcp-forge config init

# Validate configuration file
mcp-forge config validate
```

### Import/Export

#### `import` - Import configurations
```bash
# Import from file
mcp-forge import --file config.json

# Import with merge strategy
mcp-forge import --file config.json --merge

# Import to specific profile
mcp-forge --profile staging import --file prod-config.json
```

#### `export` - Export configurations
```bash
# Export current configuration
mcp-forge export --output config.json

# Export specific profile
mcp-forge --profile production export --output prod-config.json

# Export with formatting
mcp-forge export --format yaml --output config.yaml
```

## 🎯 Advanced Usage

### Working with Profiles

Profiles allow you to maintain separate configurations for different environments:

```bash
# Create profiles for different environments
mcp-forge profile create development
mcp-forge profile create staging
mcp-forge profile create production

# Add servers to specific profiles
mcp-forge --profile development add dev-fs filesystem --vars "path=/tmp"
mcp-forge --profile production add prod-fs filesystem --vars "path=/data"

# Switch between profiles
mcp-forge profile use development
mcp-forge list  # Shows only development servers

# Backup specific profile
mcp-forge --profile production backup create --name "prod-backup"
```

### Bulk Operations with Patterns

Use glob patterns for powerful bulk operations:

```bash
# Update all API servers
mcp-forge bulk update --pattern "api-*" --args "timeout=60"

# Validate all production servers
mcp-forge bulk validate --pattern "prod-*" --deep

# Remove all test servers (with dry-run first)
mcp-forge bulk remove --pattern "test-*" --dry-run
mcp-forge bulk remove --pattern "test-*"  # Execute after review
```

### Advanced Filtering

Combine multiple filters for precise server selection:

```bash
# Find filesystem servers with specific status
mcp-forge list --type filesystem --status active

# Search servers by description
mcp-forge list --search "database" --sort relevance

# Complex filtering with JSON output
mcp-forge list --type api --status active --limit 5 --json
```

### Configuration Validation

Ensure your configurations are always valid:

```bash
# Deep validation with system requirements
mcp-forge validate --deep --requirements

# Validate before deployment
mcp-forge --profile production validate-all --strict

# Custom validation rules
mcp-forge validate --rules custom-rules.json
```

## 🔧 Configuration

### Configuration File Location

MCP-Forge uses the standard Claude Desktop configuration file:

- **macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
- **Windows**: `%APPDATA%\Claude\claude_desktop_config.json`
- **Linux**: `~/.config/claude/claude_desktop_config.json`

### Profile Configuration

Profiles are stored in:
- **macOS**: `~/Library/Application Support/Claude/profiles/`
- **Windows**: `%APPDATA%\Claude\profiles\`
- **Linux**: `~/.config/claude/profiles/`

### Environment Variables

```bash
# Custom configuration path
export CLAUDE_CONFIG_PATH="/path/to/custom/config.json"

# Default profile
export MCP_FORGE_PROFILE="production"

# Editor for configuration editing
export EDITOR="code"

# Backup directory
export MCP_FORGE_BACKUP_DIR="/path/to/backups"
```

## 📋 Available Templates

### Core Templates

| Template | Description | Variables |
|----------|-------------|-----------|
| `filesystem` | File system access | `path` (required) |
| `brave-search` | Brave Search API | `api_key` (required) |
| `postgres` | PostgreSQL database | `connection_string` (required) |
| `sqlite` | SQLite database | `db_path` (required) |
| `github` | GitHub API integration | `token` (optional) |

### Template Variables

Templates support five variable types:

1. **String**: Simple text values
2. **Number**: Numeric values with validation
3. **Boolean**: True/false values
4. **Array**: List of values
5. **Object**: Complex nested structures

Example template with all variable types:

```json
{
  "name": "example-server",
  "description": "Example server with all variable types",
  "config": {
    "command": "example-server",
    "args": ["--config", "{{config_path}}"]
  },
  "variables": {
    "config_path": {
      "type": "string",
      "description": "Path to configuration file",
      "required": true
    },
    "port": {
      "type": "number",
      "description": "Server port",
      "default": 8080,
      "min": 1024,
      "max": 65535
    },
    "enabled": {
      "type": "boolean",
      "description": "Enable the server",
      "default": true
    },
    "features": {
      "type": "array",
      "description": "Enabled features",
      "items": "string",
      "default": ["basic", "advanced"]
    },
    "database": {
      "type": "object",
      "description": "Database configuration",
      "properties": {
        "host": {"type": "string", "required": true},
        "port": {"type": "number", "default": 5432}
      }
    }
  }
}
```

## 🔍 Troubleshooting

### Common Issues

#### Configuration File Not Found
```bash
# Initialize configuration
mcp-forge config init

# Check configuration path
mcp-forge config path

# Verify Claude Desktop is installed
mcp-forge doctor
```

#### Server Not Starting
```bash
# Validate server configuration
mcp-forge validate server-name --deep

# Check system requirements
mcp-forge validate --requirements

# Run health check
mcp-forge health --detailed
```

#### Template Variables Not Working
```bash
# Validate template
mcp-forge template validate template-name

# Check variable syntax
mcp-forge template show template-name

# Use interactive mode for guidance
mcp-forge add server-name template-name --interactive
```

#### Profile Issues
```bash
# List available profiles
mcp-forge profile list

# Check current profile
mcp-forge profile current

# Reset to default profile
mcp-forge profile use default
```

### Debug Mode

Enable verbose output for troubleshooting:

```bash
# Verbose output
mcp-forge --verbose command

# Debug logging
RUST_LOG=debug mcp-forge command

# Trace logging (very detailed)
RUST_LOG=trace mcp-forge command
```

### Getting Help

```bash
# General help
mcp-forge --help

# Command-specific help
mcp-forge command --help

# Show version
mcp-forge --version

# Run system diagnostic
mcp-forge doctor
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/mcp-forge.git
cd mcp-forge

# Install dependencies
cargo build

# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run -- --help
```

### Template Development

See our [Template Development Guide](docs/template-development.md) for creating custom templates.

## 📄 License

This project is dual-licensed under:

- [MIT License](LICENSE-MIT)
- [Apache License 2.0](LICENSE-APACHE)

You may choose either license for your use.

## 🙏 Acknowledgments

- [Claude Desktop](https://claude.ai) for the MCP protocol
- [Anthropic](https://anthropic.com) for Claude and MCP development
- The Rust community for excellent tooling and libraries

---

**Made with ❤️ for the Claude Desktop community** 