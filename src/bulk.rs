use crate::config::Config;
use crate::templates::TemplateManager;
use anyhow::{anyhow, Result};
use clap::Subcommand;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Batch server configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchServerConfig {
    pub name: String,
    pub template: String,
    pub vars: HashMap<String, String>,
}

/// Batch configuration file structure
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchConfig {
    pub servers: Vec<BatchServerConfig>,
}

/// Bulk operation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkOperationResult {
    pub server_name: String,
    pub operation: String,
    pub success: bool,
    pub message: String,
}

/// Handle bulk command routing
pub async fn handle_bulk_command(action: BulkCommands, profile: Option<String>) -> Result<()> {
    match action {
        BulkCommands::Add { file, dry_run } => handle_bulk_add(file, dry_run, profile).await,
        BulkCommands::Update {
            pattern,
            tag,
            set,
            dry_run,
        } => handle_bulk_update(pattern, tag, set, dry_run, profile).await,
        BulkCommands::Remove {
            pattern,
            force,
            dry_run,
        } => handle_bulk_remove(pattern, force, dry_run, profile).await,
    }
}

/// Handle bulk add from file
async fn handle_bulk_add(file_path: String, dry_run: bool, profile: Option<String>) -> Result<()> {
    let batch_config = load_batch_config(&file_path).await?;

    if dry_run {
        println!("{}", "Bulk Add Preview (Dry Run)".cyan().bold());
        println!("{}", "─────────────────────────".cyan());
    } else {
        println!("{}", "Bulk Adding Servers".cyan().bold());
        println!("{}", "──────────────────".cyan());
    }

    let mut config = Config::load(profile.as_deref()).await.unwrap_or_default();
    let template_manager = TemplateManager::new()?;
    let mut results = Vec::new();

    for server_config in &batch_config.servers {
        let result = if dry_run {
            preview_add_server(server_config, &config, &template_manager).await?
        } else {
            add_server_from_config(server_config, &mut config, &template_manager).await?
        };

        results.push(result);
    }

    display_bulk_results(&results, dry_run);

    if !dry_run {
        let success_count = results.iter().filter(|r| r.success).count();
        if success_count > 0 {
            config.save(profile.as_deref()).await?;
            println!();
            println!(
                "{}",
                format!("✅ Successfully added {} server(s)", success_count)
                    .green()
                    .bold()
            );
        }
    }

    Ok(())
}

/// Handle bulk update with pattern matching
async fn handle_bulk_update(
    pattern: Option<String>,
    tag: Option<String>,
    set_vars: Vec<String>,
    dry_run: bool,
    profile: Option<String>,
) -> Result<()> {
    let mut config = Config::load(profile.as_deref()).await?;

    if dry_run {
        println!("{}", "Bulk Update Preview (Dry Run)".cyan().bold());
        println!("{}", "───────────────────────────".cyan());
    } else {
        println!("{}", "Bulk Updating Servers".cyan().bold());
        println!("{}", "────────────────────".cyan());
    }

    // Parse environment variables to set
    let env_updates = parse_env_vars(&set_vars)?;

    // Find matching servers
    let matching_servers = find_matching_servers(&config, pattern.as_deref(), tag.as_deref())?;

    if matching_servers.is_empty() {
        println!("{}", "No servers match the specified criteria.".yellow());
        return Ok(());
    }

    let mut results = Vec::new();

    for server_name in &matching_servers {
        let result = if dry_run {
            preview_update_server(server_name, &env_updates, &config)
        } else {
            update_server_env(server_name, &env_updates, &mut config)
        };

        results.push(result);
    }

    display_bulk_results(&results, dry_run);

    if !dry_run {
        let success_count = results.iter().filter(|r| r.success).count();
        if success_count > 0 {
            config.save(profile.as_deref()).await?;
            println!();
            println!(
                "{}",
                format!("✅ Successfully updated {} server(s)", success_count)
                    .green()
                    .bold()
            );
        }
    }

    Ok(())
}

