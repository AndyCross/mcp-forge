# MCP-Forge

A uvx-compatible CLI tool for managing Claude Desktop MCP server configurations with zero operational costs.

## Overview

MCP-Forge simplifies the management of Model Context Protocol (MCP) servers for Claude Desktop by providing:

- **Template-based configuration** - Easy setup using pre-built templates
- **Zero-cost operation** - Uses only GitHub as a storage backend, no paid services required
- **Cross-platform support** - Works on Windows, macOS, and Linux
- **Offline operation** - Cached templates work without internet connectivity
- **Safe modifications** - Automatic backups before any configuration changes

## Features

### ✅ Sprint 1 - Foundation (Available Now)
- ✅ List, add, remove, and edit MCP servers
- ✅ Cross-platform Claude Desktop config file detection
- ✅ Automatic backup creation before modifications
- ✅ JSON output support for scripting
- ✅ Basic template support (filesystem, brave-search)
- ✅ Interactive CLI prompts for easy configuration

### 🚧 Sprint 2 - Template System (Coming Soon)
- 🚧 Full GitHub-based template repository
- 🚧 Template variable substitution with Handlebars
- 🚧 Local template caching for offline use
- 🚧 Template validation and creation tools

### 📋 Sprint 3 - Advanced Features (Planned)
- 📋 Enhanced filtering and search capabilities
- 📋 Configuration validation and linting
- 📋 Bulk operations and batch management
- 📋 Dry-run mode for previewing changes

### 🚀 Sprint 4 - Distribution (Planned)
- 🚀 Pre-built binaries for all platforms
- 🚀 Package manager distribution (Homebrew, Scoop)
- 🚀 Auto-update functionality
- 🚀 Comprehensive documentation site

## Installation

### Using UVX (Recommended)
```bash
# Install from source (current)
git clone https://github.com/mcp-forge/mcp-forge.git
cd mcp-forge
cargo build --release
cp target/release/mcp-forge ~/.local/bin/

# Future: Direct installation from registry
# uvx install mcp-forge
```

### Using Cargo
```bash
# Future: Install from crates.io
# cargo install mcp-forge
```

### Direct Download
```bash
# Future: Download pre-built binaries
# curl -L https://github.com/mcp-forge/mcp-forge/releases/latest/download/mcp-forge-$(uname -s)-$(uname -m) -o mcp-forge
# chmod +x mcp-forge
```

## Quick Start

### List Current Servers
```bash
# Show all configured MCP servers
mcp-forge list

# Filter by name or command
mcp-forge list --filter filesystem

# Output as JSON for scripting
mcp-forge list --json
```

### Add New Servers
```bash
# Add filesystem server (interactive)
mcp-forge add my-files filesystem

# Add with predefined variables
mcp-forge add docs filesystem --vars "paths=~/Documents,~/Projects"

# Add Brave Search server
mcp-forge add web-search brave-search --vars "api_key=your_api_key_here"
```

### Manage Existing Servers
```bash
# Edit server configuration
mcp-forge edit my-files

# Update server arguments
mcp-forge update my-files --args "~/Documents ~/Downloads"

# Remove a server
mcp-forge remove my-files

# Remove all servers (with confirmation)
mcp-forge remove --all
```

### Configuration Management
```bash
# Show current configuration
mcp-forge config show

# Validate configuration
mcp-forge config validate

# Create backup
mcp-forge config backup

# Show config file location
mcp-forge config path
```

### Template Operations
```bash
# List available templates
mcp-forge template list

# Show template details
mcp-forge template show filesystem
```

## Configuration File Locations

MCP-Forge automatically detects the Claude Desktop configuration file location:

- **macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
- **Windows**: `%APPDATA%\Claude\claude_desktop_config.json`
- **Linux**: `~/.config/claude/claude_desktop_config.json`

## Built-in Templates (Sprint 1)

### Filesystem Server
Provides Claude with access to local directories.

```bash
mcp-forge add files filesystem --vars "paths=~/Desktop,~/Downloads"
```

**Variables:**
- `paths` - Comma-separated list of directories to grant access to

### Brave Search Server
Enables Claude to search the web using Brave Search API.

```bash
mcp-forge add search brave-search --vars "api_key=YOUR_API_KEY"
```

**Variables:**  
- `api_key` - Your Brave Search API key (required)

## Safety Features

- **Automatic Backups**: Every modification creates a timestamped backup
- **Validation**: Configuration files are validated before saving
- **Confirmation Prompts**: Destructive operations require confirmation
- **Atomic Operations**: Changes are applied atomically to prevent corruption

## Architecture & Design

MCP-Forge follows a zero-cost operation model:

- **No Database**: All data stored in GitHub repositories
- **No APIs**: No paid external services or authentication required
- **No Hosting**: Runs entirely locally as a static binary
- **GitHub Only**: Uses GitHub as the universal storage backend

## Development

### Building from Source
```bash
git clone https://github.com/mcp-forge/mcp-forge.git
cd mcp-forge
cargo build --release
```

### Running Tests
```bash
cargo test
```

### Development Dependencies
- Rust 1.70+ (2021 edition)
- Git for repository operations

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Sprint Roadmap
We're following a structured 5-week sprint plan:

1. **Sprint 1**: Foundation & Core Operations (✅ Complete)
2. **Sprint 2**: Template System with GitHub Integration
3. **Sprint 3**: Advanced Features & Polish
4. **Sprint 4**: Distribution & Community Features
5. **Sprint 5**: Documentation & Launch

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Support

