# Command Reference

Complete reference for all MCP-Forge commands and options.

## Global Options

These options are available for all commands:

```bash
--profile <PROFILE>    Use specific profile
-v, --verbose          Enable verbose output
-h, --help            Print help
-V, --version         Print version
```

## Server Management Commands

### `list` - List MCP servers

List configured MCP servers with advanced filtering options.

```bash
mcp-forge list [OPTIONS]
```

**Options:**
- `--filter <PATTERN>` - Filter servers by name pattern
- `--type <TYPE>` - Filter by server type
- `--status <STATUS>` - Filter by server status (active, inactive, error)
- `--search <TERM>` - Semantic search across server metadata
- `--sort <FIELD>` - Sort by field (name, type, created, modified)
- `--limit <N>` - Limit number of results
- `--json` - Output in JSON format
- `--table` - Output in table format (default)

**Examples:**
```bash
# List all servers
mcp-forge list

# Filter by name pattern
mcp-forge list --filter "api-*"

# Search for database servers
mcp-forge list --search "database"

# Get JSON output for scripting
mcp-forge list --json

# Sort by creation date, limit to 10
mcp-forge list --sort created --limit 10
```

### `add` - Add new server

Add a new MCP server from a template.

```bash
mcp-forge add <NAME> <TEMPLATE> [OPTIONS]
```

**Arguments:**
- `<NAME>` - Name for the new server
- `<TEMPLATE>` - Template to use

**Options:**
- `--vars <VARS>` - Template variables as key=value pairs
- `--interactive` - Interactive mode with prompts
- `--dry-run` - Preview changes without applying
- `--force` - Overwrite existing server

**Examples:**
```bash
# Add filesystem server interactively
mcp-forge add my-docs filesystem

# Add with variables specified
mcp-forge add my-fs filesystem --vars "path=/home/user/docs"

# Multiple variables
mcp-forge add search brave-search --vars "api_key=key123,timeout=30"

# Preview before adding
mcp-forge add test-server filesystem --vars "path=/tmp" --dry-run
```

### `remove` - Remove servers

Remove one or more MCP servers.

```bash
mcp-forge remove [NAME] [OPTIONS]
```

**Arguments:**
- `[NAME]` - Name of server to remove (optional if using patterns)

**Options:**
- `--all` - Remove all servers
- `--pattern <PATTERN>` - Remove servers matching pattern
- `--force` - Skip confirmation prompts
- `--dry-run` - Preview what would be removed

**Examples:**
```bash
# Remove specific server
mcp-forge remove my-server

# Remove all test servers
mcp-forge remove --pattern "test-*"

# Remove all servers (with confirmation)
mcp-forge remove --all

# Preview removal
mcp-forge remove --pattern "old-*" --dry-run
```

### `edit` - Edit server configuration

Edit server configuration in your default editor.

```bash
mcp-forge edit <NAME> [OPTIONS]
```

**Arguments:**
- `<NAME>` - Name of server to edit

**Options:**
- `--editor <EDITOR>` - Specify editor to use
- `--backup` - Create backup before editing

**Examples:**
```bash
# Edit server in default editor
mcp-forge edit my-server

# Use specific editor
mcp-forge edit my-server --editor vim

# Create backup before editing
mcp-forge edit my-server --backup
```

### `update` - Update server configuration

Update server configuration programmatically.

```bash
mcp-forge update <NAME> [OPTIONS]
```

**Arguments:**
- `<NAME>` - Name of server to update

**Options:**
- `--args <ARGS>` - Update server arguments
- `--template <TEMPLATE>` - Update to new template
- `--vars <VARS>` - Update template variables
- `--dry-run` - Preview changes

**Examples:**
```bash
# Update server arguments
mcp-forge update my-fs --args "path=/new/path"

# Update template variables
mcp-forge update my-api --vars "timeout=60,retries=3"

# Switch to new template
mcp-forge update my-server --template new-template
```

## Validation & Health Commands

