# Changelog

All notable changes to MCP-Forge will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive documentation overhaul
- Contributing guidelines
- Development setup instructions

## [0.3.1] - 2024-12-15 - Code Quality & Template Repository

### Added
- **Separate Template Repository**
  - Templates moved to dedicated [mcp-forge-templates](https://github.com/AndyCross/mcp-forge-templates) repository
  - Template catalog system for centralized template management
  - Automatic template fetching and caching from GitHub
  - Community-friendly template contribution workflow

- **Enhanced Template Commands**
  - `template refresh` - Refresh template cache from repository
  - `template search` - Search templates by tags and description
  - Improved template listing with repository links
  - Better template metadata display

### Enhanced
- **Code Quality Improvements**
  - Eliminated all 46 build warnings (46 → 0)
  - Fixed all 25 clippy warnings for better code quality
  - Removed ~600+ lines of dead/unused code
  - Improved code patterns and performance optimizations

- **CI/CD Pipeline**
  - Updated GitHub Actions to use non-deprecated versions (v4)
  - Fixed artifact upload/download actions
  - Improved release automation and binary builds
  - Enhanced cross-platform build reliability

- **Template System Architecture**
  - Catalog-based template fetching from separate repository
  - Improved template path resolution and error handling
  - Better template caching and offline support
  - Enhanced template validation and metadata

### Fixed
- All clippy warnings resolved (collapsible if statements, unnecessary map_or, etc.)
- GitHub Actions deprecated action warnings
- Template repository path resolution issues
- Code formatting compliance across all files
- Needless borrows and references for better performance

### Technical
- **Code Cleanup**
  - Replaced manual Default implementations with derive attributes
  - Improved field initialization patterns
  - Better async/await patterns and error handling
  - Removed unused functions and imports across all modules

- **Repository Architecture**
  - Templates separated into independent repository
  - GitHub client updated to fetch from new template repository
  - Template catalog system for metadata management
  - Improved modularity and maintainability

- **Build System**
  - Zero warnings in both debug and release modes
  - All 25 unit tests passing
  - Improved compilation times
  - Better error messages and debugging support

### Migration Notes
- Templates are now fetched from the separate repository automatically
- Existing template functionality remains unchanged for users
- Template cache will be refreshed automatically on first use
- No breaking changes to CLI commands or functionality

## [0.3.0] - 2024-01-15 - Sprint 3: Advanced Features & Polish

### Added
- **Advanced Search & Filtering**
  - Smart filtering by name, type, status, and custom criteria
  - Semantic search with natural language queries
  - Ranking system with relevance scoring
  - Multiple output formats (JSON, table, custom)

- **Configuration Validation & Health Checks**
  - Deep validation with comprehensive system checks
  - System requirements validation
  - Health monitoring and diagnostics
  - Custom validation rules support

- **Dry-Run & Preview System**
  - Safe operation previews before execution
  - Comprehensive change impact analysis
  - User confirmation workflows
  - Risk assessment for operations

- **Bulk Operations**
  - Multi-server management capabilities
  - Pattern-based server selection with glob patterns
  - Batch processing with progress tracking
  - Bulk update, validate, and remove operations

- **Advanced Backup & Restore**
  - Automated backup scheduling
  - Incremental backup support
  - Point-in-time recovery options
  - Cross-profile backup capabilities

- **Profile Support**
  - Multi-environment configuration management
  - Profile switching and isolation
  - Environment-specific settings
  - Profile-specific templates

- **Import/Export Functionality**
  - Configuration portability between systems
  - Cross-system migration support
  - Multiple format support (JSON, YAML)
  - Merge strategies for imports

- **New Commands**
  - `validate` - Configuration validation with deep checks
  - `health` - System health monitoring
  - `doctor` - System diagnostic and repair
  - `bulk` - Bulk operations management
  - `backup` - Backup and restore operations
  - `profile` - Profile management
  - `import` - Configuration import
  - `export` - Configuration export
  - `validate-all` - Validate all configurations

### Enhanced
- **CLI Interface**
  - Comprehensive help system for all commands
  - Improved error messages with actionable guidance
  - Progress indicators for long-running operations
  - Interactive confirmation prompts

- **Template System**
  - Enhanced variable validation
  - Support for complex variable types (objects, arrays)
  - Template metadata and versioning
  - Cross-platform compatibility checks

- **Configuration Management**
  - Robust configuration file handling
  - Atomic operations to prevent corruption
  - Configuration validation before saves
  - Backup creation before modifications

### Fixed
- All compilation errors resolved (78 → 0)
- Type resolution issues across modules
- Async/await handling in CLI operations
- Template variable substitution edge cases
- Configuration file path resolution
- Cross-platform compatibility issues

### Technical
- **New Modules**
  - `src/search.rs` - Advanced filtering and search
  - `src/validation.rs` - Configuration validation
  - `src/backup.rs` - Backup and restore operations
  - `src/bulk.rs` - Bulk operations management
  - `src/profiles.rs` - Profile management

- **Dependencies Added**
  - `env_logger = "0.10"` - Enhanced logging
  - Additional validation and utility crates

- **Architecture Improvements**
  - Modular design with clear separation of concerns
  - Comprehensive error handling with anyhow
  - Async/await throughout for better performance
  - Clean module interfaces and abstractions

## [0.2.0] - 2024-01-08 - Sprint 2: Template System

### Added
- **GitHub-based Template Repository**
  - Remote template fetching from GitHub
  - Local template caching for offline use
  - Template versioning and updates
  - Community template support

- **Advanced Template System**
  - Handlebars-powered variable substitution
  - Support for 5 variable types (string, number, boolean, array, object)
  - Template validation and schema checking
  - Interactive variable prompting

- **Template Management Commands**
  - `template list` - List available templates
  - `template show` - Display template details
  - `template validate` - Validate template files
  - `template create` - Create new templates

- **Enhanced Server Templates**
  - PostgreSQL database server template
  - SQLite database server template
  - GitHub API integration template
  - Expanded filesystem server options

### Enhanced
- **Configuration Management**
  - Improved config file detection across platforms
  - Better error handling for malformed configs
  - Enhanced backup system with timestamps

- **CLI Experience**
  - Interactive mode for template variable input
  - Better progress indicators
  - Improved help text and examples

### Fixed
- Template variable parsing edge cases
- Cross-platform path handling
- GitHub API rate limiting issues
- Configuration backup race conditions

## [0.1.0] - 2024-01-01 - Sprint 1: Foundation

### Added
- **Core Server Management**
  - `list` - List all configured MCP servers
  - `add` - Add new servers from templates
  - `remove` - Remove servers (single or all)
  - `edit` - Edit server configurations
  - `update` - Update server arguments

- **Cross-Platform Support**
  - Automatic Claude Desktop config detection
  - Support for Windows, macOS, and Linux
  - Platform-specific path handling

- **Basic Template System**
  - Filesystem server template
  - Brave Search API server template
  - Template variable substitution
  - Built-in template validation

- **Configuration Management**
  - `config show` - Display current configuration
  - `config path` - Show config file location
  - `config init` - Initialize new configuration
  - `config validate` - Validate configuration

- **Safety Features**
  - Automatic backup creation before modifications
  - Configuration validation before saves
  - Confirmation prompts for destructive operations
  - Atomic configuration updates

- **Output Formats**
  - Human-readable table output
  - JSON output for scripting
  - Colored terminal output
  - Verbose logging support

### Technical
- **Initial Architecture**
  - Rust-based CLI with clap for argument parsing
  - Serde for JSON configuration handling
  - Tokio for async operations
  - Anyhow for error handling

- **Project Structure**
  - Modular design with separate concerns
  - Comprehensive error handling
  - Cross-platform compatibility
  - Zero-dependency operation model

## [0.0.1] - 2023-12-25 - Initial Concept

### Added
- Project initialization
- Basic Rust project structure
- Initial CLI framework
- Core concept validation

---

## Release Notes

### v0.3.1 Highlights - Code Quality & Architecture
This maintenance release focuses on code quality, architecture improvements, and template system enhancement. The separation of templates into their own repository enables better community contributions and independent template updates.

**Key Achievements:**
- ✅ Zero build warnings (eliminated all 46 warnings)
- ✅ Zero clippy warnings (fixed all 25 code quality issues)
- ✅ Removed 600+ lines of dead code for cleaner codebase
- ✅ Templates moved to separate repository for better management
- ✅ Enhanced CI/CD pipeline with updated GitHub Actions
- ✅ All 25 unit tests passing with improved reliability

**Template Repository Benefits:**
- Independent template updates without new releases
- Community-friendly contribution workflow
- Centralized template management and discovery
- Automatic fetching and caching system

### Sprint 3 Highlights (v0.3.0)
This release represents a major milestone in MCP-Forge development, transforming it from a basic CLI tool into a comprehensive, enterprise-ready MCP server management solution. The addition of advanced features like bulk operations, profile management, and comprehensive validation makes MCP-Forge suitable for production environments and complex deployment scenarios.

**Key Achievements:**
- ✅ Zero compilation errors (resolved 78 compilation issues)
- ✅ 7 new modules with advanced functionality
- ✅ Comprehensive CLI with 15+ commands
- ✅ Enterprise-grade features (profiles, bulk ops, validation)
- ✅ Production-ready architecture

### Sprint 2 Highlights (v0.2.0)
The template system overhaul in Sprint 2 established MCP-Forge as a powerful tool for managing diverse MCP server configurations. The GitHub integration and advanced variable system provide the foundation for a rich ecosystem of community templates.

### Sprint 1 Highlights (v0.1.0)
Sprint 1 established the solid foundation that made all subsequent development possible. The focus on safety, cross-platform compatibility, and user experience created a reliable base for advanced features.

---

**Development Timeline:**
- **Sprint 1** (Week 1): Foundation & Core Operations ✅
- **Sprint 2** (Week 2-3): Template System & GitHub Integration ✅  
- **Sprint 3** (Week 4): Advanced Features & Polish ✅
- **Sprint 4** (Week 5): Documentation & Distribution 🚧
- **Sprint 5** (Week 6): Community & Launch 📋 