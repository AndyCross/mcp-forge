use anyhow::{anyhow, Context, Result};
use crate::config::{Config, ConfigManager, McpServer};
use crate::templates::{TemplateManager, VariableType};
use crate::github::GitHubClient;
use crate::{ConfigCommands, TemplateCommands};
use crate::search::{SearchCriteria, ListOptions, filter_servers, format_servers, rank_templates};
use crate::utils;
use inquire::{Confirm, Text, Select};
use std::collections::HashMap;
use colored::Colorize;
use std::fs;

/// Handle the list command
pub async fn handle_list(filter: Option<String>, json: bool) -> Result<()> {
    let config_manager = ConfigManager::new()?;
    let servers = config_manager.list_servers()?;

    let filtered_servers: Vec<_> = if let Some(filter_term) = &filter {
        servers
            .into_iter()
            .filter(|(name, server)| {
                name.contains(filter_term)
                    || server.command.contains(filter_term)
                    || server.args.iter().any(|arg| arg.contains(filter_term))
            })
            .collect()
    } else {
        servers
    };

    if json {
        let json_output: HashMap<String, &McpServer> = filtered_servers
            .iter()
            .map(|(name, server)| (name.clone(), server))
            .collect();
        println!("{}", serde_json::to_string_pretty(&json_output)?);
    } else {
        if filtered_servers.is_empty() {
            if filter.is_some() {
                println!("No MCP servers found matching the filter criteria.");
            } else {
                println!("No MCP servers configured.");
            }
            return Ok(());
        }

        println!("Configured MCP Servers:");
        println!("─────────────────────");
        
        for (name, server) in &filtered_servers {
            println!("• {}", name);
            println!("  Command: {}", server.command);
            if !server.args.is_empty() {
                println!("  Args: {}", server.args.join(" "));
            }
            if let Some(env) = &server.env {
                if !env.is_empty() {
                    println!("  Environment:");
                    for (key, value) in env {
                        println!("    {}={}", key, value);
                    }
                }
            }
            println!();
        }
        
        println!("Total: {} server(s)", filtered_servers.len());
    }

    Ok(())
}

/// Handle the add command with enhanced template support
pub async fn handle_add(name: String, template: String, vars: Option<String>) -> Result<()> {
    let mut config_manager = ConfigManager::new()?;
    let template_manager = TemplateManager::new()?;
    
    // Check if server already exists
    if config_manager.get_server(&name)?.is_some() {
        let overwrite = Confirm::new(&format!("Server '{}' already exists. Overwrite?", name))
            .with_default(false)
            .prompt()?;
        
        if !overwrite {
            println!("Operation cancelled.");
            return Ok(());
        }
    }

    // Create backup before modification
    let backup_dir = utils::get_backup_dir()?;
    if backup_dir.exists() {
        config_manager.create_backup().await?;
    }

    // Load template
    let template_obj = match template_manager.load_template(&template).await {
        Ok(t) => t,
        Err(e) => {
            // Try fallback to built-in templates if GitHub fails
            eprintln!("{}", GitHubClient::create_github_error_message(&e));
            eprintln!("\nFalling back to built-in template...");
            
            if let Some(builtin_template) = crate::github::create_mock_template(&template) {
                builtin_template
            } else {
                anyhow::bail!("Template '{}' not found in built-in templates either", template);
            }
        }
    };

    // Gather variables
    let variables = if let Some(vars_str) = vars {
        // Non-interactive mode: parse variables
        parse_vars_to_json(&vars_str)?
    } else {
        // Interactive mode: prompt for variables
        prompt_for_template_variables(&template_obj).await?
    };

    // Apply template
    let server = template_manager.apply_template(&template_obj, &variables)?;

    // Add server to configuration
    config_manager.add_server(name.clone(), server)?;
    
    // Show success message with setup instructions
    println!("✅ Successfully added server '{}'", name);
    if let Some(instructions) = &template_obj.setup_instructions {
        println!("\n📋 Setup Instructions:");
        println!("{}", instructions);
    }
    
    if let Some(requirements) = &template_obj.requirements {
        println!("\n⚠️  Requirements:");
        for (req, version) in requirements {
            println!("  • {}: {}", req, version);
        }
    }
    
    Ok(())
}

/// Handle the remove command
pub async fn handle_remove(name: Option<String>, all: bool) -> Result<()> {
    let mut config_manager = ConfigManager::new()?;

    if all {
        let confirm = Confirm::new("Are you sure you want to remove ALL MCP servers?")
            .with_default(false)
            .prompt()?;
        
        if !confirm {
            println!("Operation cancelled.");
            return Ok(());
        }

        // Create backup before modification
        let backup_dir = utils::get_backup_dir()?;
        if backup_dir.exists() {
            config_manager.create_backup().await?;
        }

        let servers = config_manager.list_servers()?;
        let count = servers.len();
        
        for (server_name, _) in servers {
            config_manager.remove_server(&server_name)?;
        }
        
        println!("Removed {} server(s)", count);
    } else if let Some(name) = name {
        // Create backup before modification
        let backup_dir = utils::get_backup_dir()?;
        if backup_dir.exists() {
            config_manager.create_backup().await?;
        }

        let removed = config_manager.remove_server(&name)?;
        if removed {
            println!("Successfully removed server '{}'", name);
        } else {
            println!("Server '{}' not found", name);
        }
    } else {
        anyhow::bail!("Must specify either a server name or --all flag");
    }

    Ok(())
}

