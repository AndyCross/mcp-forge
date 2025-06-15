use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use crate::utils;

/// Represents an individual MCP server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServer {
    pub command: String,
    pub args: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

/// Represents the Claude Desktop configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "mcpServers")]
    pub mcpServers: HashMap<String, McpServer>,
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mcpServers: HashMap::new(),
            other: HashMap::new(),
        }
    }
}

impl Config {
    /// Load configuration from file (with optional profile support)
    pub async fn load(profile: Option<&str>) -> Result<Self> {
        let config_path = if let Some(profile_name) = profile {
            // Load from profile-specific file
            let config_dir = utils::get_config_dir()?;
            config_dir.join(format!("profile_{}.json", profile_name))
        } else {
            // Load from Claude Desktop config
            utils::get_claude_config_path()?
        };

        if !config_path.exists() {
            return Ok(Config::default());
        }

        let content = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;

        serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", config_path.display()))
    }

    /// Save configuration to file (with optional profile support)
    pub async fn save(&self, profile: Option<&str>) -> Result<()> {
        let config_path = if let Some(profile_name) = profile {
            // Save to profile-specific file
            let config_dir = utils::get_config_dir()?;
            fs::create_dir_all(&config_dir)?;
            config_dir.join(format!("profile_{}.json", profile_name))
        } else {
            // Save to Claude Desktop config
            utils::get_claude_config_path()?
        };

        // Create parent directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory: {}", parent.display()))?;
        }

        let content = serde_json::to_string_pretty(self)
            .context("Failed to serialize config")?;

        fs::write(&config_path, content)
            .with_context(|| format!("Failed to write config file: {}", config_path.display()))?;

        // Update profile server count if using profiles
        if profile.is_some() {
            let _ = crate::profiles::update_profile_server_count(profile).await;
        }

        Ok(())
    }

    /// Create a backup of the current configuration
    pub async fn create_backup(&self) -> Result<PathBuf> {
        let config_path = utils::get_claude_config_path()?;
        
        if !config_path.exists() {
            // Create default config first
            self.save(None).await?;
        }

        let backup_dir = utils::get_backup_dir()?;
        fs::create_dir_all(&backup_dir)?;

        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let backup_path = backup_dir.join(format!("backup_{}.json", timestamp));

        fs::copy(&config_path, &backup_path)
            .with_context(|| format!("Failed to create backup: {}", backup_path.display()))?;

        Ok(backup_path)
    }

    /// Restore configuration from backup
    pub async fn restore_from_backup(backup_path: &PathBuf, profile: Option<&str>) -> Result<()> {
        if !backup_path.exists() {
            return Err(anyhow!("Backup file does not exist: {}", backup_path.display()));
        }

        let content = fs::read_to_string(backup_path)
            .with_context(|| format!("Failed to read backup file: {}", backup_path.display()))?;

        let config: Config = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse backup file: {}", backup_path.display()))?;

        config.save(profile).await?;
        Ok(())
    }

    /// Add or update an MCP server
    pub fn add_server(&mut self, name: String, server: McpServer) {
        self.mcpServers.insert(name, server);
    }

    /// Remove an MCP server
    pub fn remove_server(&mut self, name: &str) -> bool {
        self.mcpServers.remove(name).is_some()
    }

    /// Get a specific MCP server
    pub fn get_server(&self, name: &str) -> Option<&McpServer> {
        self.mcpServers.get(name)
    }

    /// Get a mutable reference to a specific MCP server
    pub fn get_server_mut(&mut self, name: &str) -> Option<&mut McpServer> {
        self.mcpServers.get_mut(name)
    }

    /// List all MCP servers
    pub fn list_servers(&self) -> Vec<(String, &McpServer)> {
        self.mcpServers.iter().map(|(k, v)| (k.clone(), v)).collect()
    }

    /// Check if a server exists
    pub fn has_server(&self, name: &str) -> bool {
        self.mcpServers.contains_key(name)
    }

    /// Get the number of configured servers
    pub fn server_count(&self) -> usize {
        self.mcpServers.len()
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        for (name, server) in &self.mcpServers {
            if name.is_empty() {
                return Err(anyhow!("Server name cannot be empty"));
            }
            
            if server.command.is_empty() {
                return Err(anyhow!("Server '{}' has empty command", name));
            }
        }
        Ok(())
    }
}

