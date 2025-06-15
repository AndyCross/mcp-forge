use anyhow::{Context, Result};
use std::path::PathBuf;

/// Utility functions for MCP-Forge

/// Get the Claude Desktop configuration directory
pub fn get_config_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not find home directory")?;
    
    #[cfg(target_os = "macos")]
    let config_dir = home.join("Library/Application Support/Claude");
    
    #[cfg(target_os = "windows")]
    let config_dir = home.join("AppData/Roaming/Claude");
    
    #[cfg(target_os = "linux")]
    let config_dir = home.join(".config/claude");
    
    Ok(config_dir)
}

/// Get the Claude Desktop configuration file path
pub fn get_claude_config_path() -> Result<PathBuf> {
    let config_dir = get_config_dir()?;
    Ok(config_dir.join("claude_desktop_config.json"))
}

/// Get the MCP Forge cache directory
pub fn get_cache_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not find home directory")?;
    
    #[cfg(target_os = "macos")]
    let cache_dir = home.join("Library/Caches/mcp-forge");
    
    #[cfg(target_os = "windows")]
    let cache_dir = home.join("AppData/Local/mcp-forge/cache");
    
    #[cfg(target_os = "linux")]
    let cache_dir = home.join(".cache/mcp-forge");
    
    Ok(cache_dir)
}

/// Get the backup directory
pub fn get_backup_dir() -> Result<PathBuf> {
    let config_dir = get_config_dir()?;
    Ok(config_dir.join("backups"))
}

/// Get profile-specific configuration path
pub fn get_profile_config_path(profile_name: &str) -> Result<PathBuf> {
    let config_dir = get_config_dir()?;
    Ok(config_dir.join(format!("profile_{}.json", profile_name)))
}

/// Get profiles directory
pub fn get_profiles_dir() -> Result<PathBuf> {
    let config_dir = get_config_dir()?;
    Ok(config_dir.join("profiles"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_paths() {
        // Test that we can get config paths without errors
        assert!(get_config_dir().is_ok());
        assert!(get_claude_config_path().is_ok());
        assert!(get_cache_dir().is_ok());
        assert!(get_backup_dir().is_ok());
        assert!(get_profile_config_path("test").is_ok());
        assert!(get_profiles_dir().is_ok());
    }
} 