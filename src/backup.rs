use crate::config::Config;
use crate::profiles::update_profile_server_count;
use crate::utils;
use anyhow::{anyhow, Result};
use chrono::{DateTime, Duration, Utc};
use clap::Subcommand;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Backup metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupMetadata {
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub servers_count: usize,
    pub description: Option<String>,
    pub git_branch: Option<String>,
    pub git_commit: Option<String>,
}

/// Backup entry combining metadata and file path
#[derive(Debug, Clone)]
pub struct BackupEntry {
    pub metadata: BackupMetadata,
    pub file_path: PathBuf,
}

#[derive(Subcommand)]
pub enum BackupCommands {
    /// Create backup
    Create {
        /// Backup name
        #[arg(long)]
        name: Option<String>,
        /// Auto-generate name
        #[arg(long)]
        auto_name: bool,
    },
    /// List available backups
    List,
    /// Restore from backup
    Restore {
        /// Backup name or file
        backup: String,
        /// Preview restore without applying
        #[arg(long)]
        preview: bool,
        /// Restore specific server only
        #[arg(long)]
        server: Option<String>,
    },
    /// Clean old backups
    Clean {
        /// Remove backups older than duration (e.g., 30d, 1w)
        #[arg(long)]
        older_than: Option<String>,
        /// Force cleanup without confirmation
        #[arg(long)]
        force: bool,
    },
}

/// Handle backup command routing
pub async fn handle_backup_command(action: BackupCommands, profile: Option<String>) -> Result<()> {
    match action {
        BackupCommands::Create { name, auto_name } => {
            create_backup_with_options(name, auto_name, profile).await
        }
        BackupCommands::List => handle_backup_list().await,
        BackupCommands::Restore {
            backup,
            preview,
            server,
        } => restore_backup(backup, preview, server, profile).await,
        BackupCommands::Clean { older_than, force } => handle_backup_clean(older_than, force).await,
    }
}

/// Public wrapper for restore functionality
pub async fn restore_backup(
    backup: String,
    preview: bool,
    server: Option<String>,
    profile: Option<String>,
) -> Result<()> {
    handle_backup_restore(backup, preview, server, profile).await
}

/// Create backup with options handling
pub async fn create_backup_with_options(
    name: Option<String>,
    auto_name: bool,
    profile: Option<String>,
) -> Result<()> {
    let config = Config::load(profile.as_deref()).await?;

    let backup_name = if auto_name {
        format!("auto_{}", chrono::Utc::now().format("%Y%m%d_%H%M%S"))
    } else {
        name.unwrap_or_else(|| chrono::Utc::now().format("%Y%m%d_%H%M%S").to_string())
    };

    let backup_path = create_backup(&config, &backup_name).await?;
    println!("✅ Backup created: {}", backup_path.display());

    Ok(())
}

/// List all available backups
async fn handle_backup_list() -> Result<()> {
    let backups = list_backups().await?;

    if backups.is_empty() {
        println!("{}", "No backups found.".yellow());
        return Ok(());
    }

    println!("{}", "Available Backups".cyan().bold());
    println!("{}", "─────────────────".cyan());

    // Sort by creation date, newest first
    let mut sorted_backups = backups;
    sorted_backups.sort_by(|a, b| b.metadata.created_at.cmp(&a.metadata.created_at));

    for backup in sorted_backups {
        let age = format_duration_since(backup.metadata.created_at);
        println!();
        println!("• {}", backup.metadata.name.bold());
        println!(
            "  Created: {} ({})",
            backup.metadata.created_at.format("%Y-%m-%d %H:%M:%S UTC"),
            age.dimmed()
        );
        println!("  Servers: {}", backup.metadata.servers_count);

        if let Some(desc) = &backup.metadata.description {
            println!("  Description: {}", desc.italic());
        }

        if let Some(branch) = &backup.metadata.git_branch {
            println!("  Git branch: {}", branch.green());
        }

        if let Some(commit) = &backup.metadata.git_commit {
            println!("  Git commit: {}", commit.dimmed());
        }

        println!(
            "  File: {}",
            backup.file_path.display().to_string().dimmed()
        );
    }

    Ok(())
}

/// Restore from backup
async fn handle_backup_restore(
    backup_name: String,
    preview: bool,
    server_filter: Option<String>,
    profile: Option<String>,
) -> Result<()> {
    let backup = find_backup(&backup_name)
        .await?
        .ok_or_else(|| anyhow!("Backup '{}' not found", backup_name))?;

    let backup_config = load_backup_config(&backup.file_path).await?;
    let current_config = Config::load(profile.as_deref()).await.unwrap_or_default();

    if preview {
        preview_restore(&current_config, &backup_config, server_filter.as_deref()).await?;
        return Ok(());
    }

    println!(
        "{}",
        format!("Restoring from backup '{}'...", backup.metadata.name).cyan()
    );

    if let Some(server_name) = server_filter {
        restore_single_server(&backup_config, &server_name, profile.as_deref()).await?;
        println!(
            "{}",
            format!("✓ Server '{}' restored successfully", server_name).green()
        );
    } else {
        restore_full_config(&backup_config, profile.as_deref()).await?;
        println!("{}", "✓ Configuration restored successfully".green());
        println!("  Servers restored: {}", backup_config.mcp_servers.len());
    }

    Ok(())
}

