use crate::utils;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;

/// Represents an MCP server configuration
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(rename = "mcpServers")]
    pub mcp_servers: HashMap<String, McpServer>,
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
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
            .await
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
            fs::create_dir_all(parent).await.with_context(|| {
                format!("Failed to create config directory: {}", parent.display())
            })?;
        }

        let content =
            serde_json::to_string_pretty(self).context("Failed to serialize configuration")?;

        fs::write(&config_path, content)
            .await
            .with_context(|| format!("Failed to write config file: {}", config_path.display()))?;

        Ok(())
    }

    /// Create a backup of the current configuration
    pub async fn create_backup(&self) -> Result<PathBuf> {
        let backup_dir = utils::get_backup_dir()?;
        fs::create_dir_all(&backup_dir).await?;

        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let backup_path = backup_dir.join(format!("config_backup_{}.json", timestamp));

        let content = serde_json::to_string_pretty(self)?;
        fs::write(&backup_path, content).await?;

        Ok(backup_path)
    }

    /// Get a specific MCP server
    pub fn get_server(&self, name: &str) -> Option<&McpServer> {
        self.mcp_servers.get(name)
    }

    /// List all MCP servers
    pub fn list_servers(&self) -> Vec<(String, &McpServer)> {
        self.mcp_servers
            .iter()
            .map(|(k, v)| (k.clone(), v))
            .collect()
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
        let config = Config::default();

        // Test that we can create and serialize configs
        assert_eq!(config.mcp_servers.len(), 0);

        let json = serde_json::to_string_pretty(&config).unwrap();
        let parsed: Config = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.mcp_servers.len(), 0);
    }
}
