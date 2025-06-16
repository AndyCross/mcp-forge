# Changelog

All notable changes to MCP-Forge will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive documentation overhaul
- Contributing guidelines
- Development setup instructions

## [0.4.1] - 2025-01-16 - Profile System Consistency Fix

### Fixed
- **🔧 Profile Server Count Synchronization**
  - Fixed inconsistency where profile metadata showed incorrect server counts
  - Profile metadata now automatically updates when servers are added, removed, or modified
  - All server operations (add, remove, edit, update, bulk, backup restore, import) now update profile metadata
  - Resolved issue where `mcp-forge list` showed different server counts than `mcp-forge profile current`

- **🎯 Automatic Profile Detection**
  - Commands now automatically use the current active profile when no `--profile` parameter is specified
  - Fixed issue where operations were incorrectly using default configuration instead of active profile
  - Enhanced `Config::load()` and `Config::save()` to detect current profile automatically
  - Improved user experience by eliminating need to specify `--profile` for every command

### Enhanced
- **Profile System Reliability**
  - All profile metadata fields (server_count, last_used) now stay synchronized
  - Profile switching and syncing operations maintain accurate metadata
  - Consistent behavior across all commands and operations
  - Better error handling for profile-related operations

### Technical
- **New Functions**
  - `get_current_profile()` - Automatically detects active profile from profiles.json
  - `update_profile_server_count()` - Updates profile metadata after server modifications
  - Enhanced profile auto-detection in Config::load() and Config::save()

- **Updated Operations**
  - All CLI commands (add, remove, edit, update) now update profile metadata
  - All bulk operations maintain profile consistency
  - Backup restore operations update profile metadata
  - Import/export operations respect profile system

### Bug Fixes
- Fixed server count discrepancies between profile metadata and actual configuration
- Fixed commands using wrong configuration when profile is active
- Fixed profile sync not updating target profile metadata
- Fixed bulk operations not updating profile metadata

### Testing
- All 28 tests continue to pass
- Added comprehensive testing for profile consistency
- Verified fix with real-world usage scenarios
- No breaking changes or regressions introduced

## [0.4.0] - 2025-01-16 - Profile Management & Search Improvements

### Added
- **🔄 Default Profile Sync Support**
  - New `mcp-forge profile sync default <profile>` command
  - Enables migration from default configuration to profile-based approach
  - Perfect for users adopting profile workflows after initial setup
  - Supports dry-run preview with `--dry-run` flag
  - Comprehensive error handling and validation

- **🎯 Enhanced Template Search**
  - Improved search ranking to show only matching templates
  - Exact matches now return only the relevant template
  - Eliminated irrelevant results cluttering search output
  - Better search experience with focused, relevant results
  - Maintained support for partial matches and description searches

### Enhanced
- **Profile Management Workflow**
  - Seamless migration path from default to profile-based configuration
  - Clear documentation and help text for new sync functionality
  - Maintains backward compatibility with existing profile operations
  - Enhanced user experience for profile adoption

- **Search User Experience**
  - More intuitive search results with better relevance filtering
  - Faster template discovery with focused results
  - Improved search performance by filtering non-matching templates
  - Better ranking algorithm for template suggestions

### Technical
- **Profile Sync Implementation**
  - Special handling for "default" keyword in sync operations
  - Loads main Claude Desktop configuration when syncing from default
  - Maintains existing validation for regular profile names
  - Added comprehensive test coverage for new functionality

- **Search Algorithm Improvements**
  - Added relevance score filtering (> 0.0) to exclude non-matches
  - Improved template ranking with better score calculation
  - Enhanced search result ordering and presentation
  - Maintained existing search functionality for broader queries

### Usage Examples
```bash
# Migrate from default to profile-based approach
mcp-forge profile create development
mcp-forge profile sync default development --dry-run  # Preview
mcp-forge profile sync default development            # Execute
mcp-forge profile switch development

# Better template search experience
mcp-forge template search rightmove    # Returns only rightmove template
mcp-forge template search database     # Returns postgres + sqlite
```

