use anyhow::{anyhow, Context, Result};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use crate::config::{Config, McpServer};
use crate::utils;
use clap::Subcommand;

/// Profile information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileInfo {
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_used: Option<chrono::DateTime<chrono::Utc>>,
    pub server_count: usize,
}

/// Global profile configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileConfig {
    pub current_profile: Option<String>,
    pub profiles: HashMap<String, ProfileInfo>,
}

impl Default for ProfileConfig {
    fn default() -> Self {
        Self {
            current_profile: None,
            profiles: HashMap::new(),
        }
    }
}

/// Handle profile command routing
pub async fn handle_profile_command(action: ProfileCommands) -> Result<()> {
    match action {
        ProfileCommands::Create { name } => {
            handle_profile_create(name).await
        }
        ProfileCommands::List => {
            handle_profile_list().await
        }
        ProfileCommands::Switch { name } => {
            handle_profile_switch(name).await
        }
        ProfileCommands::Current => {
            handle_profile_current().await
        }
        ProfileCommands::Sync { from, to, dry_run } => {
            handle_profile_sync(from, to, dry_run).await
        }
        ProfileCommands::Delete { name, force } => {
            handle_profile_delete(name, force).await
        }
    }
}

/// Create a new profile
async fn handle_profile_create(name: String) -> Result<()> {
    validate_profile_name(&name)?;
    
    let mut profile_config = load_profile_config().await?;
    
    if profile_config.profiles.contains_key(&name) {
        return Err(anyhow!("Profile '{}' already exists", name));
    }

    // Create profile info
    let profile_info = ProfileInfo {
        name: name.clone(),
        description: None,
        created_at: chrono::Utc::now(),
        last_used: None,
        server_count: 0,
    };

    // Add to profile config
    profile_config.profiles.insert(name.clone(), profile_info);
    
    // Create empty configuration for this profile
    let empty_config = Config::default();
    empty_config.save(Some(&name)).await?;
    
    // Save profile config
    save_profile_config(&profile_config).await?;
    
    println!("{}", format!("✓ Profile '{}' created successfully", name).green());
    println!("  Switch to it with: mcp-forge profile switch {}", name);
    
    Ok(())
}

/// List all profiles
async fn handle_profile_list() -> Result<()> {
    let profile_config = load_profile_config().await?;
    
    if profile_config.profiles.is_empty() {
        println!("{}", "No profiles found.".yellow());
        println!("Create a new profile with: mcp-forge profile create <name>");
        return Ok(());
    }

    println!("{}", "Available Profiles".cyan().bold());
    println!("{}", "─────────────────".cyan());

    let current = profile_config.current_profile.as_deref();
    
    for (name, info) in &profile_config.profiles {
        let status = if Some(name.as_str()) == current {
            "CURRENT".green().bold()
        } else {
            "".normal()
        };
        
        println!();
        println!("• {} {}", name.bold(), status);
        println!("  Created: {}", info.created_at.format("%Y-%m-%d %H:%M UTC"));
        
        if let Some(last_used) = info.last_used {
            println!("  Last used: {}", last_used.format("%Y-%m-%d %H:%M UTC"));
        }
        
        println!("  Servers: {}", info.server_count);
        
        if let Some(desc) = &info.description {
            println!("  Description: {}", desc.italic());
        }
    }

    println!();
    if let Some(current_name) = current {
        println!("Current profile: {}", current_name.green().bold());
    } else {
        println!("No profile currently selected (using default)");
        println!("Switch to a profile with: mcp-forge profile switch <name>");
    }

    Ok(())
}

/// Switch to a different profile
async fn handle_profile_switch(name: String) -> Result<()> {
    let mut profile_config = load_profile_config().await?;
    
    if !profile_config.profiles.contains_key(&name) {
        return Err(anyhow!("Profile '{}' does not exist", name));
    }

    // Update current profile
    profile_config.current_profile = Some(name.clone());
    
    // Update last used timestamp
    if let Some(profile_info) = profile_config.profiles.get_mut(&name) {
        profile_info.last_used = Some(chrono::Utc::now());
    }
    
    save_profile_config(&profile_config).await?;
    
    println!("{}", format!("✓ Switched to profile '{}'", name).green());
    
    // Show basic info about the profile
    if let Ok(config) = Config::load(Some(&name)).await {
        println!("  Servers in this profile: {}", config.mcpServers.len());
    }
    
    Ok(())
}