/// Clean old backups
async fn handle_backup_clean(older_than: Option<String>, force: bool) -> Result<()> {
    let duration = if let Some(duration_str) = older_than {
        parse_duration(&duration_str)?
    } else {
        Duration::days(30) // Default: 30 days
    };

    let backups = list_backups().await?;
    let cutoff_date = Utc::now() - duration;

    let old_backups: Vec<_> = backups
        .into_iter()
        .filter(|backup| backup.metadata.created_at < cutoff_date)
        .collect();

    if old_backups.is_empty() {
        println!("{}", "No old backups to clean.".green());
        return Ok(());
    }

    println!(
        "{}",
        format!("Found {} old backup(s) to clean:", old_backups.len()).cyan()
    );
    for backup in &old_backups {
        let age = format_duration_since(backup.metadata.created_at);
        println!("  • {} ({})", backup.metadata.name, age.dimmed());
    }

    if !force {
        println!();
        print!("Delete these backups? [y/N]: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if !input.trim().to_lowercase().starts_with('y') {
            println!("Cleanup cancelled.");
            return Ok(());
        }
    }

    let mut deleted_count = 0;
    for backup in old_backups {
        if fs::remove_file(&backup.file_path).is_ok() {
            deleted_count += 1;
            println!("{}", format!("✓ Deleted {}", backup.metadata.name).green());
        } else {
            println!(
                "{}",
                format!("✗ Failed to delete {}", backup.metadata.name).red()
            );
        }
    }

    println!();
    println!(
        "{}",
        format!("Cleanup complete. Deleted {} backup(s).", deleted_count).green()
    );
    Ok(())
}

/// Create a backup with a specific name
pub async fn create_backup(config: &Config, name: &str) -> Result<PathBuf> {
    let backup_dir = utils::get_backup_dir()?;
    fs::create_dir_all(&backup_dir)?;

    let backup_file = backup_dir.join(format!("{}.json", sanitize_filename(name)));

    // Create metadata
    let metadata = BackupMetadata {
        name: name.to_string(),
        created_at: Utc::now(),
        servers_count: config.mcp_servers.len(),
        description: None,
        git_branch: get_git_branch().await,
        git_commit: get_git_commit().await,
    };

    // Create backup structure
    let backup_data = serde_json::json!({
        "metadata": metadata,
        "config": config
    });

    // Write backup file
    fs::write(&backup_file, serde_json::to_string_pretty(&backup_data)?)?;

    Ok(backup_file)
}

/// List all available backups
async fn list_backups() -> Result<Vec<BackupEntry>> {
    let backup_dir = utils::get_backup_dir()?;

    if !backup_dir.exists() {
        return Ok(Vec::new());
    }

    let mut backups = Vec::new();

    for entry in fs::read_dir(backup_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            if let Ok(backup_data) = load_backup_data(&path).await {
                backups.push(BackupEntry {
                    metadata: backup_data.metadata,
                    file_path: path,
                });
            }
        }
    }

    Ok(backups)
}

/// Find a backup by name or partial name
async fn find_backup(name: &str) -> Result<Option<BackupEntry>> {
    let backups = list_backups().await?;

    // First try exact match
    for backup in &backups {
        if backup.metadata.name == name {
            return Ok(Some(backup.clone()));
        }
    }

    // Then try partial match
    for backup in &backups {
        if backup.metadata.name.contains(name) {
            return Ok(Some(backup.clone()));
        }
    }

    Ok(None)
}

/// Load backup configuration
async fn load_backup_config(backup_path: &Path) -> Result<Config> {
    let backup_data = load_backup_data(backup_path).await?;
    Ok(backup_data.config)
}

/// Preview what would be restored
async fn preview_restore(
    current: &Config,
    backup: &Config,
    server_filter: Option<&str>,
) -> Result<()> {
    println!("{}", "Restore Preview".cyan().bold());
    println!("{}", "──────────────".cyan());

    let servers_to_restore = if let Some(filter) = server_filter {
        backup
            .mcp_servers
            .iter()
            .filter(|(name, _)| name == &filter)
            .collect::<HashMap<_, _>>()
    } else {
        backup.mcp_servers.iter().collect()
    };

    if servers_to_restore.is_empty() {
        println!("{}", "No servers to restore.".yellow());
        return Ok(());
    }

    println!("Servers to be restored:");
    for (name, server) in &servers_to_restore {
        let status = if current.mcp_servers.contains_key(*name) {
            "OVERWRITE".yellow()
        } else {
            "NEW".green()
        };

        println!("  {} {} - {}", status, name.bold(), server.command);
    }

    if server_filter.is_none() {
        let current_only: Vec<_> = current
            .mcp_servers
            .keys()
            .filter(|name| !backup.mcp_servers.contains_key(*name))
            .collect();

        if !current_only.is_empty() {
            println!();
            println!("Servers that will remain unchanged:");
            for name in current_only {
                println!("  {} {}", "KEEP".blue(), name.bold());
            }
        }
    }

    Ok(())
}

