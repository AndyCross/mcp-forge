use anyhow::{Context, Result};
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
    pub mcp_servers: HashMap<String, McpServer>,
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mcp_servers: HashMap::new(),
            other: HashMap::new(),
        }
    }
}

impl Config {
    /// Load configuration from file
    pub async fn load(profile: Option<&str>) -> Result<Self> {
        let config_path = if let Some(profile_name) = profile {
            utils::get_profile_config_path(profile_name)?
        } else {
            utils::get_claude_config_path()?
        };

        if !config_path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;

        let config: Self = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", config_path.display()))?;

        Ok(config)
    }

    /// Save configuration to file
    pub async fn save(&self, profile: Option<&str>) -> Result<()> {
        let config_path = if let Some(profile_name) = profile {
            utils::get_profile_config_path(profile_name)?
        } else {
            utils::get_claude_config_path()?
        };

        // Ensure parent directory exists
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory: {}", parent.display()))?;
        }

        let content = serde_json::to_string_pretty(self)
            .context("Failed to serialize configuration")?;

        fs::write(&config_path, content)
            .with_context(|| format!("Failed to write config file: {}", config_path.display()))?;

        Ok(())
    }

    /// Create a backup of the current configuration
    pub async fn create_backup(&self) -> Result<PathBuf> {
        let backup_dir = utils::get_backup_dir()?;
        fs::create_dir_all(&backup_dir)?;

        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let backup_path = backup_dir.join(format!("config_backup_{}.json", timestamp));

        let content = serde_json::to_string_pretty(self)?;
        fs::write(&backup_path, content)?;

        Ok(backup_path)
    }

    /// Add or update an MCP server
    pub fn add_server(&mut self, name: String, server: McpServer) {
        self.mcp_servers.insert(name, server);
    }

    /// Remove an MCP server
    pub fn remove_server(&mut self, name: &str) -> bool {
        self.mcp_servers.remove(name).is_some()
    }

    /// Get a specific MCP server
    pub fn get_server(&self, name: &str) -> Option<&McpServer> {
        self.mcp_servers.get(name)
    }

    /// List all MCP servers
    pub fn list_servers(&self) -> Vec<(String, &McpServer)> {
        self.mcp_servers.iter().map(|(k, v)| (k.clone(), v)).collect()
    }
}

/// Configuration manager for handling MCP server configurations
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

    /// Load configuration from file
    pub async fn load_config(&mut self) -> Result<()> {
        self.config = Config::load(self.profile.as_deref()).await?;
        Ok(())
    }

    /// List all servers
    pub fn list_servers(&self) -> Result<Vec<(String, McpServer)>> {
        Ok(self.config.mcp_servers.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
    }

    /// Get a specific server
    pub fn get_server(&self, name: &str) -> Result<Option<McpServer>> {
        Ok(self.config.mcp_servers.get(name).cloned())
    }

    /// Add a server
    pub async fn add_server(&mut self, name: String, server: McpServer) -> Result<()> {
        self.config.add_server(name, server);
        self.config.save(self.profile.as_deref()).await?;
        Ok(())
    }

    /// Remove a server
    pub async fn remove_server(&mut self, name: &str) -> Result<bool> {
        let removed = self.config.remove_server(name);
        self.config.save(self.profile.as_deref()).await?;
        Ok(removed)
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
        config.mcp_servers.insert(
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

        assert_eq!(config.mcp_servers.len(), parsed.mcp_servers.len());
        assert!(parsed.mcp_servers.contains_key("test-server"));
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
        assert_eq!(config.list_servers().len(), 1);
        assert!(config.get_server("test1").is_some());

        // Test remove
        assert!(config.remove_server("test1"));
        assert!(!config.remove_server("nonexistent"));
        assert_eq!(config.list_servers().len(), 0);
    }
} 