/// Handle bulk remove with pattern matching
async fn handle_bulk_remove(
    pattern: String,
    force: bool,
    dry_run: bool,
    profile: Option<String>,
) -> Result<()> {
    let mut config = Config::load(profile.as_deref()).await?;

    // Find matching servers
    let matching_servers = find_matching_servers(&config, Some(&pattern), None)?;

    if matching_servers.is_empty() {
        println!(
            "{}",
            format!("No servers match pattern '{}'", pattern).yellow()
        );
        return Ok(());
    }

    if dry_run {
        println!("{}", "Bulk Remove Preview (Dry Run)".cyan().bold());
        println!("{}", "─────────────────────────".cyan());
    } else {
        println!("{}", "Bulk Removing Servers".cyan().bold());
        println!("{}", "────────────────────".cyan());
    }

    println!("Servers matching pattern '{}':", pattern.bold());
    for server_name in &matching_servers {
        if let Some(server) = config.mcp_servers.get(server_name) {
            println!("  • {} - {}", server_name.bold(), server.command);
        }
    }

    if !dry_run && !force {
        println!();
        print!("Remove these {} server(s)? [y/N]: ", matching_servers.len());
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if !input.trim().to_lowercase().starts_with('y') {
            println!("Bulk removal cancelled.");
            return Ok(());
        }
    }

    if !dry_run {
        let mut removed_count = 0;
        for server_name in &matching_servers {
            if config.mcp_servers.remove(server_name).is_some() {
                removed_count += 1;
                println!("{}", format!("✓ Removed {}", server_name).green());
            } else {
                println!("{}", format!("✗ Failed to remove {}", server_name).red());
            }
        }

        if removed_count > 0 {
            config.save(profile.as_deref()).await?;
            println!();
            println!(
                "{}",
                format!("✅ Successfully removed {} server(s)", removed_count)
                    .green()
                    .bold()
            );
        }
    } else {
        println!();
        println!("🔍 Would remove {} server(s)", matching_servers.len());
    }

    Ok(())
}

/// Load batch configuration from file
async fn load_batch_config(file_path: &str) -> Result<BatchConfig> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| anyhow!("Failed to read batch config file '{}': {}", file_path, e))?;

    // Determine file format based on extension
    let path = Path::new(file_path);
    let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");

    match extension.to_lowercase().as_str() {
        "json" => serde_json::from_str(&content).map_err(|e| anyhow!("Invalid JSON format: {}", e)),
        "yaml" | "yml" => {
            serde_yaml::from_str(&content).map_err(|e| anyhow!("Invalid YAML format: {}", e))
        }
        _ => {
            // Try JSON first, then YAML
            serde_json::from_str(&content)
                .or_else(|_| serde_yaml::from_str(&content))
                .map_err(|e| anyhow!("Unable to parse file as JSON or YAML: {}", e))
        }
    }
}

/// Preview adding a server from batch config
async fn preview_add_server(
    server_config: &BatchServerConfig,
    config: &Config,
    template_manager: &TemplateManager,
) -> Result<BulkOperationResult> {
    // Check if server already exists
    if config.mcp_servers.contains_key(&server_config.name) {
        return Ok(BulkOperationResult {
            server_name: server_config.name.clone(),
            operation: "add".to_string(),
            success: false,
            message: "Server already exists (would overwrite)".to_string(),
        });
    }

    // Check if template exists
    let template_list = template_manager.list_templates().await?;
    let template_exists = template_list
        .iter()
        .any(|t| t.name == server_config.template);

    if !template_exists {
        return Ok(BulkOperationResult {
            server_name: server_config.name.clone(),
            operation: "add".to_string(),
            success: false,
            message: format!("Template '{}' not found", server_config.template),
        });
    }

    Ok(BulkOperationResult {
        server_name: server_config.name.clone(),
        operation: "add".to_string(),
        success: true,
        message: format!("Would add with template '{}'", server_config.template),
    })
}

