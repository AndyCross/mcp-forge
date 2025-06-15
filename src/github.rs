use crate::templates::{Template, TemplateCatalog};
use anyhow::{anyhow, Context, Result};
use base64::{self, Engine};
use serde::Deserialize;

#[cfg(test)]
use std::collections::HashMap;

/// Configuration for the template repository
#[derive(Debug, Clone)]
pub struct TemplateRepository {
    pub owner: String,
    pub repo: String,
    pub branch: String,
}

impl Default for TemplateRepository {
    fn default() -> Self {
        Self {
            owner: "AndyCross".to_string(),
            repo: "mcp-forge-templates".to_string(),
            branch: "master".to_string(),
        }
    }
}

/// GitHub API response for repository files
#[derive(Deserialize)]
struct GitHubFileResponse {
    content: String,
    encoding: String,
}

/// GitHub client for fetching MCP server templates
pub struct GitHubClient {
    client: reqwest::Client,
    repo: TemplateRepository,
    base_url: String,
}

impl GitHubClient {
    /// Create a new GitHub client
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            repo: TemplateRepository::default(),
            base_url: "https://api.github.com".to_string(),
        }
    }

    /// Fetch the template catalog from GitHub
    pub async fn fetch_template_catalog(&self) -> Result<TemplateCatalog> {
        let url = format!(
            "{}/repos/{}/{}/contents/catalog.json?ref={}",
            self.base_url, self.repo.owner, self.repo.repo, self.repo.branch
        );

        let response = self
            .client
            .get(&url)
            .header("User-Agent", "mcp-forge")
            .send()
            .await
            .context("Failed to fetch template catalog from GitHub")?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "GitHub API request failed with status: {} - {}",
                response.status(),
                Self::create_github_error_message(&anyhow!("API request failed"))
            ));
        }

        let github_response: GitHubFileResponse = response
            .json()
            .await
            .context("Failed to parse GitHub API response")?;

        let content = if github_response.encoding == "base64" {
            let decoded = base64::engine::general_purpose::STANDARD
                .decode(github_response.content.replace('\n', ""))
                .context("Failed to decode base64 content")?;
            String::from_utf8(decoded).context("Invalid UTF-8 in decoded content")?
        } else {
            github_response.content
        };

        let catalog: TemplateCatalog =
            serde_json::from_str(&content).context("Failed to parse template catalog JSON")?;

        Ok(catalog)
    }

    /// Fetch a specific template from GitHub
    pub async fn fetch_template(&self, template_name: &str) -> Result<Template> {
        // First fetch the catalog to get the template path
        let catalog = self.fetch_template_catalog().await?;

        let template_metadata = catalog
            .templates
            .get(template_name)
            .ok_or_else(|| anyhow!("Template '{}' not found in catalog", template_name))?;

        let url = format!(
            "{}/repos/{}/{}/contents/{}?ref={}",
            self.base_url,
            self.repo.owner,
            self.repo.repo,
            template_metadata.path,
            self.repo.branch
        );

        let response = self
            .client
            .get(&url)
            .header("User-Agent", "mcp-forge")
            .send()
            .await
            .with_context(|| format!("Failed to fetch template '{}' from GitHub", template_name))?;

        if !response.status().is_success() {
            if response.status() == 404 {
                return Err(anyhow!(
                    "Template '{}' not found in repository",
                    template_name
                ));
            }
            return Err(anyhow!(
                "GitHub API request failed with status: {} - {}",
                response.status(),
                Self::create_github_error_message(&anyhow!("API request failed"))
            ));
        }

        let github_response: GitHubFileResponse = response
            .json()
            .await
            .context("Failed to parse GitHub API response")?;

        let content = if github_response.encoding == "base64" {
            let decoded = base64::engine::general_purpose::STANDARD
                .decode(github_response.content.replace('\n', ""))
                .context("Failed to decode base64 content")?;
            String::from_utf8(decoded).context("Invalid UTF-8 in decoded content")?
        } else {
            github_response.content
        };

        let template: Template = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse template '{}' JSON", template_name))?;

        Ok(template)
    }

    /// Create a helpful error message for GitHub-related errors
    pub fn create_github_error_message(error: &anyhow::Error) -> String {
        let error_str = error.to_string().to_lowercase();

        if error_str.contains("network") || error_str.contains("connection") {
            "Network connection failed. Please check your internet connection and try again."
                .to_string()
        } else if error_str.contains("timeout") {
            "Request timed out. GitHub might be experiencing issues. Please try again later."
                .to_string()
        } else if error_str.contains("rate limit") {
            "GitHub API rate limit exceeded. Please wait a few minutes before trying again."
                .to_string()
        } else if error_str.contains("404") || error_str.contains("not found") {
            "Template not found in the repository. It may have been moved or removed.".to_string()
        } else if error_str.contains("403") || error_str.contains("forbidden") {
            "Access denied. The repository might be private or you may have exceeded rate limits."
                .to_string()
        } else {
            format!("GitHub API error: {}", error)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_github_client_creation() {
        let client = GitHubClient::new();
        assert_eq!(client.repo.owner, "AndyCross");
        assert_eq!(client.repo.repo, "mcp-forge-templates");
        assert_eq!(client.repo.branch, "master");
    }

    #[test]
    fn test_error_message_creation() {
        let network_error = anyhow!("network connection failed");
        let message = GitHubClient::create_github_error_message(&network_error);
        assert!(message.contains("Network connection failed"));

        let timeout_error = anyhow!("request timeout");
        let message = GitHubClient::create_github_error_message(&timeout_error);
        assert!(message.contains("Request timed out"));

        let not_found_error = anyhow!("404 not found");
        let message = GitHubClient::create_github_error_message(&not_found_error);
        assert!(message.contains("Template not found"));
    }

    #[test]
    fn test_mock_template_creation() {
        // Test that we can create basic template structures
        let template = Template {
            name: "test".to_string(),
            version: "1.0.0".to_string(),
            description: "Test template".to_string(),
            author: "Test".to_string(),
            tags: vec!["test".to_string()],
            platforms: vec!["macos".to_string()],
            variables: HashMap::new(),
            config: crate::templates::TemplateConfig {
                command: "echo".to_string(),
                args: vec!["test".to_string()],
                env: None,
            },
            requirements: None,
            setup_instructions: None,
        };

        assert_eq!(template.name, "test");
        assert_eq!(template.version, "1.0.0");
    }
}