### Developer Notes
- Added test coverage for default profile sync logic
- Updated help documentation to clarify "default" usage
- Maintained all existing functionality and backward compatibility
- Enhanced error messages for better user guidance

## [0.3.3] - 2024-12-15 - Security Enhancement: Environment Variable Masking

### Added
- **🔒 Security Feature: Environment Variable Masking**
  - Automatic detection and masking of sensitive environment variables
  - Smart pattern recognition for CLIENT_ID, CLIENT_SECRET, API_KEY, TOKEN, SECRET, PASSWORD, etc.
  - Case-insensitive matching with support for various separators (_, -, .)
  - Secure masking showing first 3 + last 3 characters with asterisks in between
  - Comprehensive coverage across all output locations

- **🛡️ Credential Leak Prevention**
  - Applied to `mcp-forge list` command output
  - Applied to `--dry-run` and `--preview` operations
  - Applied to template display and bulk operations
  - Applied to server diff displays and configuration changes
  - Applied to all environment variable outputs throughout the application

- **🧪 Comprehensive Testing**
  - New test suite for environment variable masking functionality
  - 26 total tests passing with security coverage
  - Manual testing verified with real credential patterns
  - Edge case testing for short values and various patterns

### Enhanced
- **Security Posture**
  - Prevents accidental credential exposure in command output
  - Maintains usability while protecting sensitive information
  - Zero performance impact on normal operations
  - Configurable masking patterns for different credential types

- **Developer Experience**
  - Clear visual indication of masked vs unmasked values
  - Consistent masking behavior across all commands
  - Maintains debugging capability while protecting secrets
  - No breaking changes to existing functionality

### Technical
- **New Function**: `mask_sensitive_env_value()` in `src/utils.rs`
- **Pattern Detection**: 11 sensitive patterns with normalization
- **Masking Logic**: Minimum 4 asterisks, preserves first/last 3 characters
- **Coverage**: Applied to 7 different output locations
- **Testing**: Comprehensive test coverage with edge cases

### Security Notes
- **Before**: `REDDIT_CLIENT_SECRET=KJCYTuWHOKRIaE0qx_SfimX1j_PHag`
- **After**: `REDDIT_CLIENT_SECRET=KJC************************Hag`
- Non-sensitive variables like `PORT`, `HOST` remain unmasked
- Masking is applied consistently across all command outputs

## [0.3.2] - 2024-12-15 - Development Tools & Automation

### Added
- **Development Tools & Automation**
  - Comprehensive Makefile with common development tasks
  - Automated version management across all packaging files
  - Version consistency checking and validation
  - Development workflow automation (build, test, check, release)

- **CI/CD Enhancements**
  - GitHub Actions workflow for automatic packaging file updates
  - Automated version synchronization on releases
  - Enhanced release pipeline with version management
  - Cross-platform binary build improvements

- **Version Management System**
  - `scripts/update-version.sh` - Automated version synchronization script
  - Version consistency validation across Cargo.toml, Scoop, and Homebrew
  - Automated packaging file updates on release creation
  - Developer-friendly version management commands

- **Documentation Improvements**
  - Comprehensive version management documentation
  - Development tools usage guide
  - Enhanced README with development workflow
  - Makefile command reference

### Enhanced
- **Developer Experience**
  - Streamlined development workflow with make commands
  - Automated quality checks (formatting, clippy, tests)
  - One-command release preparation
  - Reduced manual overhead for version management

- **Release Process**
  - Automated packaging file updates via GitHub Actions
  - Version consistency enforcement
  - Simplified release workflow
  - Better release documentation

### Technical
- **New Files**
  - `Makefile` - Development task automation
  - `scripts/update-version.sh` - Version synchronization script
  - `.github/workflows/update-packaging.yml` - Automated packaging updates
  - `docs/version-management.md` - Version management documentation

- **Build System**
  - Make-based development workflow
  - Automated version management
  - Enhanced CI/CD pipeline
  - Cross-platform compatibility maintained

### Developer Notes
- Use `make help` to see all available development commands
- Version updates now automatically sync across all packaging files
- GitHub Actions will update packaging files when releases are created
- See `docs/version-management.md` for detailed workflow information

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