/// Add server from batch configuration
async fn add_server_from_config(
    server_config: &BatchServerConfig,
    config: &mut Config,
    template_manager: &TemplateManager,
) -> Result<BulkOperationResult> {
    // Get template
    let template = match template_manager
        .load_template(&server_config.template)
        .await
    {
        Ok(template) => template,
        Err(e) => {
            return Ok(BulkOperationResult {
                server_name: server_config.name.clone(),
                operation: "add".to_string(),
                success: false,
                message: format!(
                    "Failed to load template '{}': {}",
                    server_config.template, e
                ),
            })
        }
    };

    let variables: HashMap<String, serde_json::Value> = server_config
        .vars
        .iter()
        .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
        .collect();

    let server = match template_manager.apply_template(&template, &variables) {
        Ok(server) => server,
        Err(e) => {
            return Ok(BulkOperationResult {
                server_name: server_config.name.clone(),
                operation: "add".to_string(),
                success: false,
                message: format!("Template application failed: {}", e),
            });
        }
    };

    config
        .mcp_servers
        .insert(server_config.name.clone(), server);
    Ok(BulkOperationResult {
        server_name: server_config.name.clone(),
        operation: "add".to_string(),
        success: true,
        message: "Added successfully".to_string(),
    })
}

/// Find servers matching pattern or tag
pub fn find_matching_servers(
    config: &Config,
    pattern: Option<&str>,
    _tag: Option<&str>, // TODO: Implement tag filtering when metadata is available
) -> Result<Vec<String>> {
    let mut matching = Vec::new();

    for (name, _server) in &config.mcp_servers {
        if let Some(pattern_str) = pattern {
            // Simple pattern matching - could be enhanced with regex
            if name.contains(pattern_str) {
                matching.push(name.clone());
            }
        } else {
            // If no pattern, return all servers
            matching.push(name.clone());
        }
    }

    if matching.is_empty() && pattern.is_some() {
        return Err(anyhow!(
            "No servers found matching pattern: {}",
            pattern.unwrap()
        ));
    }

    Ok(matching)
}

/// Parse environment variable assignments
pub fn parse_env_vars(set_vars: &[String]) -> Result<HashMap<String, String>> {
    let mut env_updates = HashMap::new();

    for var_assignment in set_vars {
        if let Some((key, value)) = var_assignment.split_once('=') {
            env_updates.insert(key.to_string(), value.to_string());
        } else {
            return Err(anyhow!(
                "Invalid environment variable assignment: '{}'. Use format KEY=VALUE",
                var_assignment
            ));
        }
    }

    Ok(env_updates)
}

/// Preview updating a server's environment
fn preview_update_server(
    server_name: &str,
    env_updates: &HashMap<String, String>,
    config: &Config,
) -> BulkOperationResult {
    if !config.mcp_servers.contains_key(server_name) {
        return BulkOperationResult {
            server_name: server_name.to_string(),
            operation: "update".to_string(),
            success: false,
            message: "Server not found".to_string(),
        };
    }

    let changes: Vec<String> = env_updates
        .iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect();

    BulkOperationResult {
        server_name: server_name.to_string(),
        operation: "update".to_string(),
        success: true,
        message: format!("Would set: {}", changes.join(", ")),
    }
}

/// Update server environment variables
fn update_server_env(
    server_name: &str,
    env_updates: &HashMap<String, String>,
    config: &mut Config,
) -> BulkOperationResult {
    if let Some(server) = config.mcp_servers.get_mut(server_name) {
        // Initialize env if it doesn't exist
        if server.env.is_none() {
            server.env = Some(HashMap::new());
        }

        // Apply updates
        if let Some(env) = &mut server.env {
            for (key, value) in env_updates {
                env.insert(key.clone(), value.clone());
            }
        }

        BulkOperationResult {
            server_name: server_name.to_string(),
            operation: "update".to_string(),
            success: true,
            message: "Environment updated".to_string(),
        }
    } else {
        BulkOperationResult {
            server_name: server_name.to_string(),
            operation: "update".to_string(),
            success: false,
            message: "Server not found".to_string(),
        }
    }
}

