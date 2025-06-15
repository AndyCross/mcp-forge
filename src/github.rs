use anyhow::{Context, Result};
use serde::Deserialize;
use crate::templates::{Template, TemplateCatalog};
use std::collections::HashMap;
use base64::Engine;

/// GitHub repository information for templates
#[derive(Debug, Clone)]
pub struct TemplateRepository {
    pub owner: String,
    pub repo: String,
    pub branch: String,
}

impl Default for TemplateRepository {
    fn default() -> Self {
        Self {
            owner: "modelcontextprotocol".to_string(),
            repo: "servers".to_string(),
            branch: "main".to_string(),
        }
    }
}

/// GitHub API response for repository files
#[derive(Deserialize)]
struct GitHubFileResponse {
    content: String,
    encoding: String,
}

/// GitHub client for template operations
pub struct GitHubClient {
    client: reqwest::Client,
    repo: TemplateRepository,
    base_url: String,
}

impl GitHubClient {
    /// Create a new GitHub client
    pub fn new() -> Self {
        let repo = TemplateRepository::default();
        let base_url = format!("https://api.github.com/repos/{}/{}", repo.owner, repo.repo);
        
        Self {
            client: reqwest::Client::new(),
            repo,
            base_url,
        }
    }

    /// Fetch the template catalog from GitHub
    pub async fn fetch_template_catalog(&self) -> Result<TemplateCatalog> {
        let url = format!("{}/contents/catalog.json?ref={}", self.base_url, self.repo.branch);
        
        let response = self.client
            .get(&url)
            .header("User-Agent", "mcp-forge")
            .send()
            .await
            .context("Failed to fetch template catalog")?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to fetch catalog: HTTP {}", response.status());
        }

        let file_response: GitHubFileResponse = response
            .json()
            .await
            .context("Failed to parse GitHub API response")?;

        let content = if file_response.encoding == "base64" {
            String::from_utf8(
                base64::engine::general_purpose::STANDARD
                    .decode(&file_response.content.replace('\n', ""))
                    .context("Failed to decode base64 content")?
            ).context("Invalid UTF-8 in decoded content")?
        } else {
            file_response.content
        };

        let catalog: TemplateCatalog = serde_json::from_str(&content)
            .context("Failed to parse template catalog")?;

        Ok(catalog)
    }

    /// Fetch a specific template from GitHub
    pub async fn fetch_template(&self, template_name: &str) -> Result<Template> {
        // First get the catalog to find the template path
        let catalog = self.fetch_template_catalog().await?;
        
        let template_metadata = catalog.templates.get(template_name)
            .with_context(|| format!("Template '{}' not found in catalog", template_name))?;

        let url = format!("{}/contents/{}?ref={}", self.base_url, template_metadata.path, self.repo.branch);
        
        let response = self.client
            .get(&url)
            .header("User-Agent", "mcp-forge")
            .send()
            .await
            .with_context(|| format!("Failed to fetch template '{}'", template_name))?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to fetch template '{}': HTTP {}", template_name, response.status());
        }

        let file_response: GitHubFileResponse = response
            .json()
            .await
            .context("Failed to parse GitHub API response")?;

        let content = if file_response.encoding == "base64" {
            String::from_utf8(
                base64::engine::general_purpose::STANDARD
                    .decode(&file_response.content.replace('\n', ""))
                    .context("Failed to decode base64 content")?
            ).context("Invalid UTF-8 in decoded content")?
        } else {
            file_response.content
        };

        let template: Template = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse template '{}'", template_name))?;

        Ok(template)
    }

    /// Create a beautiful error message for GitHub failures
    pub fn create_github_error_message(error: &anyhow::Error) -> String {
        let error_str = error.to_string();
        
        if error_str.contains("rate limit") {
            format!(
                "🚫 GitHub API Rate Limit Exceeded\n\n\
                The GitHub API rate limit has been reached. This happens when making too many requests.\n\
                \n\
                💡 What you can do:\n\
                • Wait a few minutes and try again\n\
                • Use cached templates: mcp-forge template list --cached\n\
                • The rate limit resets every hour\n\
                \n\
                ℹ️  Note: mcp-forge works offline with cached templates for exactly this reason!"
            )
        } else if error_str.contains("not found") || error_str.contains("404") {
            format!(
                "📂 Template Repository Not Found\n\n\
                The template repository could not be found or accessed.\n\
                \n\
                💡 This might mean:\n\
                • The repository is still being set up\n\
                • There's a temporary network issue\n\
                • The repository URL has changed\n\
                \n\
                🔄 Try: mcp-forge template refresh --force"
            )
        } else if error_str.contains("network") || error_str.contains("timeout") {
            format!(
                "🌐 Network Connection Issue\n\n\
                Unable to connect to GitHub to fetch templates.\n\
                \n\
                💡 What you can do:\n\
                • Check your internet connection\n\
                • Try again in a few moments\n\
                • Use offline mode: mcp-forge template list --offline\n\
                \n\
                ✨ mcp-forge is designed to work offline with cached templates!"
            )
        } else {
            format!(
                "⚠️  GitHub Integration Error\n\n\
                An unexpected error occurred while fetching templates from GitHub.\n\
                \n\
                Error details: {}\n\
                \n\
                💡 Suggestions:\n\
                • Try using cached templates: mcp-forge template list --cached\n\
                • Clear and refresh cache: mcp-forge template refresh --clear\n\
                • Check GitHub status: https://status.github.com\n\
                \n\
                🔄 You can continue using mcp-forge with locally cached templates.",
                error_str
            )
        }
    }
}