/// Legacy type alias for backward compatibility
pub type ClaudeConfig = Config;

/// Configuration manager for backward compatibility
pub struct ConfigManager {
    config: Config,
    profile: Option<String>,
}

impl ConfigManager {
    /// Create a new ConfigManager
    pub fn new() -> Result<Self> {
        Ok(Self {
            config: Config::default(),
            profile: None,
        })
    }

    /// Create a new ConfigManager with a specific profile
    pub fn with_profile(profile: Option<String>) -> Result<Self> {
        Ok(Self {
            config: Config::default(),
            profile,
        })
    }

    /// Get the configuration path
    pub fn config_path(&self) -> PathBuf {
        if let Some(profile_name) = &self.profile {
            let config_dir = utils::get_config_dir().unwrap_or_else(|_| PathBuf::from("~/.config/mcp-forge"));
            config_dir.join(format!("profile_{}.json", profile_name))
        } else {
            utils::get_claude_config_path().unwrap_or_else(|_| PathBuf::from("~/.config/claude/claude_desktop_config.json"))
        }
    }

    /// Load configuration
    pub async fn load_config(&mut self) -> Result<()> {
        self.config = Config::load(self.profile.as_deref()).await?;
        Ok(())
    }

    /// Save configuration
    pub async fn save_config(&self) -> Result<()> {
        self.config.save(self.profile.as_deref()).await
    }

    /// List all servers
    pub fn list_servers(&self) -> Result<Vec<(String, McpServer)>> {
        Ok(self.config.list_servers().into_iter().map(|(k, v)| (k, v.clone())).collect())
    }

    /// Get a server
    pub fn get_server(&self, name: &str) -> Result<Option<McpServer>> {
        Ok(self.config.get_server(name).cloned())
    }

    /// Add a server
    pub fn add_server(&mut self, name: String, server: McpServer) -> Result<()> {
        self.config.add_server(name, server);
        Ok(())
    }

    /// Remove a server
    pub fn remove_server(&mut self, name: &str) -> Result<bool> {
        Ok(self.config.remove_server(name))
    }

    /// Create a backup
    pub async fn create_backup(&self) -> Result<PathBuf> {
        self.config.create_backup().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_serialization() {
        let mut config = Config::default();
        config.mcpServers.insert(
            "test-server".to_string(),
            McpServer {
                command: "node".to_string(),
                args: vec!["server.js".to_string()],
                env: None,
                other: HashMap::new(),
            },
        );

        let json = serde_json::to_string_pretty(&config).unwrap();
        let parsed: Config = serde_json::from_str(&json).unwrap();

        assert_eq!(config.mcpServers.len(), parsed.mcpServers.len());
        assert!(parsed.mcpServers.contains_key("test-server"));
    }

    #[test]
    fn test_config_operations() {
        let mut config = Config::default();
        
        let server = McpServer {
            command: "test".to_string(),
            args: vec!["arg1".to_string()],
            env: None,
            other: HashMap::new(),
        };

        // Test add
        config.add_server("test1".to_string(), server.clone());
        assert_eq!(config.server_count(), 1);
        assert!(config.has_server("test1"));

        // Test get
        assert!(config.get_server("test1").is_some());
        assert!(config.get_server("nonexistent").is_none());

        // Test remove
        assert!(config.remove_server("test1"));
        assert!(!config.remove_server("nonexistent"));
        assert_eq!(config.server_count(), 0);
    }

    #[test]
    fn test_config_validation() {
        let mut config = Config::default();
        
        // Valid config should pass
        config.add_server("valid".to_string(), McpServer {
            command: "test".to_string(),
            args: vec![],
            env: None,
            other: HashMap::new(),
        });
        assert!(config.validate().is_ok());

        // Invalid config with empty command should fail
        config.add_server("invalid".to_string(), McpServer {
            command: "".to_string(),
            args: vec![],
            env: None,
            other: HashMap::new(),
        });
        assert!(config.validate().is_err());
    }
} 