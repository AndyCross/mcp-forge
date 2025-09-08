use anyhow::{Context, Result};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Template variable types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum VariableType {
    String,
    Boolean,
    Number,
    Array,
    Select,
}

/// Template variable definition with enhanced validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateVariable {
    #[serde(rename = "type")]
    pub var_type: VariableType,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    #[serde(default)]
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>, // For select type
}

/// Enhanced template configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub tags: Vec<String>,
    pub platforms: Vec<String>,
    pub variables: HashMap<String, TemplateVariable>,
    pub config: TemplateConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_instructions: Option<String>,
}

/// Template configuration section
/// Supports both command-based and URL-based servers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
}

impl TemplateConfig {
    /// Validate the template configuration
    pub fn validate(&self) -> Result<()> {
        // A template must have either a URL or a command, but not both
        match (self.url.as_ref(), self.command.as_ref()) {
            (Some(_), Some(_)) => {
                anyhow::bail!("Template cannot have both 'url' and 'command' fields")
            }
            (None, None) => {
                anyhow::bail!("Template must have either 'url' or 'command' field")
            }
            (Some(_), None) => {
                // URL template - valid
                Ok(())
            }
            (None, Some(_)) => {
                // Command template - valid
                Ok(())
            }
        }
    }

    /// Check if this is a URL-type template
    pub fn is_url_template(&self) -> bool {
        self.url.is_some()
    }

    /// Check if this is a command-type template
    pub fn is_command_template(&self) -> bool {
        self.command.is_some()
    }
}

/// Template catalog for repository index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateCatalog {
    pub version: String,
    pub last_updated: String,
    pub templates: HashMap<String, TemplateMetadata>,
}

/// Template metadata in catalog
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub tags: Vec<String>,
    pub platforms: Vec<String>,
    pub category: String, // "official", "community", "experimental"
    pub path: String,     // Path in repository
}

/// Cache metadata for tracking updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMetadata {
    pub last_refresh: chrono::DateTime<chrono::Utc>,
    pub etag: Option<String>,
    pub catalog_etag: Option<String>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

impl Default for CacheMetadata {
    fn default() -> Self {
        Self {
            last_refresh: chrono::Utc::now(),
            etag: None,
            catalog_etag: None,
            expires_at: chrono::Utc::now() + chrono::Duration::days(30), // 1 month cache
        }
    }
}

/// Template manager for handling template operations
pub struct TemplateManager {
    cache_dir: PathBuf,
    templates_dir: PathBuf,
    handlebars: Handlebars<'static>,
    github_client: crate::github::GitHubClient,
}

impl TemplateManager {
    /// Create a new template manager
    pub fn new() -> Result<Self> {
        let cache_dir = dirs::cache_dir()
            .ok_or_else(|| anyhow::anyhow!("Unable to determine cache directory"))?
            .join("mcp-forge");

        let templates_dir = cache_dir.join("templates");

        std::fs::create_dir_all(&cache_dir)?;
        std::fs::create_dir_all(&templates_dir)?;

        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(true);

        // Register built-in helpers
        handlebars.register_helper("os", Box::new(os_helper));
        handlebars.register_helper("arch", Box::new(arch_helper));
        handlebars.register_helper("home_dir", Box::new(home_dir_helper));
        handlebars.register_helper("config_dir", Box::new(config_dir_helper));

        Ok(Self {
            cache_dir,
            templates_dir,
            handlebars,
            github_client: crate::github::GitHubClient::new(),
        })
    }

    /// Get cache metadata file path
    fn cache_metadata_path(&self) -> PathBuf {
        self.cache_dir.join("metadata.json")
    }

    /// Get catalog cache file path
    fn catalog_cache_path(&self) -> PathBuf {
        self.cache_dir.join("catalog.json")
    }

    /// Get template cache file path
    fn template_cache_path(&self, name: &str) -> PathBuf {
        self.templates_dir.join(format!("{}.json", name))
    }