/// Create mock templates for testing/development
pub fn create_mock_template(name: &str) -> Option<Template> {
    match name {
        "filesystem" => Some(Template {
            name: "filesystem".to_string(),
            version: "1.0.0".to_string(),
            description: "Access local filesystem from Claude".to_string(),
            author: "Anthropic".to_string(),
            tags: vec!["filesystem".to_string(), "official".to_string(), "core".to_string()],
            platforms: vec!["windows".to_string(), "macos".to_string(), "linux".to_string()],
            variables: {
                let mut vars = HashMap::new();
                vars.insert("paths".to_string(), crate::templates::TemplateVariable {
                    var_type: crate::templates::VariableType::Array,
                    description: "Directories to grant access to".to_string(),
                    default: Some(serde_json::json!(["{{home_dir}}/Desktop", "{{home_dir}}/Downloads"])),
                    required: true,
                    validation: Some("path_exists".to_string()),
                    options: None,
                });
                vars.insert("readonly".to_string(), crate::templates::TemplateVariable {
                    var_type: crate::templates::VariableType::Boolean,
                    description: "Read-only access".to_string(),
                    default: Some(serde_json::json!(false)),
                    required: false,
                    validation: None,
                    options: None,
                });
                vars
            },
            config: crate::templates::TemplateConfig {
                command: "npx".to_string(),
                args: vec![
                    "-y".to_string(),
                    "@modelcontextprotocol/server-filesystem".to_string(),
                    "{{#each paths}}{{this}}{{#unless @last}} {{/unless}}{{/each}}".to_string()
                ],
                env: Some({
                    let mut env = HashMap::new();
                    env.insert("{{#if readonly}}READONLY{{/if}}".to_string(), "{{readonly}}".to_string());
                    env
                }),
            },
            requirements: Some({
                let mut req = HashMap::new();
                req.insert("nodejs".to_string(), ">=18.0.0".to_string());
                req
            }),
            setup_instructions: Some("Requires Node.js. Run 'node --version' to verify installation.".to_string()),
        }),
        "brave-search" => Some(Template {
            name: "brave-search".to_string(),
            version: "1.0.0".to_string(),
            description: "Search the web using Brave Search API".to_string(),
            author: "Anthropic".to_string(),
            tags: vec!["search".to_string(), "web".to_string(), "official".to_string()],
            platforms: vec!["windows".to_string(), "macos".to_string(), "linux".to_string()],
            variables: {
                let mut vars = HashMap::new();
                vars.insert("api_key".to_string(), crate::templates::TemplateVariable {
                    var_type: crate::templates::VariableType::String,
                    description: "Brave Search API key".to_string(),
                    default: None,
                    required: true,
                    validation: None,
                    options: None,
                });
                vars
            },
            config: crate::templates::TemplateConfig {
                command: "npx".to_string(),
                args: vec![
                    "-y".to_string(),
                    "@modelcontextprotocol/server-brave-search".to_string()
                ],
                env: Some({
                    let mut env = HashMap::new();
                    env.insert("BRAVE_API_KEY".to_string(), "{{api_key}}".to_string());
                    env
                }),
            },
            requirements: Some({
                let mut req = HashMap::new();
                req.insert("nodejs".to_string(), ">=18.0.0".to_string());
                req
            }),
            setup_instructions: Some("Get your API key from https://brave.com/search/api/".to_string()),
        }),
        "postgres" => Some(Template {
            name: "postgres".to_string(),
            version: "1.0.0".to_string(),
            description: "Query PostgreSQL databases from Claude".to_string(),
            author: "Anthropic".to_string(),
            tags: vec!["database".to_string(), "sql".to_string(), "official".to_string()],
            platforms: vec!["windows".to_string(), "macos".to_string(), "linux".to_string()],
            variables: {
                let mut vars = HashMap::new();
                vars.insert("host".to_string(), crate::templates::TemplateVariable {
                    var_type: crate::templates::VariableType::String,
                    description: "PostgreSQL server host".to_string(),
                    default: Some(serde_json::json!("localhost")),
                    required: true,
                    validation: None,
                    options: None,
                });
                vars.insert("port".to_string(), crate::templates::TemplateVariable {
                    var_type: crate::templates::VariableType::Number,
                    description: "PostgreSQL server port".to_string(),
                    default: Some(serde_json::json!(5432)),
                    required: false,
                    validation: None,
                    options: None,
                });
                vars.insert("database".to_string(), crate::templates::TemplateVariable {
                    var_type: crate::templates::VariableType::String,
                    description: "Database name".to_string(),
                    default: None,
                    required: true,
                    validation: None,
                    options: None,
                });
                vars.insert("username".to_string(), crate::templates::TemplateVariable {
                    var_type: crate::templates::VariableType::String,
                    description: "Database username".to_string(),
                    default: None,
                    required: true,
                    validation: None,
                    options: None,
                });
                vars.insert("password".to_string(), crate::templates::TemplateVariable {
                    var_type: crate::templates::VariableType::String,
                    description: "Database password".to_string(),
                    default: None,
                    required: true,
                    validation: None,
                    options: None,
                });
                vars.insert("ssl_mode".to_string(), crate::templates::TemplateVariable {
                    var_type: crate::templates::VariableType::Select,
                    description: "SSL connection mode".to_string(),
                    default: Some(serde_json::json!("prefer")),
                    required: false,
                    validation: None,
                    options: Some(vec![
                        "disable".to_string(),
                        "allow".to_string(),
                        "prefer".to_string(),
                        "require".to_string(),
                        "verify-ca".to_string(),
                        "verify-full".to_string()
                    ]),
                });
                vars
            },
            config: crate::templates::TemplateConfig {
                command: "npx".to_string(),
                args: vec![
                    "-y".to_string(),
                    "@modelcontextprotocol/server-postgres".to_string()
                ],
                env: Some({
                    let mut env = HashMap::new();
                    env.insert("POSTGRES_HOST".to_string(), "{{host}}".to_string());
                    env.insert("POSTGRES_PORT".to_string(), "{{port}}".to_string());
                    env.insert("POSTGRES_DB".to_string(), "{{database}}".to_string());
                    env.insert("POSTGRES_USER".to_string(), "{{username}}".to_string());
                    env.insert("POSTGRES_PASSWORD".to_string(), "{{password}}".to_string());
                    env.insert("POSTGRES_SSL_MODE".to_string(), "{{ssl_mode}}".to_string());
                    env
                }),
            },
            requirements: Some({
                let mut req = HashMap::new();
                req.insert("nodejs".to_string(), ">=18.0.0".to_string());
                req.insert("postgres".to_string(), ">=12.0.0".to_string());
                req
            }),
            setup_instructions: Some("Ensure PostgreSQL is running and accessible. Create the database and user if they don't exist. Consider using environment variables or a .env file for sensitive credentials.".to_string()),
        }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_github_client_creation() {
        let client = GitHubClient::new();
        assert_eq!(client.repo.owner, "modelcontextprotocol");
        assert_eq!(client.repo.repo, "servers");
        assert_eq!(client.repo.branch, "main");
    }

    #[test]
    fn test_mock_template_creation() {
        let template = create_mock_template("filesystem").unwrap();
        assert_eq!(template.name, "filesystem");
        assert!(!template.variables.is_empty());
        assert!(template.variables.contains_key("paths"));
    }

    #[test]
    fn test_error_message_creation() {
        let rate_limit_error = anyhow::anyhow!("GitHub API rate limit exceeded");
        let message = GitHubClient::create_github_error_message(&rate_limit_error);
        assert!(message.contains("Rate Limit"));
        assert!(message.contains("cached templates"));
    }
} 