/// Handle the edit command
pub async fn handle_edit(name: String) -> Result<()> {
    let mut config_manager = ConfigManager::new()?;
    
    let server = config_manager.get_server(&name)?
        .with_context(|| format!("Server '{}' not found", name))?;

    println!("Editing server '{}'", name);
    println!("Current configuration:");
    println!("  Command: {}", server.command);
    println!("  Args: {}", server.args.join(" "));

    let new_command = Text::new("Command:")
        .with_default(&server.command)
        .prompt()?;

    let new_args_str = Text::new("Arguments (space-separated):")
        .with_default(&server.args.join(" "))
        .prompt()?;

    let new_args: Vec<String> = if new_args_str.trim().is_empty() {
        Vec::new()
    } else {
        new_args_str.split_whitespace().map(|s| s.to_string()).collect()
    };

    // Create backup before modification
    let backup_dir = utils::get_backup_dir()?;
    if backup_dir.exists() {
        config_manager.create_backup().await?;
    }

    let updated_server = McpServer {
        command: new_command,
        args: new_args,
        env: server.env,
        other: server.other,
    };

    config_manager.add_server(name.clone(), updated_server)?;
    println!("Successfully updated server '{}'", name);

    Ok(())
}

/// Handle the update command
pub async fn handle_update(name: String, args: Option<String>) -> Result<()> {
    let mut config_manager = ConfigManager::new()?;
    
    let mut server = config_manager.get_server(&name)?
        .with_context(|| format!("Server '{}' not found", name))?;

    if let Some(new_args) = args {
        // Create backup before modification
        let backup_dir = utils::get_backup_dir()?;
        if backup_dir.exists() {
            config_manager.create_backup().await?;
        }

        server.args = if new_args.trim().is_empty() {
            Vec::new()
        } else {
            new_args.split_whitespace().map(|s| s.to_string()).collect()
        };

        config_manager.add_server(name.clone(), server)?;
        println!("Successfully updated arguments for server '{}'", name);
    } else {
        println!("No updates specified. Use --args to update arguments.");
    }

    Ok(())
}

/// Handle template commands with enhanced functionality
pub async fn handle_template_command(action: TemplateCommands) -> Result<()> {
    let template_manager = TemplateManager::new()?;
    
    match action {
        TemplateCommands::List { cached, offline } => {
            handle_template_list(cached, offline).await?
        }
        TemplateCommands::Show { name } => {
            handle_template_show(name).await?
        }
        TemplateCommands::Search { term, rank_by, tag, platform } => {
            handle_template_search(term, rank_by, tag, platform).await?
        }
        TemplateCommands::Refresh { force, clear } => {
            handle_template_refresh(force, clear).await?
        }
        TemplateCommands::Create { name: _ } => {
            println!("Template creation functionality will be available in Sprint 3.");
        }
        TemplateCommands::Validate { file } => {
            println!("Validating template file: {}", file);
            
            let content = std::fs::read_to_string(&file)
                .with_context(|| format!("Failed to read template file: {}", file))?;
            
            match template_manager.validate_template(&content) {
                Ok(template) => {
                    println!("✅ Template '{}' is valid!", template.name);
                    println!("   Version: {}", template.version);
                    println!("   Variables: {}", template.variables.len());
                }
                Err(e) => {
                    eprintln!("❌ Template validation failed: {}", e);
                }
            }
        }
    }
    Ok(())
}

/// Handle config commands
pub async fn handle_config_command(action: ConfigCommands) -> Result<()> {
    let config_manager = ConfigManager::new()?;

    match action {
        ConfigCommands::Show => {
            let mut config_manager = ConfigManager::new()?;
            config_manager.load_config().await?;
            let config = Config::load(None).await?;
            println!("{}", serde_json::to_string_pretty(&config)?);
        }
        ConfigCommands::Validate { deep, requirements } => {
            let profile = None; // TODO: Get from global args
            crate::validation::validate_config(deep, requirements, None, profile).await?
        }
        ConfigCommands::Backup { name, auto_name } => {
            let profile = None; // TODO: Get from global args
            crate::backup::create_backup_with_options(name, auto_name, profile).await?
        }
        ConfigCommands::Restore { backup, preview, server } => {
            let profile = None; // TODO: Get from global args
            crate::backup::restore_backup(backup, preview, server, profile).await?
        }
        ConfigCommands::Init => {
            let config = Config::default();
            config.save(None).await?;
            println!("✅ Initialized empty configuration");
        }
        ConfigCommands::Path => {
            let path = utils::get_claude_config_path()?;
            println!("{}", path.display());
        }
    }
    Ok(())
}