/// Restore a single server
async fn restore_single_server(
    backup_config: &Config,
    server_name: &str,
    profile: Option<&str>,
) -> Result<()> {
    let server = backup_config
        .mcp_servers
        .get(server_name)
        .ok_or_else(|| anyhow!("Server '{}' not found in backup", server_name))?;

    let mut current_config = Config::load(profile).await.unwrap_or_default();
    current_config
        .mcp_servers
        .insert(server_name.to_string(), server.clone());

    current_config.save(profile).await?;

    // Update profile metadata
    update_profile_server_count(profile).await?;

    Ok(())
}

/// Restore full configuration
async fn restore_full_config(backup_config: &Config, profile: Option<&str>) -> Result<()> {
    backup_config.save(profile).await?;

    // Update profile metadata
    update_profile_server_count(profile).await?;

    Ok(())
}

/// Get current git branch if available
async fn get_git_branch() -> Option<String> {
    tokio::process::Command::new("git")
        .args(["branch", "--show-current"])
        .output()
        .await
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout)
                    .ok()
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
            } else {
                None
            }
        })
}

/// Get current git commit if available
async fn get_git_commit() -> Option<String> {
    tokio::process::Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .await
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout)
                    .ok()
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
            } else {
                None
            }
        })
}

/// Backup data structure
#[derive(Debug, Serialize, Deserialize)]
struct BackupData {
    metadata: BackupMetadata,
    config: Config,
}

/// Load backup data from file
async fn load_backup_data(path: &Path) -> Result<BackupData> {
    let content = fs::read_to_string(path)?;
    let backup_data: BackupData = serde_json::from_str(&content)?;
    Ok(backup_data)
}

/// Parse duration string (e.g., "30d", "1w", "24h")
fn parse_duration(duration_str: &str) -> Result<Duration> {
    let duration_str = duration_str.trim().to_lowercase();

    if let Some(num_str) = duration_str.strip_suffix('d') {
        let days: i64 = num_str.parse()?;
        Ok(Duration::days(days))
    } else if let Some(num_str) = duration_str.strip_suffix('w') {
        let weeks: i64 = num_str.parse()?;
        Ok(Duration::weeks(weeks))
    } else if let Some(num_str) = duration_str.strip_suffix('h') {
        let hours: i64 = num_str.parse()?;
        Ok(Duration::hours(hours))
    } else if let Some(num_str) = duration_str.strip_suffix('m') {
        let minutes: i64 = num_str.parse()?;
        Ok(Duration::minutes(minutes))
    } else {
        // Try parsing as days
        let days: i64 = duration_str
            .parse()
            .map_err(|_| anyhow!("Invalid duration format. Use format like '30d', '1w', '24h'"))?;
        Ok(Duration::days(days))
    }
}

/// Format duration since a timestamp
fn format_duration_since(timestamp: DateTime<Utc>) -> String {
    let duration = Utc::now().signed_duration_since(timestamp);

    if duration.num_days() > 0 {
        format!("{} day(s) ago", duration.num_days())
    } else if duration.num_hours() > 0 {
        format!("{} hour(s) ago", duration.num_hours())
    } else if duration.num_minutes() > 0 {
        format!("{} minute(s) ago", duration.num_minutes())
    } else {
        "Just now".to_string()
    }
}

/// Sanitize filename by removing invalid characters
fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c => c,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration() {
        assert_eq!(parse_duration("30d").unwrap(), Duration::days(30));
        assert_eq!(parse_duration("2w").unwrap(), Duration::weeks(2));
        assert_eq!(parse_duration("24h").unwrap(), Duration::hours(24));
        assert_eq!(parse_duration("60m").unwrap(), Duration::minutes(60));
        assert_eq!(parse_duration("7").unwrap(), Duration::days(7));
    }

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("feature/new-stuff"), "feature_new-stuff");
        assert_eq!(sanitize_filename("backup:2024"), "backup_2024");
        assert_eq!(sanitize_filename("normal-name"), "normal-name");
    }

    #[test]
    fn test_backup_metadata() {
        let metadata = BackupMetadata {
            name: "test".to_string(),
            created_at: Utc::now(),
            servers_count: 5,
            description: Some("Test backup".to_string()),
            git_branch: Some("main".to_string()),
            git_commit: Some("abcd123".to_string()),
        };

        assert_eq!(metadata.name, "test");
        assert_eq!(metadata.servers_count, 5);
    }
}
