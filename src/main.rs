use anyhow::Result;
use clap::{Parser, Subcommand};

mod cli;
mod config;
mod github;
mod templates;
mod utils;
mod validation;
mod backup;
mod bulk;
mod profiles;
mod search;


// Re-export enum types from their respective modules
pub use backup::BackupCommands;
pub use bulk::BulkCommands;
pub use profiles::ProfileCommands;

#[derive(Parser)]
#[command(name = "mcp-forge")]
#[command(about = "A CLI tool for managing Claude Desktop MCP server configurations")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Use specific profile
    #[arg(long, global = true)]
    profile: Option<String>,
    
    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// List MCP servers with advanced filtering
    List {
        /// Filter by name/command/args
        #[arg(short, long)]
        filter: Option<String>,
        /// Filter by tag
        #[arg(long)]
        tag: Option<String>,
        /// Filter by platform
        #[arg(long)]
        platform: Option<String>,
        /// Filter by author
        #[arg(long)]
        author: Option<String>,
        /// Filter by requirements
        #[arg(long)]
        requires: Option<String>,
        /// Sort by field (name, command, author)
        #[arg(long)]
        sort: Option<String>,
        /// Sort in descending order
        #[arg(long)]
        desc: bool,
        /// Output format (default, table, json)
        #[arg(long)]
        format: Option<String>,
        /// Show requirements
        #[arg(long)]
        show_requirements: bool,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Add new server from template
    Add {
        /// Server name
        name: String,
        /// Template name
        template: String,
        /// Variables as key=value pairs
        #[arg(long)]
        vars: Option<String>,
        /// Preview changes without applying
        #[arg(long)]
        dry_run: bool,
        /// Show diff of changes
        #[arg(long)]
        preview: bool,
    },
    /// Remove server(s)
    Remove {
        /// Server name or pattern
        name: Option<String>,
        /// Remove all servers
        #[arg(long)]
        all: bool,
        /// Pattern matching for bulk removal
        #[arg(long)]
        pattern: Option<String>,
        /// Skip confirmation prompts
        #[arg(long)]
        force: bool,
        /// Preview changes without applying
        #[arg(long)]
        dry_run: bool,
    },
    /// Edit server configuration
    Edit {
        /// Server name
        name: String,
        /// Preview changes without applying
        #[arg(long)]
        dry_run: bool,
    },
    /// Update server configuration
    Update {
        /// Server name or pattern
        name: Option<String>,
        /// New arguments
        #[arg(long)]
        args: Option<String>,
        /// Filter by tag for bulk updates
        #[arg(long)]
        tag: Option<String>,
        /// Set environment variables
        #[arg(long)]
        set: Vec<String>,
        /// Preview changes without applying
        #[arg(long)]
        dry_run: bool,
        /// Show diff of changes
        #[arg(long)]
        preview: bool,
    },
    /// Template operations
    Template {
        #[command(subcommand)]
        action: TemplateCommands,
    },
    /// Configuration operations
    Config {
        #[command(subcommand)]
        action: ConfigCommands,
    },
    /// Backup operations
    Backup {
        #[command(subcommand)]
        action: BackupCommands,
    },
    /// Bulk operations
    Bulk {
        #[command(subcommand)]
        action: BulkCommands,
    },
    /// Profile management
    Profile {
        #[command(subcommand)]
        action: ProfileCommands,
    },
    /// Validation and health checks
    Validate {
        /// Perform deep validation
        #[arg(long)]
        deep: bool,
        /// Validate system requirements
        #[arg(long)]
        requirements: bool,
        /// Server name to validate (all if not specified)
        server: Option<String>,
    },
    /// System health check
    Health,
    /// Validate all configurations
    ValidateAll,
    /// System diagnostic
    Doctor,
    /// Import configuration
    Import {
        /// Input file
        #[arg(long)]
        file: String,
        /// Merge with existing configuration
        #[arg(long)]
        merge: bool,
        /// Replace existing configuration
        #[arg(long)]
        replace: bool,
        /// Preview changes without applying
        #[arg(long)]
        dry_run: bool,
    },
    /// Export configuration
    Export {
        /// Output format (json, yaml, template)
        #[arg(long)]
        format: Option<String>,
        /// Export as template
        #[arg(long)]
        template: bool,
        /// Output file (stdout if not specified)
        #[arg(long)]
        output: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum TemplateCommands {
    /// List available templates
    List {
        /// Show cached templates only
        #[arg(long)]
        cached: bool,
        /// Show offline templates
        #[arg(long)]
        offline: bool,
    },
    /// Show template details
    Show {
        /// Template name
        name: String,
    },
    /// Search templates
    Search {
        /// Search term
        term: String,
        /// Rank by downloads
        #[arg(long)]
        rank_by: Option<String>,
        /// Filter by tag
        #[arg(long)]
        tag: Option<String>,
        /// Filter by platform
        #[arg(long)]
        platform: Option<String>,
    },
    /// Refresh template cache
    Refresh {
        /// Force refresh even if cache is valid
        #[arg(long)]
        force: bool,
        /// Clear cache before refresh
        #[arg(long)]
        clear: bool,
    },
    /// Create new template
    Create {
        /// Template name
        name: String,
    },
    /// Validate template
    Validate {
        /// Template file
        file: String,
    },
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Show current configuration
    Show,
    /// Validate configuration
    Validate {
        /// Perform deep validation
        #[arg(long)]
        deep: bool,
        /// Validate requirements
        #[arg(long)]
        requirements: bool,
    },
    /// Create backup
    Backup {
        /// Backup name
        #[arg(long)]
        name: Option<String>,
        /// Auto-generate name
        #[arg(long)]
        auto_name: bool,
    },
    /// Restore from backup
    Restore {
        /// Backup file or name
        backup: String,
        /// Preview restore without applying
        #[arg(long)]
        preview: bool,
        /// Restore specific server only
        #[arg(long)]
        server: Option<String>,
    },
    /// Initialize empty configuration
    Init,
    /// Show configuration file path
    Path,
}



#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Set up logging if verbose
    if cli.verbose {
        env_logger::init();
    }

    match cli.command {
        Commands::List { 
            filter, tag, platform, author, requires, sort, desc, 
            format, show_requirements, json 
        } => {
            let criteria = search::SearchCriteria {
                text: filter,
                tags: tag.map_or(vec![], |t| vec![t]),
                platform,
                author,
                requires,
            };
            let options = search::ListOptions {
                sort,
                desc,
                format,
                show_requirements,
                json,
            };
            cli::handle_enhanced_list(criteria, options, cli.profile).await
        }
        Commands::Add { name, template, vars, dry_run, preview } => {
            cli::handle_enhanced_add(name, template, vars, dry_run, preview, cli.profile).await
        }
        Commands::Remove { name, all, pattern, force, dry_run } => {
            cli::handle_enhanced_remove(name, all, pattern, force, dry_run, cli.profile).await
        }
        Commands::Edit { name, dry_run } => {
            cli::handle_enhanced_edit(name, dry_run, cli.profile).await
        }
        Commands::Update { name, args, tag, set, dry_run, preview } => {
            cli::handle_enhanced_update(name, args, tag, set, dry_run, preview, cli.profile).await
        }
        Commands::Template { action } => {
            cli::handle_template_command(action).await
        }
        Commands::Config { action } => {
            cli::handle_config_command(action).await
        }
        Commands::Backup { action } => {
            backup::handle_backup_command(action, cli.profile).await
        }
        Commands::Bulk { action } => {
            bulk::handle_bulk_command(action, cli.profile).await
        }
        Commands::Profile { action } => {
            profiles::handle_profile_command(action).await
        }
        Commands::Validate { deep, requirements, server } => {
            validation::handle_validate(deep, requirements, server, cli.profile).await
        }
        Commands::Health => {
            validation::handle_health_check(cli.profile).await
        }
        Commands::ValidateAll => {
            validation::handle_validate_all(cli.profile).await
        }
        Commands::Doctor => {
            validation::handle_doctor(cli.profile).await
        }
        Commands::Import { file, merge, replace, dry_run } => {
            cli::handle_import(file, merge, replace, dry_run, cli.profile).await
        }
        Commands::Export { format, template, output } => {
            cli::handle_export(format, template, output, cli.profile).await
        }
    }
}