/// Prompt for template variables interactively
async fn prompt_for_template_variables(template: &crate::templates::Template) -> Result<HashMap<String, serde_json::Value>> {
    let mut values = HashMap::new();
    
    if template.variables.is_empty() {
        return Ok(values);
    }
    
    println!("Please provide values for template variables:");
    
    for (name, variable) in &template.variables {
        let value = match &variable.var_type {
            VariableType::String => {
                let mut prompt = Text::new(name);
                if !variable.description.is_empty() {
                    prompt = prompt.with_help_message(&variable.description);
                }
                if let Some(default) = &variable.default {
                    if let Some(default_str) = default.as_str() {
                        prompt = prompt.with_default(default_str);
                    }
                }
                serde_json::Value::String(prompt.prompt()?)
            }
            VariableType::Boolean => {
                let default = variable.default.as_ref()
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                let confirm = Confirm::new(name).with_default(default);
                serde_json::Value::Bool(confirm.prompt()?)
            }
            VariableType::Number => {
                let mut prompt = Text::new(name);
                if let Some(default) = &variable.default {
                    if let Some(default_str) = default.as_str() {
                        prompt = prompt.with_default(default_str);
                    }
                }
                let input = prompt.prompt()?;
                serde_json::Value::String(input)
            }
            VariableType::Array => {
                let prompt_text = format!("{} (comma-separated)", name);
                let mut prompt = Text::new(&prompt_text);
                if let Some(default) = &variable.default {
                    if let Some(default_str) = default.as_str() {
                        prompt = prompt.with_default(default_str);
                    }
                }
                let input = prompt.prompt()?.split(',').map(|s| s.trim().to_string()).collect::<Vec<_>>();
                serde_json::Value::Array(input.into_iter().map(serde_json::Value::String).collect())
            }
            VariableType::Select => {
                if let Some(options) = &variable.options {
                    let selected = Select::new(name, options.clone()).prompt()?;
                    serde_json::Value::String(selected)
                } else {
                    return Err(anyhow!("Select variable '{}' has no options defined", name));
                }
            }
        };
        
        values.insert(name.clone(), value);
    }
    
    Ok(values)
}

/// Parse variables from string format
fn parse_vars_to_json(vars_str: &str) -> Result<HashMap<String, serde_json::Value>> {
    let mut variables = HashMap::new();
    
    for pair in vars_str.split(',') {
        let pair = pair.trim();
        if let Some((key, value)) = pair.split_once('=') {
            variables.insert(key.trim().to_string(), serde_json::Value::String(value.trim().to_string()));
        } else {
            return Err(anyhow!("Invalid variable format: '{}'. Use KEY=VALUE format", pair));
        }
    }
    
    Ok(variables)
}

/// Create filesystem server
fn create_filesystem_server(vars: Option<String>) -> Result<McpServer> {
    let variables = if let Some(vars_str) = vars {
        parse_vars(&vars_str)?
    } else {
        HashMap::new()
    };

    let template_manager = TemplateManager::new()?;

    let paths = variables.get("paths")
        .cloned()
        .unwrap_or_else(|| "~/Desktop,~/Downloads".to_string());

    let paths_vec: Vec<String> = paths
        .split(',')
        .map(|p| p.trim().to_string())
        .collect();

    let mut args = vec!["-y".to_string(), "@modelcontextprotocol/server-filesystem".to_string()];
    args.extend(paths_vec);

    Ok(McpServer {
        command: "npx".to_string(),
        args,
        env: None,
        other: HashMap::new(),
    })
}

/// Create a brave search server configuration (legacy function for fallback)
fn create_brave_search_server(vars: Option<String>) -> Result<McpServer> {
    let api_key = if let Some(vars_str) = vars {
        parse_vars(&vars_str)?
            .get("api_key")
            .cloned()
            .context("api_key variable is required for brave-search template")?
    } else {
        Text::new("Brave Search API Key:")
            .prompt()?
    };

    let mut env = HashMap::new();
    env.insert("BRAVE_API_KEY".to_string(), api_key);

    Ok(McpServer {
        command: "npx".to_string(),
        args: vec!["-y".to_string(), "@modelcontextprotocol/server-brave-search".to_string()],
        env: Some(env),
        other: HashMap::new(),
    })
}

/// Handle enhanced list command with filtering and formatting
pub async fn handle_enhanced_list(
    criteria: SearchCriteria,
    options: ListOptions,
    profile: Option<String>,
) -> Result<()> {
    let config = Config::load(profile.as_deref()).await?;
    
    if config.mcpServers.is_empty() {
        println!("{}", "No MCP servers configured.".yellow());
        println!("Add a server with: mcp-forge add <name> <template>");
        return Ok(());
    }

    // Convert to list format
    let servers: Vec<(String, McpServer)> = config.mcpServers.into_iter().collect();
    
    // Apply filtering
    let filtered_servers = filter_servers(servers, &criteria);
    
    // Apply sorting
    let sorted_servers = crate::search::sort_servers(filtered_servers, &options);
    
    // Format and display
    let output = format_servers(&sorted_servers, &options);
    println!("{}", output);
    
    Ok(())
}