/// Show current profile
async fn handle_profile_current() -> Result<()> {
    let profile_config = load_profile_config().await?;
    
    if let Some(current_name) = &profile_config.current_profile {
        println!("Current profile: {}", current_name.green().bold());
        
        if let Some(profile_info) = profile_config.profiles.get(current_name) {
            println!("  Created: {}", profile_info.created_at.format("%Y-%m-%d %H:%M UTC"));
            if let Some(last_used) = profile_info.last_used {
                println!("  Last used: {}", last_used.format("%Y-%m-%d %H:%M UTC"));
            }
            println!("  Servers: {}", profile_info.server_count);
        }
        
        // Show servers in current profile
        if let Ok(config) = Config::load(Some(current_name)).await {
            if !config.mcpServers.is_empty() {
                println!();
                println!("Servers in this profile:");
                for name in config.mcpServers.keys() {
                    println!("  • {}", name);
                }
            }
        }
    } else {
        println!("No profile currently selected (using default configuration)");
        println!("Available profiles:");
        for name in profile_config.profiles.keys() {
            println!("  • {}", name);
        }
    }
    
    Ok(())
}

/// Sync configuration between profiles
async fn handle_profile_sync(from: String, to: String, dry_run: bool) -> Result<()> {
    let profile_config = load_profile_config().await?;
    
    // Validate profiles exist
    if !profile_config.profiles.contains_key(&from) {
        return Err(anyhow!("Source profile '{}' does not exist", from));
    }
    if !profile_config.profiles.contains_key(&to) {
        return Err(anyhow!("Target profile '{}' does not exist", to));
    }

    let source_config = Config::load(Some(&from)).await?;
    let target_config = Config::load(Some(&to)).await.unwrap_or_default();

    if dry_run {
        preview_profile_sync(&source_config, &target_config, &from, &to).await?;
        return Ok(());
    }

    println!("{}", format!("Syncing configuration from '{}' to '{}'...", from, to).cyan());

    // Copy the entire configuration
    source_config.save(Some(&to)).await?;
    
    println!("{}", format!("✓ Configuration synced successfully").green());
    println!("  Servers copied: {}", source_config.mcpServers.len());
    
    Ok(())
}

/// Delete a profile
async fn handle_profile_delete(name: String, force: bool) -> Result<()> {
    let mut profile_config = load_profile_config().await?;
    
    if !profile_config.profiles.contains_key(&name) {
        return Err(anyhow!("Profile '{}' does not exist", name));
    }

    // Check if it's the current profile
    if profile_config.current_profile.as_ref() == Some(&name) {
        if !force {
            return Err(anyhow!("Cannot delete current profile '{}'. Switch to another profile first or use --force", name));
        }
        profile_config.current_profile = None;
    }

    if !force {
        println!("Are you sure you want to delete profile '{}'?", name.red());
        if let Some(profile_info) = profile_config.profiles.get(&name) {
            println!("  Servers: {}", profile_info.server_count);
            println!("  Created: {}", profile_info.created_at.format("%Y-%m-%d"));
        }
        println!();
        print!("This action cannot be undone. Continue? [y/N]: ");
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if !input.trim().to_lowercase().starts_with('y') {
            println!("Profile deletion cancelled.");
            return Ok(());
        }
    }

    // Remove from profile config
    profile_config.profiles.remove(&name);
    save_profile_config(&profile_config).await?;
    
    // Delete the profile's configuration file
    if let Ok(config_path) = get_profile_config_path(&name) {
        if config_path.exists() {
            fs::remove_file(config_path)?;
        }
    }
    
    println!("{}", format!("✓ Profile '{}' deleted successfully", name).green());
    
    Ok(())
}

/// Preview profile sync operation
async fn preview_profile_sync(
    source: &Config,
    target: &Config,
    from_name: &str,
    to_name: &str,
) -> Result<()> {
    println!("{}", "Profile Sync Preview".cyan().bold());
    println!("{}", "───────────────────".cyan());
    println!("From: {} ({} servers)", from_name.bold(), source.mcpServers.len());
    println!("To: {} ({} servers)", to_name.bold(), target.mcpServers.len());
    println!();

    // Show what would be added/overwritten
    let mut new_servers = Vec::new();
    let mut overwritten_servers = Vec::new();
    
    for (name, _) in &source.mcpServers {
        if target.mcpServers.contains_key(name) {
            overwritten_servers.push(name);
        } else {
            new_servers.push(name);
        }
    }
    
    if !new_servers.is_empty() {
        println!("Servers to be added:");
        for name in new_servers {
            println!("  {} {}", "NEW".green(), name.bold());
        }
        println!();
    }
    
    if !overwritten_servers.is_empty() {
        println!("Servers to be overwritten:");
        for name in overwritten_servers {
            println!("  {} {}", "OVERWRITE".yellow(), name.bold());
        }
        println!();
    }
    
    // Show servers that would be removed from target
    let removed_servers: Vec<_> = target.mcpServers.keys()
        .filter(|name| !source.mcpServers.contains_key(*name))
        .collect();
    
    if !removed_servers.is_empty() {
        println!("Servers to be removed from target:");
        for name in removed_servers {
            println!("  {} {}", "REMOVE".red(), name.bold());
        }
        println!();
    }

    println!("Run without --dry-run to apply these changes.");
    
    Ok(())
}