    /// Load cache metadata
    fn load_cache_metadata(&self) -> Result<CacheMetadata> {
        let path = self.cache_metadata_path();
        if !path.exists() {
            return Ok(CacheMetadata::default());
        }

        let content = std::fs::read_to_string(&path).context("Failed to read cache metadata")?;

        serde_json::from_str(&content).context("Failed to parse cache metadata")
    }

    /// Save cache metadata
    fn save_cache_metadata(&self, metadata: &CacheMetadata) -> Result<()> {
        let content =
            serde_json::to_string_pretty(metadata).context("Failed to serialize cache metadata")?;

        std::fs::write(self.cache_metadata_path(), content).context("Failed to save cache metadata")
    }

    /// Check if cache is expired
    fn is_cache_expired(&self) -> Result<bool> {
        let metadata = self.load_cache_metadata()?;
        Ok(chrono::Utc::now() > metadata.expires_at)
    }

    /// Load template catalog from cache
    pub fn load_cached_catalog(&self) -> Result<Option<TemplateCatalog>> {
        let path = self.catalog_cache_path();
        if !path.exists() {
            return Ok(None);
        }

        let content = std::fs::read_to_string(&path).context("Failed to read cached catalog")?;

        let catalog: TemplateCatalog =
            serde_json::from_str(&content).context("Failed to parse cached catalog")?;

        Ok(Some(catalog))
    }

    /// Save template catalog to cache
    pub fn save_catalog_cache(&self, catalog: &TemplateCatalog) -> Result<()> {
        let content =
            serde_json::to_string_pretty(catalog).context("Failed to serialize catalog")?;

        std::fs::write(self.catalog_cache_path(), content).context("Failed to save catalog cache")
    }

    /// Load template from cache
    pub fn load_cached_template(&self, name: &str) -> Result<Option<Template>> {
        let path = self.template_cache_path(name);
        if !path.exists() {
            return Ok(None);
        }

        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read cached template: {}", name))?;

        let template: Template = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse cached template: {}", name))?;

        Ok(Some(template))
    }

    /// Save template to cache
    pub fn save_template_cache(&self, template: &Template) -> Result<()> {
        let content =
            serde_json::to_string_pretty(template).context("Failed to serialize template")?;

        std::fs::write(self.template_cache_path(&template.name), content)
            .with_context(|| format!("Failed to save template cache: {}", template.name))
    }

    /// Load template (from cache or GitHub)
    pub async fn load_template(&self, name: &str) -> Result<Template> {
        // Try cache first if not expired
        if !self.is_cache_expired()? {
            if let Some(template) = self.load_cached_template(name)? {
                return Ok(template);
            }
        }

        // Fetch from GitHub
        let template = self.github_client.fetch_template(name).await?;

        // Cache the template
        self.save_template_cache(&template)?;

        Ok(template)
    }

    /// List available templates
    pub async fn list_templates(&self) -> Result<Vec<TemplateMetadata>> {
        let catalog = self.load_catalog().await?;
        Ok(catalog.templates.into_values().collect())
    }

    /// Load catalog (from cache or GitHub)
    pub async fn load_catalog(&self) -> Result<TemplateCatalog> {
        // Try cache first
        if let Ok(Some(catalog)) = self.load_cached_catalog() {
            if !self.is_cache_expired().unwrap_or(true) {
                return Ok(catalog);
            }
        }

        // Fetch from GitHub
        let catalog = self.github_client.fetch_template_catalog().await?;

        // Cache it
        self.save_catalog_cache(&catalog)?;

        Ok(catalog)
    }