/// Handle enhanced add command with dry-run and preview
pub async fn handle_enhanced_add(
    name: String,
    template: String,
    vars: Option<String>,
    dry_run: bool,
    preview: bool,
    profile: Option<String>,
) -> Result<()> {
    let mut config = Config::load(profile.as_deref()).await.unwrap_or_default();
    let template_manager = TemplateManager::new()?;
    
    // Check if server already exists
    if config.mcpServers.contains_key(&name) {
        if !dry_run {
            let overwrite = Confirm::new(&format!("Server '{}' already exists. Overwrite?", name))
                .with_default(false)
                .prompt()?;
            if !overwrite {
                println!("Operation cancelled.");
                return Ok(());
            }
        } else {
            println!("{}", format!("Would overwrite existing server '{}'", name).yellow());
        }
    }

    // Get template
    let template_def = template_manager.load_template(&template).await?;
    
    // Parse variables
    let variable_values = if let Some(vars_str) = vars {
        parse_vars_to_json(&vars_str)?
    } else if !dry_run {
        prompt_for_template_variables(&template_def).await?
    } else {
        HashMap::new()
    };

    // Apply template
    let server = template_manager.apply_template(&template_def, &variable_values)?;
    
    if dry_run || preview {
        preview_add_operation(&name, &server, &config, dry_run).await?;
        return Ok(());
    }

    // Create backup before modification
    let backup_dir = utils::get_backup_dir()?;
    if backup_dir.exists() {
        config.create_backup().await?;
    }
    
    // Add server
    config.mcpServers.insert(name.clone(), server);
    config.save(profile.as_deref()).await?;

    println!("{}", format!("✓ Server '{}' added successfully", name).green());
    
    Ok(())
}

/// Handle enhanced remove command with pattern matching and dry-run
pub async fn handle_enhanced_remove(
    name: Option<String>,
    all: bool,
    pattern: Option<String>,
    force: bool,
    dry_run: bool,
    profile: Option<String>,
) -> Result<()> {
    let mut config = Config::load(profile.as_deref()).await?;

    let servers_to_remove = if all {
        config.mcpServers.keys().cloned().collect::<Vec<_>>()
    } else if let Some(pattern_str) = pattern {
        crate::bulk::find_matching_servers(&config, Some(&pattern_str), None)?
    } else if let Some(server_name) = name {
        if config.mcpServers.contains_key(&server_name) {
            vec![server_name]
        } else {
            return Err(anyhow!("Server '{}' not found", server_name));
        }
    } else {
        return Err(anyhow!("Must specify server name, pattern, or --all"));
    };

    if servers_to_remove.is_empty() {
        println!("{}", "No servers to remove.".yellow());
        return Ok(());
    }

    if dry_run {
        println!("{}", "Remove Preview (Dry Run)".cyan().bold());
        println!("{}", "────────────────────".cyan());
        for server_name in &servers_to_remove {
            if let Some(server) = config.mcpServers.get(server_name) {
                println!("  {} {} - {}", "REMOVE".red(), server_name.bold(), server.command);
            }
        }
        println!();
        println!("{}", format!("Would remove {} server(s)", servers_to_remove.len()).cyan());
        return Ok(());
    }

    // Confirm removal
    if !force {
        println!("Servers to be removed:");
        for server_name in &servers_to_remove {
            if let Some(server) = config.mcpServers.get(server_name) {
                println!("  • {} - {}", server_name.bold(), server.command);
            }
        }
        
        let confirm = Confirm::new(&format!("Remove {} server(s)?", servers_to_remove.len()))
            .with_default(false)
            .prompt()?;
        if !confirm {
            println!("Removal cancelled.");
            return Ok(());
        }
    }

    // Create backup before modification
    let backup_dir = utils::get_backup_dir()?;
    if backup_dir.exists() {
        config.create_backup().await?;
    }
    
    // Remove servers
    let mut removed_count = 0;
    for server_name in &servers_to_remove {
        if config.mcpServers.remove(server_name).is_some() {
            removed_count += 1;
            println!("{}", format!("✓ Removed {}", server_name).green());
        }
    }

    config.save(profile.as_deref()).await?;
    println!();
    println!("{}", format!("✅ Successfully removed {} server(s)", removed_count).green().bold());

    Ok(())
}

/// Handle enhanced edit command with dry-run
pub async fn handle_enhanced_edit(
    name: String,
    dry_run: bool,
    profile: Option<String>,
) -> Result<()> {
    let mut config = Config::load(profile.as_deref()).await?;
    
    let server = config.mcpServers.get(&name)
        .ok_or_else(|| anyhow!("Server '{}' not found", name))?
        .clone();

    if dry_run {
        preview_edit_operation(&name, &server).await?;
        return Ok(());
    }

    println!("{}", format!("Editing server '{}'", name).cyan());
    
    // Edit server configuration
    let edited_server = edit_server_interactive(&server).await?;
    
    // Show diff
    show_server_diff(&server, &edited_server, &name).await?;
    
    let confirm = Confirm::new("Apply these changes?")
        .with_default(true)
        .prompt()?;
    
    if !confirm {
        println!("Edit cancelled.");
        return Ok(());
    }

    // Create backup before modification
    let backup_dir = utils::get_backup_dir()?;
    if backup_dir.exists() {
        config.create_backup().await?;
    }
    
    // Update server
    config.mcpServers.insert(name.clone(), edited_server);
    config.save(profile.as_deref()).await?;

    println!("{}", format!("✓ Server '{}' updated successfully", name).green());
    
    Ok(())
}