- **Issues**: [GitHub Issues](https://github.com/mcp-forge/mcp-forge/issues)
- **Discussions**: [GitHub Discussions](https://github.com/mcp-forge/mcp-forge/discussions)
- **Documentation**: [Project Wiki](https://github.com/mcp-forge/mcp-forge/wiki)

---

**Status**: Sprint 1 Complete ✅ | Next: Sprint 2 Template System 🚧 

# MCP-Forge Templates

This repository contains the official template collection for [MCP-Forge](https://github.com/mcp-forge/mcp-forge), a CLI tool for managing Claude Desktop MCP server configurations.

## Repository Structure

```
├── catalog.json                   # Master template index
├── templates/
│   ├── official/                  # Official templates by Anthropic
│   │   ├── filesystem.json
│   │   ├── brave-search.json
│   │   ├── sqlite.json
│   │   ├── postgres.json
│   │   └── github.json
│   └── community/                 # Community-contributed templates
├── schemas/
│   └── template-schema.json       # JSON schema for template validation
└── examples/                      # Example templates and configurations
```

## Available Templates

### 🏛️ Official Templates

- **filesystem** - Access local filesystem from Claude
- **brave-search** - Search the web using Brave Search API
- **sqlite** - Query SQLite databases from Claude
- **postgres** - Query PostgreSQL databases from Claude
- **github** - Interact with GitHub repositories and issues

### 👥 Community Templates

Community templates will be added as contributions are made. See [Contributing](#contributing) below.

## Template Format

Templates use a standardized JSON format with variable substitution powered by Handlebars. Here's the structure:

```json
{
  "name": "template-name",
  "version": "1.0.0",
  "description": "Brief description of what this template does",
  "author": "Author Name",
  "tags": ["category", "type"],
  "platforms": ["windows", "macos", "linux"],
  "variables": {
    "variable_name": {
      "type": "string|boolean|number|array|select",
      "description": "Description of this variable",
      "default": "optional_default_value",
      "required": true,
      "options": ["for", "select", "type"] // only for select type
    }
  },
  "config": {
    "command": "command_to_run",
    "args": ["arg1", "{{variable_name}}"],
    "env": {
      "ENV_VAR": "{{variable_value}}"
    }
  },
  "requirements": {
    "nodejs": ">=18.0.0"
  },
  "setup_instructions": "Instructions for setting up this template"
}
```

### Variable Types

- **string** - Text input
- **boolean** - True/false choice
- **number** - Numeric input
- **array** - List of values (comma-separated in CLI)
- **select** - Choose from predefined options

### Built-in Variables

Templates can use these built-in variables:

- `{{os}}` - Operating system (windows, macos, linux)
- `{{arch}}` - Architecture (x64, arm64)
- `{{home_dir}}` - User's home directory
- `{{config_dir}}` - Claude config directory

## Using Templates

### With MCP-Forge

```bash
# List available templates
mcp-forge template list

# Show template details
mcp-forge template show filesystem

# Add server using template (interactive)
mcp-forge add my-filesystem filesystem

# Add server using template (non-interactive)
mcp-forge add my-filesystem filesystem --vars paths=/home/user/docs,readonly=false
```

### Manual Installation

You can also download and use templates manually:

```bash
# Download template
curl -o filesystem.json https://raw.githubusercontent.com/mcp-forge/templates/main/templates/official/filesystem.json

# Validate template
mcp-forge template validate filesystem.json
```

## Contributing

We welcome community contributions! Here's how to contribute:

### 1. Fork and Clone

```bash
git clone https://github.com/your-username/templates.git
cd templates
```

### 2. Create a New Template

```bash
# Create template file
mkdir -p templates/community
cat > templates/community/my-template.json << 'EOF'
{
  "name": "my-template",
  "version": "1.0.0",
  "description": "Description of my template",
  "author": "Your Name",
  "tags": ["category"],
  "platforms": ["windows", "macos", "linux"],
  "variables": {},
  "config": {
    "command": "command",
    "args": []
  }
}
EOF
```

### 3. Validate Your Template

```bash
# Install mcp-forge if you haven't already
uvx install mcp-forge

# Validate template
mcp-forge template validate templates/community/my-template.json
```

### 4. Update Catalog

Add your template to `catalog.json`:

```json
{
  "my-template": {
    "name": "my-template",
    "version": "1.0.0",
    "description": "Description of my template",
    "author": "Your Name",
    "tags": ["category"],
    "platforms": ["windows", "macos", "linux"],
    "category": "community",
    "path": "templates/community/my-template.json"
  }
}
```

### 5. Submit Pull Request

1. Test your template thoroughly
2. Ensure it follows the schema
3. Add documentation if needed
4. Submit a pull request with a clear description

## Template Guidelines

### Quality Standards

- ✅ Follow the JSON schema
- ✅ Include comprehensive variable descriptions
- ✅ Provide setup instructions
- ✅ Test on multiple platforms
- ✅ Use semantic versioning

### Security Best Practices

- 🔒 Never include hardcoded secrets
- 🔒 Use environment variables for sensitive data
- 🔒 Validate user inputs appropriately
- 🔒 Follow principle of least privilege

### Naming Conventions

- Template names: `lowercase-with-hyphens`
- Variable names: `snake_case` or `camelCase`
- Environment variables: `UPPER_CASE`

## Schema Validation

All templates are validated against the JSON schema in `schemas/template-schema.json`. You can validate your templates using:

```bash
# Using mcp-forge
mcp-forge template validate my-template.json

# Using a JSON schema validator
ajv validate -s schemas/template-schema.json -d templates/community/my-template.json
```

## Support

- 📖 [MCP-Forge Documentation](https://github.com/mcp-forge/mcp-forge)
- 🐛 [Report Issues](https://github.com/mcp-forge/templates/issues)
- 💬 [Discussions](https://github.com/mcp-forge/templates/discussions)

## License

This repository is licensed under the MIT License. See [LICENSE](LICENSE) for details.

---

**Happy templating!** 🚀 