    /// Apply template variables to generate MCP server configuration
    pub fn apply_template(
        &self,
        template: &Template,
        variables: &HashMap<String, serde_json::Value>,
    ) -> Result<crate::config::McpServer> {
        // Validate template configuration first
        template.config.validate()?;
        
        // Validate variables
        self.validate_variables(template, variables)?;

        // Create context for template rendering
        let mut context = serde_json::Map::new();
        for (key, value) in variables {
            context.insert(key.clone(), value.clone());
        }

        // Check if this is a URL template or command template
        if template.config.is_url_template() {
            // Render URL
            let url = template.config.url.as_ref().unwrap();
            let rendered_url = self
                .handlebars
                .render_template(url, &context)
                .with_context(|| format!("Failed to render URL template: {}", url))?;

            // Render environment variables if present
            let rendered_env = self.render_env(&template.config.env, &context)?;

            Ok(crate::config::McpServer {
                command: None,
                args: None,
                url: Some(rendered_url),
                env: rendered_env,
                other: HashMap::new(),
            })
        } else {
            // Render command
            let command = template.config.command.as_ref().unwrap();
            let rendered_command = self
                .handlebars
                .render_template(command, &context)
                .with_context(|| format!("Failed to render command template: {}", command))?;

            // Render arguments
            let rendered_args = if let Some(args) = &template.config.args {
                let mut rendered = Vec::new();
                for arg in args {
                    let rendered_arg = self
                        .handlebars
                        .render_template(arg, &context)
                        .with_context(|| format!("Failed to render argument template: {}", arg))?;
                    rendered.push(rendered_arg);
                }
                Some(rendered)
            } else {
                Some(Vec::new()) // Default to empty args for command servers
            };

            // Render environment variables if present
            let rendered_env = self.render_env(&template.config.env, &context)?;

            Ok(crate::config::McpServer {
                command: Some(rendered_command),
                args: rendered_args,
                url: None,
                env: rendered_env,
                other: HashMap::new(),
            })
        }
    }

    /// Helper method to render environment variables
    fn render_env(
        &self,
        env: &Option<HashMap<String, String>>,
        context: &serde_json::Map<String, serde_json::Value>,
    ) -> Result<Option<HashMap<String, String>>> {
        if let Some(env) = env {
            let mut rendered_env_map = HashMap::new();
            for (key, value) in env {
                let rendered_key = self
                    .handlebars
                    .render_template(key, context)
                    .with_context(|| {
                        format!("Failed to render environment key template: {}", key)
                    })?;
                let rendered_value = self
                    .handlebars
                    .render_template(value, context)
                    .with_context(|| {
                        format!("Failed to render environment value template: {}", value)
                    })?;

                // Only add non-empty keys and values
                if !rendered_key.trim().is_empty() && !rendered_value.trim().is_empty() {
                    rendered_env_map.insert(rendered_key, rendered_value);
                }
            }
            if rendered_env_map.is_empty() {
                Ok(None)
            } else {
                Ok(Some(rendered_env_map))
            }
        } else {
            Ok(None)
        }
    }