/// Handle enhanced update command with bulk operations
pub async fn handle_enhanced_update(
    name: Option<String>,
    args: Option<String>,
    tag: Option<String>,
    set_env: Vec<String>,
    dry_run: bool,
    preview: bool,
    profile: Option<String>,
) -> Result<()> {
    let mut config = Config::load(profile.as_deref()).await?;
    
    // Determine servers to update
    let servers_to_update = if let Some(server_name) = name {
        if config.mcpServers.contains_key(&server_name) {
            vec![server_name]
        } else {
            return Err(anyhow!("Server '{}' not found", server_name));
        }
    } else if tag.is_some() {
        // TODO: Implement tag-based filtering when metadata is available
        return Err(anyhow!("Tag-based filtering not yet implemented"));
    } else {
        return Err(anyhow!("Must specify server name or tag"));
    };

    // Parse environment variables
    let env_updates = if !set_env.is_empty() {
        crate::bulk::parse_env_vars(&set_env)?
    } else {
        HashMap::new()
    };

    if dry_run || preview {
        preview_update_operation(&servers_to_update, &args, &env_updates, &config).await?;
        return Ok(());
    }

    // Create backup before modification
    let backup_dir = utils::get_backup_dir()?;
    if backup_dir.exists() {
        config.create_backup().await?;
    }
    
    // Apply updates
    let mut updated_count = 0;
    for server_name in &servers_to_update {
        if let Some(server) = config.mcpServers.get_mut(server_name) {
            let mut changed = false;
            
            // Update arguments
            if let Some(new_args) = &args {
                let parsed_args: Vec<String> = new_args
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();
                server.args = parsed_args;
                changed = true;
            }
            
            // Update environment variables
            if !env_updates.is_empty() {
                if server.env.is_none() {
                    server.env = Some(HashMap::new());
                }
                if let Some(env) = &mut server.env {
                    for (key, value) in &env_updates {
                        env.insert(key.clone(), value.clone());
                    }
                }
                changed = true;
            }
            
            if changed {
                updated_count += 1;
                println!("{}", format!("✓ Updated {}", server_name).green());
            }
        }
    }

    config.save(profile.as_deref()).await?;
    println!();
    println!("{}", format!("✅ Successfully updated {} server(s)", updated_count).green().bold());

    Ok(())
}

/// Preview add operation
async fn preview_add_operation(
    name: &str,
    server: &McpServer,
    config: &Config,
    dry_run: bool,
) -> Result<()> {
    let title = if dry_run {
        "Add Preview (Dry Run)".cyan().bold()
    } else {
        "Add Preview".cyan().bold()
    };
    
    println!("{}", title);
    println!("{}", "─────────────────".cyan());
    
    let status = if config.mcpServers.contains_key(name) {
        "OVERWRITE".yellow()
    } else {
        "NEW".green()
    };
    
    println!("{} {}", status, name.bold());
    println!("  Command: {}", server.command);
    if !server.args.is_empty() {
        println!("  Arguments: {}", server.args.join(" "));
    }
    if let Some(env) = &server.env {
        if !env.is_empty() {
            println!("  Environment:");
            for (key, value) in env {
                println!("    {}={}", key, value);
            }
        }
    }
    
    Ok(())
}

/// Preview edit operation
async fn preview_edit_operation(name: &str, server: &McpServer) -> Result<()> {
    println!("{}", "Edit Preview (Dry Run)".cyan().bold());
    println!("{}", "────────────────────".cyan());
    println!("Server: {}", name.bold());
    println!("  Current command: {}", server.command);
    if !server.args.is_empty() {
        println!("  Current arguments: {}", server.args.join(" "));
    }
    println!();
    println!("Use without --dry-run to edit interactively.");
    
    Ok(())
}

/// Preview update operation
async fn preview_update_operation(
    servers: &[String],
    args: &Option<String>,
    env_updates: &HashMap<String, String>,
    config: &Config,
) -> Result<()> {
    println!("{}", "Update Preview".cyan().bold());
    println!("{}", "─────────────".cyan());
    
    for server_name in servers {
        if let Some(server) = config.mcpServers.get(server_name) {
            println!("Server: {}", server_name.bold());
            
            if let Some(new_args) = args {
                println!("  Arguments: {} → {}", 
                         server.args.join(" ").dimmed(),
                         new_args.cyan());
            }
            
            if !env_updates.is_empty() {
                println!("  Environment updates:");
                for (key, value) in env_updates {
                    println!("    {}={}", key.cyan(), value.cyan());
                }
            }
            
            println!();
        }
    }
    
    Ok(())
}

