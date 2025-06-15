# Basic Usage Examples

This guide provides practical examples for getting started with MCP-Forge.

## Getting Started

### 1. Check Your Current Configuration

Before making any changes, see what's currently configured:

```bash
# List all configured servers
mcp-forge list

# Show configuration file location
mcp-forge config path

# Display current configuration
mcp-forge config show
```

### 2. Add Your First Server

Let's add a filesystem server to give Claude access to your documents:

```bash
# Add filesystem server (interactive mode)
mcp-forge add my-docs filesystem

# When prompted, enter the path to your documents
# Example: /Users/yourname/Documents
```

Or add it non-interactively:

```bash
# Add filesystem server with path specified
mcp-forge add my-docs filesystem --vars "path=/Users/yourname/Documents"
```

### 3. Verify the Server

Check that your server was added correctly:

```bash
# List servers to see your new addition
mcp-forge list

# Validate the server configuration
mcp-forge validate my-docs
```

### 4. Add a Search Server

Add web search capabilities with Brave Search:

```bash
# Add Brave Search server (you'll need an API key)
mcp-forge add web-search brave-search --vars "api_key=your_api_key_here"
```

## Common Operations

### Managing Servers

```bash
# List all servers
mcp-forge list

# List servers with filtering
mcp-forge list --filter "filesystem"

# Get JSON output for scripting
mcp-forge list --json

# Edit a server configuration
mcp-forge edit my-docs

# Update server arguments
mcp-forge update my-docs --args "path=/new/path"

# Remove a server
mcp-forge remove my-docs
```

### Working with Templates

```bash
# See available templates
mcp-forge template list

# Get details about a template
mcp-forge template show filesystem

# Validate a custom template
mcp-forge template validate my-template.json
```

### Configuration Management

```bash
# Show current configuration
mcp-forge config show

# Validate configuration
mcp-forge config validate

# Initialize new configuration
mcp-forge config init

# Show config file path
mcp-forge config path
```

## Real-World Scenarios

### Scenario 1: Developer Setup

A developer wants to give Claude access to their project files and enable web search:

```bash
# Add filesystem access to projects directory
mcp-forge add projects filesystem --vars "path=/Users/dev/Projects"

# Add web search capability
mcp-forge add search brave-search --vars "api_key=your_brave_api_key"

# Verify setup
mcp-forge list
mcp-forge validate-all
```

### Scenario 2: Content Creator Setup

A content creator needs access to their writing folder and research capabilities:

```bash
# Add access to writing directory
mcp-forge add writing filesystem --vars "path=/Users/writer/Documents/Writing"

# Add web search for research
mcp-forge add research brave-search --vars "api_key=your_api_key"

# List to confirm
mcp-forge list --json
```

### Scenario 3: Data Analyst Setup

A data analyst needs database access and file system access:

```bash
# Add SQLite database access
mcp-forge add analytics-db sqlite --vars "db_path=/path/to/analytics.db"

# Add filesystem access to data directory
mcp-forge add data-files filesystem --vars "path=/Users/analyst/Data"

# Validate all configurations
mcp-forge validate-all
```

## Safety and Backup

### Creating Backups

```bash
# Create a manual backup before making changes
mcp-forge backup create

# Create a named backup
mcp-forge backup create --name "before-major-changes"

# List all backups
mcp-forge backup list
```

### Restoring from Backup

```bash
# Restore from a specific backup
mcp-forge backup restore backup-2024-01-15.json

# List available backups first
mcp-forge backup list
```

## Troubleshooting

### Common Issues

#### Server Not Appearing in Claude

```bash
# Validate the server configuration
mcp-forge validate server-name

# Check system health
mcp-forge health

# Run full diagnostic
mcp-forge doctor
```

#### Configuration File Issues

```bash
# Check if config file exists and is valid
mcp-forge config validate

# Show config file location
mcp-forge config path

# Initialize new config if needed
mcp-forge config init
```

#### Template Issues

```bash
# Validate template
mcp-forge template validate template-name

# Show template details
mcp-forge template show template-name

# Use interactive mode for guidance
mcp-forge add server-name template-name --interactive
```

## Getting Help

```bash
# General help
mcp-forge --help

# Command-specific help
mcp-forge list --help
mcp-forge add --help
mcp-forge template --help

# Show version
mcp-forge --version
```

## Next Steps

Once you're comfortable with basic operations, explore:

- [Advanced Filtering](advanced-filtering.md) - Complex server filtering and search
- [Bulk Operations](bulk-operations.md) - Managing multiple servers at once
- [Profile Management](profile-management.md) - Multi-environment configurations
- [Template Development](template-development.md) - Creating custom templates

## Tips and Best Practices

1. **Always backup** before making significant changes
2. **Validate configurations** after modifications
3. **Use descriptive names** for your servers
4. **Test with dry-run** for bulk operations
5. **Keep templates updated** for security and features
6. **Use profiles** for different environments (dev, prod, etc.)
7. **Regular health checks** to ensure everything is working 