### `validate` - Validate configurations

Validate server configurations with optional deep checks.

```bash
mcp-forge validate [SERVER] [OPTIONS]
```

**Arguments:**
- `[SERVER]` - Server name to validate (all if not specified)

**Options:**
- `--deep` - Perform deep validation
- `--requirements` - Validate system requirements
- `--strict` - Strict validation mode
- `--fix` - Attempt to fix issues automatically

**Examples:**
```bash
# Validate specific server
mcp-forge validate my-server

# Deep validation of all servers
mcp-forge validate --deep

# Validate system requirements
mcp-forge validate --requirements

# Strict validation with auto-fix
mcp-forge validate --strict --fix
```

### `validate-all` - Validate all configurations

Validate all server configurations.

```bash
mcp-forge validate-all [OPTIONS]
```

**Options:**
- `--deep` - Perform deep validation
- `--parallel` - Run validations in parallel
- `--continue-on-error` - Continue validation even if errors occur

### `health` - System health check

Check system health and MCP server status.

```bash
mcp-forge health [OPTIONS]
```

**Options:**
- `--detailed` - Show detailed health information
- `--json` - Output in JSON format
- `--fix` - Attempt to fix health issues

### `doctor` - System diagnostic

Run comprehensive system diagnostic.

```bash
mcp-forge doctor [OPTIONS]
```

**Options:**
- `--quick` - Quick diagnostic (skip detailed checks)
- `--fix` - Attempt to fix issues automatically
- `--report` - Generate diagnostic report

## Bulk Operations Commands

### `bulk` - Bulk operations

Perform operations on multiple servers.

```bash
mcp-forge bulk <OPERATION> [OPTIONS]
```

**Operations:**
- `update` - Update multiple servers
- `validate` - Validate multiple servers
- `remove` - Remove multiple servers
- `backup` - Backup multiple servers

**Options:**
- `--pattern <PATTERN>` - Server selection pattern
- `--filter <FILTER>` - Additional filtering
- `--dry-run` - Preview operations
- `--parallel` - Run operations in parallel
- `--continue-on-error` - Continue on individual failures

**Examples:**
```bash
# Update all API servers
mcp-forge bulk update --pattern "api-*" --args "timeout=60"

# Validate all production servers
mcp-forge bulk validate --pattern "prod-*" --deep

# Remove test servers with preview
mcp-forge bulk remove --pattern "test-*" --dry-run
```

## Backup & Restore Commands

### `backup` - Backup operations

Manage configuration backups.

```bash
mcp-forge backup <SUBCOMMAND> [OPTIONS]
```

**Subcommands:**

#### `create` - Create backup
```bash
mcp-forge backup create [OPTIONS]

--name <NAME>        Named backup
--auto               Auto-generated name
--compress           Compress backup file
```

#### `list` - List backups
```bash
mcp-forge backup list [OPTIONS]

--json               JSON output
--sort <FIELD>       Sort by field
```

#### `restore` - Restore backup
```bash
mcp-forge backup restore <BACKUP> [OPTIONS]

--force              Force restore without confirmation
--merge              Merge with existing configuration
```

#### `cleanup` - Cleanup old backups
```bash
mcp-forge backup cleanup [OPTIONS]

--keep <N>           Keep N most recent backups
--older-than <DAYS>  Remove backups older than N days
```

## Profile Management Commands

### `profile` - Profile operations

Manage configuration profiles for different environments.

```bash
mcp-forge profile <SUBCOMMAND> [OPTIONS]
```

**Subcommands:**

#### `list` - List profiles
```bash
mcp-forge profile list [OPTIONS]

--json               JSON output
--detailed           Show detailed information
```

#### `create` - Create profile
```bash
mcp-forge profile create <NAME> [OPTIONS]

--copy-from <PROFILE>  Copy from existing profile
--template <TEMPLATE>  Use profile template
```

#### `use` - Switch to profile
```bash
mcp-forge profile use <NAME>
```