    /// Validate template variables
    pub fn validate_variables(
        &self,
        template: &Template,
        variables: &HashMap<String, serde_json::Value>,
    ) -> Result<()> {
        // Check required variables
        for (var_name, var_def) in &template.variables {
            if var_def.required {
                if !variables.contains_key(var_name) {
                    anyhow::bail!("Required variable '{}' is missing", var_name);
                }

                let value = &variables[var_name];
                if value.is_null() {
                    anyhow::bail!("Required variable '{}' cannot be null", var_name);
                }

                // For string variables, check if empty
                if var_def.var_type == VariableType::String {
                    if let Some(str_val) = value.as_str() {
                        if str_val.trim().is_empty() {
                            anyhow::bail!("Required variable '{}' cannot be empty", var_name);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Refresh template cache
    pub async fn refresh_cache(&self) -> Result<()> {
        let github_client = crate::github::GitHubClient::new();

        // Fetch fresh catalog
        let catalog = github_client.fetch_template_catalog().await?;
        self.save_catalog_cache(&catalog)?;

        // Update cache metadata
        let metadata = CacheMetadata {
            last_refresh: chrono::Utc::now(),
            expires_at: chrono::Utc::now() + chrono::Duration::days(30),
            ..Default::default()
        };
        self.save_cache_metadata(&metadata)?;

        Ok(())
    }

    /// Clear template cache
    pub fn clear_cache(&self) -> Result<()> {
        if self.cache_dir.exists() {
            std::fs::remove_dir_all(&self.cache_dir).context("Failed to clear cache directory")?;
            std::fs::create_dir_all(&self.cache_dir)
                .context("Failed to recreate cache directory")?;
            std::fs::create_dir_all(&self.templates_dir)
                .context("Failed to recreate templates directory")?;
        }
        Ok(())
    }
}

// Handlebars helper functions
fn os_helper(
    _: &handlebars::Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    out.write(&get_os_name())?;
    Ok(())
}

fn arch_helper(
    _: &handlebars::Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    out.write(&get_arch_name())?;
    Ok(())
}

fn home_dir_helper(
    _: &handlebars::Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    out.write(&get_home_dir())?;
    Ok(())
}

fn config_dir_helper(
    _: &handlebars::Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    out.write(&get_config_dir())?;
    Ok(())
}

// Platform detection functions
fn get_os_name() -> String {
    #[cfg(target_os = "windows")]
    return "windows".to_string();
    #[cfg(target_os = "macos")]
    return "macos".to_string();
    #[cfg(target_os = "linux")]
    return "linux".to_string();
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    return "unknown".to_string();
}

fn get_arch_name() -> String {
    #[cfg(target_arch = "x86_64")]
    return "x64".to_string();
    #[cfg(target_arch = "aarch64")]
    return "arm64".to_string();
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    return "unknown".to_string();
}

fn get_home_dir() -> String {
    dirs::home_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| "~".to_string())
}

fn get_config_dir() -> String {
    crate::utils::get_config_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| "~/.config/claude".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_serialization() {
        let template_json = r#"
        {
            "name": "test-template",
            "version": "1.0.0",
            "description": "Test template",
            "author": "Test Author",
            "tags": ["test"],
            "platforms": ["macos"],
            "variables": {
                "test_var": {
                    "type": "string",
                    "description": "Test variable",
                    "required": true
                }
            },
            "config": {
                "command": "echo",
                "args": ["{{test_var}}"]
            }
        }
        "#;

        let template: Template = serde_json::from_str(template_json).unwrap();
        assert_eq!(template.name, "test-template");
        assert_eq!(template.variables.len(), 1);
    }

    #[test]
    fn test_variable_validation() {
        let template = Template {
            name: "test".to_string(),
            version: "1.0.0".to_string(),
            description: "Test".to_string(),
            author: "Test".to_string(),
            tags: vec!["test".to_string()],
            platforms: vec!["macos".to_string()],
            variables: {
                let mut vars = HashMap::new();
                vars.insert(
                    "required_string".to_string(),
                    TemplateVariable {
                        var_type: VariableType::String,
                        description: "Required string".to_string(),
                        default: None,
                        required: true,
                        validation: None,
                        options: None,
                    },
                );
                vars
            },
            config: TemplateConfig {
                command: Some("echo".to_string()),
                args: Some(vec!["test".to_string()]),
                url: None,
                env: None,
            },
            requirements: None,
            setup_instructions: None,
        };

        let manager = TemplateManager::new().unwrap();

        // Test missing required variable
        let empty_vars = HashMap::new();
        assert!(manager.validate_variables(&template, &empty_vars).is_err());

        // Test valid variable
        let mut valid_vars = HashMap::new();
        valid_vars.insert(
            "required_string".to_string(),
            serde_json::Value::String("test".to_string()),
        );
        assert!(manager.validate_variables(&template, &valid_vars).is_ok());
    }

    #[test]
    fn test_platform_detection() {
        let os = get_os_name();
        assert!(!os.is_empty());

        let arch = get_arch_name();
        assert!(!arch.is_empty());
    }
}