/// Show diff between two server configurations
async fn show_server_diff(old: &McpServer, new: &McpServer, name: &str) -> Result<()> {
    println!("\n{} Changes for server '{}':", "📝".cyan(), name);
    
    // Check command changes
    if old.command != new.command {
        println!("  Command: {} → {}", old.command.red(), new.command.green());
    }
    
    // Check args changes
    if old.args != new.args {
        println!("  Args: {} → {}", 
            old.args.join(" ").red(), 
            new.args.join(" ").green());
    }
    
    // Check env changes with proper lifetimes
    let empty_env = HashMap::new();
    let old_env = old.env.as_ref().unwrap_or(&empty_env);
    let new_env = new.env.as_ref().unwrap_or(&empty_env);
    
    if old_env != new_env {
        println!("  Environment variables:");
        
        // Show removed variables
        for (key, value) in old_env {
            if !new_env.contains_key(key) {
                println!("    {} {}: {}", "-".red(), key.red(), value.red());
            }
        }
        
        // Show added/changed variables
        for (key, value) in new_env {
            if let Some(old_value) = old_env.get(key) {
                if old_value != value {
                    println!("    {} {}: {} → {}", "~".yellow(), key, old_value.red(), value.green());
                }
            } else {
                println!("    {} {}: {}", "+".green(), key.green(), value.green());
            }
        }
    }
    
    Ok(())
}

/// Interactive server editor
async fn edit_server_interactive(server: &McpServer) -> Result<McpServer> {
    let mut edited = server.clone();
    
    // Edit command
    let new_command = Text::new("Command:")
        .with_initial_value(&server.command)
        .prompt()?;
    edited.command = new_command;
    
    // Edit arguments
    let args_string = server.args.join(" ");
    let new_args_string = Text::new("Arguments:")
        .with_initial_value(&args_string)
        .prompt()?;
    edited.args = new_args_string
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    
    // Edit environment variables
    if let Some(env) = &server.env {
        if !env.is_empty() {
            let edit_env = Confirm::new("Edit environment variables?")
                .with_default(false)
                .prompt()?;
            
            if edit_env {
                let mut new_env = HashMap::new();
                for (key, value) in env {
                    let new_value = Text::new(&format!("{}:", key))
                        .with_initial_value(value)
                        .prompt()?;
                    new_env.insert(key.clone(), new_value);
                }
                edited.env = Some(new_env);
            }
        }
    }
    
    Ok(edited)
}

/// Parse variables string (KEY=VALUE,KEY2=VALUE2)
fn parse_vars_string(vars: &str) -> Result<HashMap<String, String>> {
    let mut result = HashMap::new();
    
    for pair in vars.split(',') {
        if let Some((key, value)) = pair.split_once('=') {
            result.insert(key.trim().to_string(), value.trim().to_string());
        } else {
            return Err(anyhow!("Invalid variable format: '{}'. Use KEY=VALUE format", pair));
        }
    }
    
    Ok(result)
}

/// Prompt for template variables
async fn prompt_for_variables(template: &crate::templates::Template) -> Result<HashMap<String, String>> {
    let mut values = HashMap::new();
    
    for (name, variable) in &template.variables {
        let value = match &variable.var_type {
            VariableType::String => {
                let mut prompt = Text::new(name);
                if !variable.description.is_empty() {
                    prompt = prompt.with_help_message(&variable.description);
                }
                if let Some(default) = &variable.default {
                    if let Some(default_str) = default.as_str() {
                        prompt = prompt.with_default(default_str);
                    }
                }
                prompt.prompt()?
            }
            VariableType::Boolean => {
                let default = variable.default.as_ref()
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                let confirm = Confirm::new(name).with_default(default);
                confirm.prompt()?.to_string()
            }
            VariableType::Number => {
                let mut prompt = Text::new(name);
                if let Some(default) = &variable.default {
                    if let Some(default_str) = default.as_str() {
                        prompt = prompt.with_default(default_str);
                    }
                }
                prompt.prompt()?
            }
            VariableType::Array => {
                let prompt_text = format!("{} (comma-separated)", name);
                let mut prompt = Text::new(&prompt_text);
                if let Some(default) = &variable.default {
                    if let Some(default_str) = default.as_str() {
                        prompt = prompt.with_default(default_str);
                    }
                }
                prompt.prompt()?.split(',').map(|s| s.trim().to_string()).collect::<Vec<_>>().join(",")
            }
            VariableType::Select => {
                if let Some(options) = &variable.options {
                    let selected = Select::new(name, options.clone()).prompt()?;
                    selected
                } else {
                    return Err(anyhow!("Select variable '{}' has no options defined", name));
                }
            }
        };
        
        values.insert(name.clone(), value);
    }
    
    Ok(values)
}

/// Load configuration from file
async fn load_config_from_file(file_path: &str) -> Result<Config> {
    let content = fs::read_to_string(file_path)?;
    
    // Try JSON first, then YAML
    serde_json::from_str(&content)
        .or_else(|_| serde_yaml::from_str(&content))
        .map_err(|e| anyhow!("Failed to parse config file: {}", e))
}