#### `copy` - Copy profile
```bash
mcp-forge profile copy <SOURCE> <DEST>
```

#### `delete` - Delete profile
```bash
mcp-forge profile delete <NAME> [OPTIONS]

--force              Skip confirmation
```

#### `current` - Show current profile
```bash
mcp-forge profile current
```

## Template Management Commands

### `template` - Template operations

Manage MCP server templates.

```bash
mcp-forge template <SUBCOMMAND> [OPTIONS]
```

**Subcommands:**

#### `list` - List templates
```bash
mcp-forge template list [OPTIONS]

--category <CAT>     Filter by category
--json               JSON output
--detailed           Show detailed information
```

#### `show` - Show template details
```bash
mcp-forge template show <TEMPLATE> [OPTIONS]

--variables          Show only variables
--config             Show only configuration
```

#### `validate` - Validate template
```bash
mcp-forge template validate <FILE> [OPTIONS]

--strict             Strict validation
--schema <SCHEMA>    Use custom schema
```

#### `create` - Create new template
```bash
mcp-forge template create <NAME> [OPTIONS]

--interactive        Interactive creation
--from-server <NAME> Create from existing server
```

## Configuration Commands

### `config` - Configuration operations

Manage MCP-Forge configuration.

```bash
mcp-forge config <SUBCOMMAND> [OPTIONS]
```

**Subcommands:**

#### `show` - Show configuration
```bash
mcp-forge config show [OPTIONS]

--json               JSON output
--raw                Show raw configuration
```

#### `path` - Show config file path
```bash
mcp-forge config path
```

#### `init` - Initialize configuration
```bash
mcp-forge config init [OPTIONS]

--force              Overwrite existing
--template <TEMPLATE> Use configuration template
```

#### `validate` - Validate configuration
```bash
mcp-forge config validate [OPTIONS]

--strict             Strict validation
--fix                Attempt to fix issues
```

## Import/Export Commands

### `import` - Import configuration

Import configuration from external sources.

```bash
mcp-forge import [OPTIONS]
```

**Options:**
- `--file <FILE>` - Import from file
- `--url <URL>` - Import from URL
- `--format <FORMAT>` - Specify format (json, yaml)
- `--merge` - Merge with existing configuration
- `--dry-run` - Preview import

### `export` - Export configuration

Export configuration to external formats.

```bash
mcp-forge export [OPTIONS]
```

**Options:**
- `--output <FILE>` - Output file path
- `--format <FORMAT>` - Export format (json, yaml)
- `--servers <PATTERN>` - Export specific servers
- `--pretty` - Pretty-print output

## Environment Variables

MCP-Forge recognizes these environment variables:

- `CLAUDE_CONFIG_PATH` - Custom configuration file path
- `MCP_FORGE_PROFILE` - Default profile to use
- `EDITOR` - Default editor for configuration editing
- `MCP_FORGE_BACKUP_DIR` - Custom backup directory
- `RUST_LOG` - Logging level (error, warn, info, debug, trace)

## Exit Codes

- `0` - Success
- `1` - General error
- `2` - Configuration error
- `3` - Validation error
- `4` - Network error
- `5` - Permission error

## Configuration File Format

MCP-Forge uses the standard Claude Desktop configuration format:

```json
{
  "mcpServers": {
    "server-name": {
      "command": "command-to-run",
      "args": ["arg1", "arg2"],
      "env": {
        "ENV_VAR": "value"
      }
    }
  }
}
```

## Pattern Syntax

MCP-Forge supports glob patterns for server selection:

- `*` - Match any characters
- `?` - Match single character
- `[abc]` - Match any character in brackets
- `{a,b}` - Match any alternative
- `**` - Match directories recursively

**Examples:**
- `api-*` - All servers starting with "api-"
- `*-prod` - All servers ending with "-prod"
- `test-?` - test-1, test-2, etc.
- `{dev,test}-*` - All dev-* and test-* servers 