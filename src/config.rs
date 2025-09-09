use crate::utils;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;

/// Represents an MCP server configuration
/// Supports both command-based and URL-based servers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
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
    /// Always loads from the main Claude Desktop configuration file
    pub async fn load(_profile: Option<&str>) -> Result<Self> {
        // Always load from the main Claude Desktop config file
        // The profile parameter is ignored - profiles are managed separately
        let config_path = utils::get_claude_config_path()?;

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
    /// Always saves to the main Claude Desktop configuration file
    pub async fn save(&self, _profile: Option<&str>) -> Result<()> {
        // Always save to the main Claude Desktop config file
        // The profile parameter is ignored - profiles are managed separately
        let config_path = utils::get_claude_config_path()?;

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

impl McpServer {
    /// Check if this is a URL-type server
    pub fn is_url_server(&self) -> bool {
        self.url.is_some()
    }

    /// Check if this is a command-type server
    #[allow(dead_code)] // Used in tests and for future API consistency
    pub fn is_command_server(&self) -> bool {
        self.command.is_some()
    }

    /// Validate the server configuration
    #[allow(dead_code)] // May be used in future validation workflows
    pub fn validate(&self) -> Result<()> {
        // A server must have either a URL or a command, but not both
        match (self.url.as_ref(), self.command.as_ref()) {
            (Some(_), Some(_)) => {
                anyhow::bail!("Server cannot have both 'url' and 'command' fields")
            }
            (None, None) => {
                anyhow::bail!("Server must have either 'url' or 'command' field")
            }
            (Some(_), None) => {
                // URL server - valid
                Ok(())
            }
            (None, Some(_)) => {
                // Command server - args can be empty but should be present for command servers
                Ok(())
            }
        }
    }

    /// Get a display string for the server type
    #[allow(dead_code)] // Used in tests and for future display features
    pub fn server_type(&self) -> &str {
        if self.is_url_server() {
            "url"
        } else {
            "command"
        }
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
                command: Some("node".to_string()),
                args: Some(vec!["server.js".to_string()]),
                url: None,
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
    fn test_url_server() {
        let mut config = Config::default();
        config.mcp_servers.insert(
            "url-server".to_string(),
            McpServer {
                command: None,
                args: None,
                url: Some("https://example.com/mcp".to_string()),
                env: None,
                other: HashMap::new(),
            },
        );

        let json = serde_json::to_string_pretty(&config).unwrap();
        let parsed: Config = serde_json::from_str(&json).unwrap();

        assert_eq!(config.mcp_servers.len(), parsed.mcp_servers.len());
        assert!(parsed.mcp_servers.contains_key("url-server"));
        
        let server = parsed.mcp_servers.get("url-server").unwrap();
        assert!(server.is_url_server());
        assert!(!server.is_command_server());
        assert_eq!(server.server_type(), "url");
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
