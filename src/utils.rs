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

/// Get the backup directory
pub fn get_backup_dir() -> Result<PathBuf> {
    let config_dir = get_config_dir()?;
    Ok(config_dir.join("backups"))
}

/// Mask sensitive parts of URLs to prevent credential leaks
///
/// This function looks for API keys and tokens in URL query parameters
/// and masks them to prevent accidental exposure.
pub fn mask_sensitive_url(url: &str) -> String {
    // Parse the URL to find query parameters
    if let Ok(parsed_url) = url::Url::parse(url) {
        let mut masked_url = format!("{}://{}", parsed_url.scheme(), parsed_url.host_str().unwrap_or(""));
        
        if let Some(port) = parsed_url.port() {
            masked_url.push_str(&format!(":{}", port));
        }
        
        masked_url.push_str(parsed_url.path());
        
        // Check for query parameters
        if let Some(query) = parsed_url.query() {
            let mut masked_params = Vec::new();
            for pair in query.split('&') {
                if let Some((key, value)) = pair.split_once('=') {
                    let normalized_key = key.to_lowercase();
                    // Check if this is a sensitive parameter
                    let is_sensitive = normalized_key.contains("key") 
                        || normalized_key.contains("token")
                        || normalized_key.contains("secret")
                        || normalized_key.contains("password")
                        || normalized_key.contains("apikey")
                        || normalized_key.contains("api_key");
                    
                    if is_sensitive && value.len() > 6 {
                        // Mask the value
                        let first_part = &value[..3];
                        let last_part = &value[value.len() - 3..];
                        let masked_value = format!("{}***{}", first_part, last_part);
                        masked_params.push(format!("{}={}", key, masked_value));
                    } else if is_sensitive {
                        masked_params.push(format!("{}=***", key));
                    } else {
                        masked_params.push(format!("{}={}", key, value));
                    }
                } else {
                    masked_params.push(pair.to_string());
                }
            }
            if !masked_params.is_empty() {
                masked_url.push('?');
                masked_url.push_str(&masked_params.join("&"));
            }
        }
        
        if let Some(fragment) = parsed_url.fragment() {
            masked_url.push('#');
            masked_url.push_str(fragment);
        }
        
        masked_url
    } else {
        // If URL parsing fails, return as-is (might not be a valid URL)
        url.to_string()
    }
}

/// Mask sensitive environment variable values to prevent credential leaks
///
/// This function checks if an environment variable key contains sensitive patterns
/// like CLIENT_ID, CLIENT_SECRET, etc. (case insensitive, with various separators)
/// and masks the value showing only first 3 and last 3 characters.
pub fn mask_sensitive_env_value(key: &str, value: &str) -> String {
    // Convert key to lowercase and normalize separators for pattern matching
    let normalized_key = key.to_lowercase().replace(['_', '-', '.'], "");

    // List of sensitive patterns to look for
    let sensitive_patterns = [
        "clientid",
        "clientsecret",
        "apikey",
        "accesstoken",
        "secretkey",
        "privatekey",
        "password",
        "passwd",
        "token",
        "secret",
        "key",
    ];

    // Check if the key contains any sensitive patterns
    let is_sensitive = sensitive_patterns
        .iter()
        .any(|pattern| normalized_key.contains(pattern));

    if is_sensitive && value.len() > 6 {
        // Show first 3 and last 3 characters with asterisks in between
        let first_part = &value[..3];
        let last_part = &value[value.len() - 3..];
        let middle_length = value.len() - 6;
        let asterisks = "*".repeat(middle_length.max(4)); // At least 4 asterisks
        format!("{}{}{}", first_part, asterisks, last_part)
    } else if is_sensitive {
        // For very short values, just show asterisks
        "*".repeat(value.len().max(8))
    } else {
        // Not sensitive, return as-is
        value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_paths() {
        // Test that we can get config paths without errors
        assert!(get_config_dir().is_ok());
        assert!(get_claude_config_path().is_ok());
        assert!(get_backup_dir().is_ok());
    }

    #[test]
    fn test_mask_sensitive_env_value() {
        // Test CLIENT_ID masking (22 chars: 3 + 16 + 3)
        assert_eq!(
            mask_sensitive_env_value("CLIENT_ID", "LgAqzbS6oL-60HwSULGzrA"),
            "LgA****************zrA"
        );

        // Test CLIENT_SECRET masking (30 chars: 3 + 24 + 3)
        assert_eq!(
            mask_sensitive_env_value("CLIENT_SECRET", "KJCYTuWHOKRIaE0qx_SfimX1j_PHag"),
            "KJC************************Hag"
        );

        // Test different case and separators (12 chars: 3 + 6 + 3)
        assert_eq!(
            mask_sensitive_env_value("client-id", "abc123def456"),
            "abc******456"
        );

        // Test API.KEY (14 chars: 3 + 8 + 3)
        assert_eq!(
            mask_sensitive_env_value("API.KEY", "secretvalue123"),
            "sec********123"
        );

        // Test non-sensitive values (should not be masked)
        assert_eq!(
            mask_sensitive_env_value("DATABASE_HOST", "localhost"),
            "localhost"
        );

        assert_eq!(mask_sensitive_env_value("PORT", "8080"), "8080");

        // Test short sensitive values
        assert_eq!(mask_sensitive_env_value("SECRET", "abc"), "********");

        // Test API_KEY pattern (13 chars: 3 + 7 + 3)
        assert_eq!(
            mask_sensitive_env_value("REDDIT_API_KEY", "test123456789"),
            "tes*******789"
        );
    }
}