/// Merge two configurations
fn merge_configs(current: &Config, import: &Config) -> Result<Config> {
    let mut merged = current.clone();
    
    // Merge servers (import overwrites existing)
    for (name, server) in &import.mcpServers {
        merged.mcpServers.insert(name.clone(), server.clone());
    }
    
    Ok(merged)
}

/// Export configuration as JSON
fn export_as_json(config: &Config) -> Result<String> {
    serde_json::to_string_pretty(config)
        .map_err(|e| anyhow!("Failed to serialize config as JSON: {}", e))
}

/// Export configuration as YAML
fn export_as_yaml(config: &Config) -> Result<String> {
    serde_yaml::to_string(config)
        .map_err(|e| anyhow!("Failed to serialize config as YAML: {}", e))
}

/// Export configuration as template
fn export_as_template(config: &Config) -> Result<String> {
    // Create a template structure from the current configuration
    let template_servers: Vec<_> = config.mcpServers.iter()
        .map(|(name, server)| {
            serde_json::json!({
                "name": name,
                "command": server.command,
                "args": server.args,
                "env": server.env
            })
        })
        .collect();

    let template = serde_json::json!({
        "servers": template_servers
    });

    serde_json::to_string_pretty(&template)
        .map_err(|e| anyhow!("Failed to create template: {}", e))
}

// Template command implementations
async fn handle_template_list(cached: bool, offline: bool) -> Result<()> {
    let template_manager = TemplateManager::new()?;
    
    if offline || cached {
        // Show cached templates only
        if let Some(catalog) = template_manager.load_cached_catalog()? {
            println!("📦 Cached Templates:");
            for (name, metadata) in catalog.templates {
                println!("  • {} - {}", name, metadata.description);
                println!("    Author: {} | Platforms: {}", metadata.author, metadata.platforms.join(", "));
            }
        } else {
            println!("No cached templates available. Run 'mcp-forge template refresh' first.");
        }
        return Ok(());
    }

    let templates = template_manager.list_templates().await?;

    if templates.is_empty() {
        println!("{}", "No templates available.".yellow());
        return Ok(());
    }

    println!("{}", "Available Templates".cyan().bold());
    println!("{}", "──────────────────".cyan());

    for template in templates {
        println!();
        println!("• {} ({})", template.name.bold(), template.category.dimmed());
        println!("  {}", template.description);
        if !template.tags.is_empty() {
            println!("  Tags: {}", template.tags.join(", ").dimmed());
        }
        println!("  Platforms: {}", template.platforms.join(", ").dimmed());
    }

    Ok(())
}

async fn handle_template_show(name: String) -> Result<()> {
    let template_manager = TemplateManager::new()?;
    let template = template_manager.load_template(&name).await?;
    
    println!("{}", format!("Template: {}", template.name).cyan().bold());
    println!("{}", "─".repeat(template.name.len() + 10).cyan());
    
    println!("Name: {}", template.name);
    println!("Version: {}", template.version);
    println!("Author: {}", template.author);
    println!("Description: {}", template.description);
    println!("Platforms: {}", template.platforms.join(", "));
    println!("Tags: {}", template.tags.join(", "));
    
    if !template.variables.is_empty() {
        println!("\nVariables:");
        for (var_name, var) in &template.variables {
            print!("  • {} ({:?})", var_name.bold(), var.var_type);
            if !var.description.is_empty() {
                println!(" - {}", var.description);
            } else {
                println!();
            }
            if let Some(default) = &var.default {
                println!("    Default: {}", default);
            }
        }
    }
    
    println!("\nConfiguration:");
    println!("Command: {}", template.config.command);
    if !template.config.args.is_empty() {
        println!("Arguments: {}", template.config.args.join(" "));
    }
    
    if let Some(env) = &template.config.env {
        if !env.is_empty() {
            println!("Environment:");
            for (key, value) in env {
                println!("  {}={}", key, value);
            }
        }
    }
    
    if let Some(requirements) = &template.requirements {
        println!("\nRequirements:");
        for (req, version) in requirements {
            println!("  {}: {}", req, version);
        }
    }
    
    if let Some(instructions) = &template.setup_instructions {
        println!("\nSetup Instructions:");
        println!("{}", instructions);
    }
    
    Ok(())
}

