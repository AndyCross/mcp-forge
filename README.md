# 🔧 MCP-Forge

**A powerful CLI tool for managing Claude Desktop MCP server configurations**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)

MCP-Forge is a comprehensive command-line tool designed to streamline the management of Model Context Protocol (MCP) servers in Claude Desktop. From basic server management to advanced enterprise features like bulk operations, configuration validation, and profile management.

## 🆕 Recent Updates (v0.3.1)

- **🧹 Code Quality**: Eliminated all build warnings and cleaned up 600+ lines of dead code
- **📦 Template Repository**: Moved templates to separate repository for better management
- **🔧 CI/CD Improvements**: Updated GitHub Actions for reliable releases
- **✅ Enhanced Testing**: All 25 unit tests passing with comprehensive coverage
- **🚀 Performance**: Improved code patterns and reduced binary size

## ✨ Features

### 🚀 **Core Management**
- **Server Management**: Add, remove, edit, and update MCP servers
- **Template System**: Pre-built templates from separate repository for popular MCP servers
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

#### Package Managers (Recommended)

**Homebrew (macOS/Linux)**
```bash
brew tap andycross/tap
brew install mcp-forge
```

**Scoop (Windows)**
```powershell
scoop bucket add andycross https://github.com/AndyCross/scoop-bucket
scoop install mcp-forge
```

**Cargo (All Platforms)**
```bash
cargo install --git https://github.com/AndyCross/mcp-forge
```

#### Direct Download
```bash
# Download latest release binary
curl -L https://github.com/AndyCross/mcp-forge/releases/latest/download/mcp-forge -o mcp-forge
chmod +x mcp-forge
sudo mv mcp-forge /usr/local/bin/
```

#### From Source
```bash
git clone https://github.com/AndyCross/mcp-forge.git
cd mcp-forge
cargo build --release
./target/release/mcp-forge --help
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
# List available templates (fetched from GitHub)
mcp-forge template list

# Show template details
mcp-forge template show filesystem

# Search templates by tag or description
mcp-forge template search database

# Refresh template cache from repository
mcp-forge template refresh

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

## 📋 Template System

### Template Repository Architecture

MCP-Forge uses a separate repository for templates to enable:
- **Independent Updates**: Templates can be updated without releasing new versions of MCP-Forge
- **Community Contributions**: Easy for community members to contribute new templates
- **Centralized Management**: All templates are managed in one place
- **Automatic Fetching**: Templates are automatically fetched from GitHub and cached locally

**Template Repository**: [mcp-forge-templates](https://github.com/AndyCross/mcp-forge-templates)

### How Templates Work

1. **Catalog Fetching**: MCP-Forge fetches a catalog of available templates from GitHub
2. **Local Caching**: Templates are cached locally for offline use and performance
3. **Automatic Updates**: Cache is refreshed periodically or manually with `mcp-forge template refresh`
4. **Template Installation**: Templates are applied with variable substitution to create server configurations

### Available Templates

| Template | Description | Variables | Repository |
|----------|-------------|-----------|-----------|
| `filesystem` | Local filesystem access | `readonly`, `paths` | [Official](https://github.com/AndyCross/mcp-forge-templates) |
| `brave-search` | Brave Search API integration | `api_key` (required) | [Official](https://github.com/AndyCross/mcp-forge-templates) |
| `postgres` | PostgreSQL database connection | `connection_string` (required) | [Official](https://github.com/AndyCross/mcp-forge-templates) |
| `sqlite` | SQLite database connection | `db_path` (required) | [Official](https://github.com/AndyCross/mcp-forge-templates) |
| `github` | GitHub API integration | `token` (optional) | [Official](https://github.com/AndyCross/mcp-forge-templates) |

### Template Usage Examples

```bash
# List all available templates
mcp-forge template list

# Show detailed information about a template
mcp-forge template show filesystem

# Add a server using a template
mcp-forge add my-files filesystem --vars "readonly=true,paths=['/home/user/docs']"

# Interactive template installation with prompts
mcp-forge add my-db postgres --interactive
```

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
git clone https://github.com/AndyCross/mcp-forge.git
cd mcp-forge

# Install dependencies
cargo build

# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run -- --help
```

### Template Development

#### Contributing Templates

Templates are managed in a separate repository for easier community contributions:

1. **Fork the Templates Repository**: [mcp-forge-templates](https://github.com/AndyCross/mcp-forge-templates)
2. **Add Your Template**: Create a new JSON file in the `official/` directory
3. **Update Catalog**: Add your template to `catalog.json`
4. **Submit Pull Request**: Submit a PR with your new template

#### Template Structure

Templates follow a standardized JSON format:

```json
{
  "name": "my-template",
  "version": "1.0.0",
  "description": "Description of what this template does",
  "author": "Your Name",
  "tags": ["category", "type"],
  "platforms": ["macos", "linux", "windows"],
  "variables": {
    "required_var": {
      "type": "string",
      "description": "Description of this variable",
      "required": true
    },
    "optional_var": {
      "type": "boolean", 
      "description": "Optional variable with default",
      "default": false
    }
  },
  "config": {
    "command": "your-command",
    "args": ["--arg", "{{required_var}}"],
    "env": {
      "ENV_VAR": "{{optional_var}}"
    }
  },
  "requirements": {
    "nodejs": ">=18.0.0"
  },
  "setup_instructions": "Additional setup instructions if needed"
}
```

See our [Template Development Guide](docs/template-development.md) for detailed information on creating custom templates.

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