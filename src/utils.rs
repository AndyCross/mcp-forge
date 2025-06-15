use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};

/// Utility functions for MCP-Forge

/// Expand tilde (~) in file paths to the user's home directory
pub fn expand_path(path: &str) -> Result<String> {
    if path.starts_with("~/") {
        if let Some(home_dir) = dirs::home_dir() {
            let expanded = home_dir.join(&path[2..]);
            Ok(expanded.to_string_lossy().to_string())
        } else {
            anyhow::bail!("Unable to determine home directory");
        }
    } else {
        Ok(path.to_string())
    }
}

/// Check if a file exists and is readable
pub fn is_file_accessible(path: &Path) -> bool {
    path.exists() && path.is_file()
}

/// Format file size in human-readable format
pub fn format_file_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

/// Validate server name (alphanumeric, hyphens, underscores only)
pub fn validate_server_name(name: &str) -> Result<()> {
    if name.is_empty() {
        anyhow::bail!("Server name cannot be empty");
    }

    if name.len() > 64 {
        anyhow::bail!("Server name cannot be longer than 64 characters");
    }

    for char in name.chars() {
        if !char.is_alphanumeric() && char != '-' && char != '_' {
            anyhow::bail!("Server name can only contain alphanumeric characters, hyphens, and underscores");
        }
    }

    Ok(())
}

/// Get the current timestamp as a string
pub fn get_timestamp() -> String {
    chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string()
}

/// Truncate string to a maximum length, adding ellipsis if necessary
pub fn truncate_string(s: &str, max_length: usize) -> String {
    if s.len() <= max_length {
        s.to_string()
    } else {
        format!("{}...", &s[..max_length.saturating_sub(3)])
    }
}

/// Pretty print JSON with consistent formatting
pub fn pretty_print_json(value: &serde_json::Value) -> Result<String> {
    serde_json::to_string_pretty(value)
        .map_err(|e| anyhow::anyhow!("Failed to format JSON: {}", e))
}

/// Get the Claude Desktop configuration file path
pub fn get_claude_config_path() -> Result<PathBuf> {
    #[cfg(target_os = "macos")]
    {
        if let Some(home_dir) = dirs::home_dir() {
            Ok(home_dir.join("Library/Application Support/Claude/claude_desktop_config.json"))
        } else {
            Err(anyhow!("Unable to determine home directory"))
        }
    }
    #[cfg(target_os = "windows")]
    {
        if let Some(config_dir) = dirs::config_dir() {
            Ok(config_dir.join("Claude/claude_desktop_config.json"))
        } else {
            Err(anyhow!("Unable to determine config directory"))
        }
    }
    #[cfg(target_os = "linux")]
    {
        if let Some(config_dir) = dirs::config_dir() {
            Ok(config_dir.join("Claude/claude_desktop_config.json"))
        } else {
            Err(anyhow!("Unable to determine config directory"))
        }
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        Err(anyhow!("Unsupported operating system"))
    }
}

/// Get the backup directory
pub fn get_backup_dir() -> Result<PathBuf> {
    let config_dir = get_config_dir()?;
    Ok(config_dir.join("backups"))
}

/// Get the mcp-forge config directory
pub fn get_config_dir() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow!("Could not determine config directory"))?;
    Ok(config_dir.join("mcp-forge"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_server_name() {
        assert!(validate_server_name("valid-name").is_ok());
        assert!(validate_server_name("valid_name").is_ok());
        assert!(validate_server_name("validname123").is_ok());
        
        assert!(validate_server_name("").is_err());
        assert!(validate_server_name("invalid name").is_err());
        assert!(validate_server_name("invalid@name").is_err());
        assert!(validate_server_name(&"a".repeat(65)).is_err());
    }

    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(0), "0 B");
        assert_eq!(format_file_size(1), "1 B");
        assert_eq!(format_file_size(1024), "1.0 KB");
        assert_eq!(format_file_size(1536), "1.5 KB");
        assert_eq!(format_file_size(1048576), "1.0 MB");
    }

    #[test]
    fn test_truncate_string() {
        assert_eq!(truncate_string("short", 10), "short");
        assert_eq!(truncate_string("this is a long string", 10), "this is...");
        assert_eq!(truncate_string("exact", 5), "exact");
    }
} 