async fn handle_template_search(
    term: String,
    rank_by: Option<String>,
    tag: Option<String>,
    platform: Option<String>,
) -> Result<()> {
    let template_manager = TemplateManager::new()?;
    let mut templates = template_manager.list_templates().await?;
    
    // Apply filters
    if let Some(tag_filter) = tag {
        templates.retain(|t| t.tags.contains(&tag_filter));
    }
    
    if let Some(platform_filter) = platform {
        templates.retain(|t| t.platforms.contains(&platform_filter));
    }
    
    // Rank templates
    let ranked = rank_templates(templates, &term, rank_by.as_deref());
    
    if ranked.is_empty() {
        println!("{}", "No templates found matching the search criteria.".yellow());
        return Ok(());
    }

    println!("{}", format!("Search Results for '{}'", term).cyan().bold());
    println!("{}", "─".repeat(20 + term.len()).cyan());

    for (template, ranking) in ranked.iter().take(10) {
        println!();
        println!("• {} ({})", template.name.bold(), template.category.dimmed());
        println!("  {}", template.description);
        println!("  {} Score: {:.2} | Downloads: {} | Rating: {:.1}★", 
                 "📊".dimmed(),
                 ranking.relevance_score + ranking.quality_score,
                 ranking.download_count,
                 ranking.community_rating);
    }

    Ok(())
}

async fn handle_template_refresh(force: bool, clear: bool) -> Result<()> {
    let template_manager = TemplateManager::new()?;
    
    if clear {
        template_manager.clear_cache()?;
        println!("🗑️  Template cache cleared.");
    }
    
    if force {
        println!("🔄 Force refreshing template cache...");
    } else {
        println!("🔄 Refreshing template cache...");
    }
    
    match template_manager.refresh_cache().await {
        Ok(()) => {
            println!("✅ Template cache refreshed successfully!");
        }
        Err(e) => {
            eprintln!("{}", GitHubClient::create_github_error_message(&e));
        }
    }
    
    Ok(())
}

async fn handle_template_create(_name: String) -> Result<()> {
    println!("{}", "Template creation not yet implemented.".yellow());
    println!("This feature will be added in a future release.");
    Ok(())
}

async fn handle_template_validate(_file: String) -> Result<()> {
    println!("{}", "Template validation not yet implemented.".yellow());
    println!("This feature will be added in a future release.");
    Ok(())
}

// Config command implementations
async fn handle_config_show() -> Result<()> {
    let config = Config::load(None).await?;
    
    println!("{}", serde_json::to_string_pretty(&config)?);
    Ok(())
}

async fn handle_config_init() -> Result<()> {
    let config = Config::default();
    config.save(None).await?;
    
    println!("{}", "✓ Configuration file initialized".green());
    if let Ok(path) = utils::get_claude_config_path() {
        println!("  Location: {}", path.display());
    }
    
    Ok(())
}

async fn handle_config_path() -> Result<()> {
    let path = utils::get_claude_config_path()?;
    println!("{}", path.display());
    Ok(())
}

/// Handle configuration import
pub async fn handle_import(
    file: String,
    merge: bool,
    replace: bool,
    dry_run: bool,
    profile: Option<String>,
) -> Result<()> {
    let config = load_config_from_file(&file).await?;
    
    if dry_run {
        println!("🔍 Would import configuration from: {}", file);
        println!("  Servers to import: {}", config.mcpServers.len());
        for (name, server) in &config.mcpServers {
            println!("    • {} ({})", name, server.command);
        }
        return Ok(());
    }
    
    let current_config = Config::load(profile.as_deref()).await.unwrap_or_default();
    
    if replace {
        // Replace entire configuration
        config.save(profile.as_deref()).await?;
        println!("✅ Configuration replaced from: {}", file);
    } else if merge {
        // Merge configurations
        let merged = merge_configs(&current_config, &config)?;
        merged.save(profile.as_deref()).await?;
        println!("✅ Configuration merged from: {}", file);
    } else {
        // Default behavior - show what would be done
        println!("Configuration preview from: {}", file);
        println!("Servers to import: {}", config.mcpServers.len());
        
        let confirm = Confirm::new("Import this configuration?")
            .with_default(false)
            .prompt()?;
            
        if confirm {
            let merged = merge_configs(&current_config, &config)?;
            merged.save(profile.as_deref()).await?;
            println!("✅ Configuration imported from: {}", file);
        }
    }
    
    Ok(())
}

/// Handle configuration export
pub async fn handle_export(
    format: Option<String>,
    template: bool,
    output: Option<String>,
    profile: Option<String>,
) -> Result<()> {
    let config = Config::load(profile.as_deref()).await?;
    
    let content = if template {
        export_as_template(&config)?
    } else {
        match format.as_deref() {
            Some("yaml") => export_as_yaml(&config)?,
            Some("json") | None => export_as_json(&config)?,
            Some(f) => return Err(anyhow!("Unsupported format: {}", f)),
        }
    };
    
    if let Some(output_path) = output {
        std::fs::write(&output_path, content)?;
        println!("✅ Configuration exported to: {}", output_path);
    } else {
        println!("{}", content);
    }
    
    Ok(())
}

/// Parse variables from string format (for compatibility)
fn parse_vars(vars_str: &str) -> Result<HashMap<String, String>> {
    let mut variables = HashMap::new();
    
    for pair in vars_str.split(',') {
        let pair = pair.trim();
        if let Some((key, value)) = pair.split_once('=') {
            variables.insert(key.trim().to_string(), value.trim().to_string());
        } else {
            return Err(anyhow!("Invalid variable format: '{}'. Use KEY=VALUE format", pair));
        }
    }
    
    Ok(variables)
} 