/// Load profile configuration
async fn load_profile_config() -> Result<ProfileConfig> {
    let profile_path = get_profiles_config_path()?;
    
    if !profile_path.exists() {
        return Ok(ProfileConfig::default());
    }
    
    let content = fs::read_to_string(&profile_path)?;
    let config: ProfileConfig = serde_json::from_str(&content)?;
    Ok(config)
}

/// Save profile configuration
async fn save_profile_config(config: &ProfileConfig) -> Result<()> {
    let profile_path = get_profiles_config_path()?;
    
    // Create parent directory if needed
    if let Some(parent) = profile_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    let content = serde_json::to_string_pretty(config)?;
    fs::write(profile_path, content)?;
    
    Ok(())
}

/// Get path to profiles configuration file
fn get_profiles_config_path() -> Result<PathBuf> {
    let config_dir = utils::get_config_dir()?;
    Ok(config_dir.join("profiles.json"))
}

/// Get path to a specific profile's configuration
fn get_profile_config_path(profile_name: &str) -> Result<PathBuf> {
    let config_dir = utils::get_config_dir()?;
    Ok(config_dir.join(format!("profile_{}.json", profile_name)))
}

/// Validate profile name
fn validate_profile_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow!("Profile name cannot be empty"));
    }
    
    if name.len() > 50 {
        return Err(anyhow!("Profile name cannot be longer than 50 characters"));
    }
    
    // Check for invalid characters
    if name.chars().any(|c| !c.is_alphanumeric() && c != '-' && c != '_') {
        return Err(anyhow!("Profile name can only contain letters, numbers, hyphens, and underscores"));
    }
    
    // Reserved names
    if matches!(name.to_lowercase().as_str(), "default" | "main" | "config" | "global") {
        return Err(anyhow!("'{}' is a reserved profile name", name));
    }
    
    Ok(())
}

/// Update profile server count
pub async fn update_profile_server_count(profile_name: Option<&str>) -> Result<()> {
    if let Some(name) = profile_name {
        let mut profile_config = load_profile_config().await?;
        
        if let Some(profile_info) = profile_config.profiles.get_mut(name) {
            if let Ok(config) = Config::load(Some(name)).await {
                profile_info.server_count = config.mcpServers.len();
                save_profile_config(&profile_config).await?;
            }
        }
    }
    
    Ok(())
}

#[derive(Subcommand)]
pub enum ProfileCommands {
    /// Create new profile
    Create {
        /// Profile name
        name: String,
    },
    /// List available profiles
    List,
    /// Switch to profile
    Switch {
        /// Profile name
        name: String,
    },
    /// Show current profile
    Current,
    /// Sync configuration between profiles
    Sync {
        /// Source profile
        from: String,
        /// Target profile
        to: String,
        /// Preview changes without applying
        #[arg(long)]
        dry_run: bool,
    },
    /// Delete profile
    Delete {
        /// Profile name
        name: String,
        /// Force deletion without confirmation
        #[arg(long)]
        force: bool,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_profile_name() {
        // Valid names
        assert!(validate_profile_name("development").is_ok());
        assert!(validate_profile_name("prod-env").is_ok());
        assert!(validate_profile_name("test_2").is_ok());
        assert!(validate_profile_name("env123").is_ok());

        // Invalid names
        assert!(validate_profile_name("").is_err());
        assert!(validate_profile_name("name with spaces").is_err());
        assert!(validate_profile_name("name@domain").is_err());
        assert!(validate_profile_name("default").is_err());
        assert!(validate_profile_name("a".repeat(51).as_str()).is_err());
    }

    #[test]
    fn test_profile_config_serialization() {
        let mut config = ProfileConfig::default();
        config.current_profile = Some("test".to_string());
        
        let profile_info = ProfileInfo {
            name: "test".to_string(),
            description: Some("Test profile".to_string()),
            created_at: chrono::Utc::now(),
            last_used: None,
            server_count: 5,
        };
        
        config.profiles.insert("test".to_string(), profile_info);
        
        let json = serde_json::to_string(&config).unwrap();
        let parsed: ProfileConfig = serde_json::from_str(&json).unwrap();
        
        assert_eq!(parsed.current_profile, Some("test".to_string()));
        assert_eq!(parsed.profiles.len(), 1);
        assert_eq!(parsed.profiles["test"].server_count, 5);
    }
} 