/// Display bulk operation results
fn display_bulk_results(results: &[BulkOperationResult], dry_run: bool) {
    let mut success_count = 0;
    let mut error_count = 0;

    for result in results {
        let status_symbol = if result.success {
            success_count += 1;
            "✓".green()
        } else {
            error_count += 1;
            "✗".red()
        };

        let operation_text = if dry_run {
            format!(
                "[{}] {}",
                result.operation.to_uppercase(),
                result.server_name
            )
        } else {
            result.server_name.clone()
        };

        println!(
            "{} {} - {}",
            status_symbol,
            operation_text.bold(),
            result.message
        );
    }

    println!();
    if dry_run {
        println!("Preview Summary:");
        println!(
            "  {} operation(s) would succeed",
            success_count.to_string().green()
        );
        if error_count > 0 {
            println!(
                "  {} operation(s) would fail",
                error_count.to_string().red()
            );
        }
    } else {
        println!("Operation Summary:");
        println!("  {} successful", success_count.to_string().green());
        if error_count > 0 {
            println!("  {} failed", error_count.to_string().red());
        }
    }
}

#[derive(Subcommand)]
pub enum BulkCommands {
    /// Add multiple servers from file
    Add {
        /// Input file (YAML or JSON)
        #[arg(long)]
        file: String,
        /// Preview changes without applying
        #[arg(long)]
        dry_run: bool,
    },
    /// Update multiple servers
    Update {
        /// Pattern to match server names
        #[arg(long)]
        pattern: Option<String>,
        /// Filter by tag
        #[arg(long)]
        tag: Option<String>,
        /// Set environment variables
        #[arg(long)]
        set: Vec<String>,
        /// Preview changes without applying
        #[arg(long)]
        dry_run: bool,
    },
    /// Remove multiple servers
    Remove {
        /// Pattern to match server names
        #[arg(long)]
        pattern: String,
        /// Force removal without confirmation
        #[arg(long)]
        force: bool,
        /// Preview changes without applying
        #[arg(long)]
        dry_run: bool,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::McpServer;

    #[test]
    fn test_parse_env_vars() {
        let vars = vec![
            "DEBUG=true".to_string(),
            "PORT=3000".to_string(),
            "HOST=localhost".to_string(),
        ];

        let parsed = parse_env_vars(&vars).unwrap();
        assert_eq!(parsed.get("DEBUG"), Some(&"true".to_string()));
        assert_eq!(parsed.get("PORT"), Some(&"3000".to_string()));
        assert_eq!(parsed.get("HOST"), Some(&"localhost".to_string()));
    }

    #[test]
    fn test_parse_env_vars_invalid() {
        let vars = vec!["INVALID_FORMAT".to_string()];
        assert!(parse_env_vars(&vars).is_err());
    }

    #[test]
    fn test_find_matching_servers() {
        let mut config = Config::default();
        config.mcp_servers.insert(
            "test-server-1".to_string(),
            McpServer {
                command: "cmd1".to_string(),
                args: vec![],
                env: None,
                other: HashMap::new(),
            },
        );
        config.mcp_servers.insert(
            "test-server-2".to_string(),
            McpServer {
                command: "cmd2".to_string(),
                args: vec![],
                env: None,
                other: HashMap::new(),
            },
        );
        config.mcp_servers.insert(
            "prod-server".to_string(),
            McpServer {
                command: "cmd3".to_string(),
                args: vec![],
                env: None,
                other: HashMap::new(),
            },
        );

        // Test pattern matching (contains)
        let matches = find_matching_servers(&config, Some("test-"), None).unwrap();
        assert_eq!(matches.len(), 2);
        assert!(matches.contains(&"test-server-1".to_string()));
        assert!(matches.contains(&"test-server-2".to_string()));

        // Test exact pattern
        let matches = find_matching_servers(&config, Some("prod-server"), None).unwrap();
        assert_eq!(matches.len(), 1);
        assert!(matches.contains(&"prod-server".to_string()));
    }

    #[test]
    fn test_batch_config_serialization() {
        let batch_config = BatchConfig {
            servers: vec![BatchServerConfig {
                name: "test1".to_string(),
                template: "filesystem".to_string(),
                vars: {
                    let mut vars = HashMap::new();
                    vars.insert("path".to_string(), "/tmp".to_string());
                    vars
                },
            }],
        };

        let json = serde_json::to_string(&batch_config).unwrap();
        let parsed: BatchConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.servers.len(), 1);
        assert_eq!(parsed.servers[0].name, "test1");
    }
}
