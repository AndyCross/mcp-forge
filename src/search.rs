use crate::config::McpServer;
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::collections::HashMap;

/// Search criteria for filtering servers and templates
#[derive(Debug, Clone)]
pub struct SearchCriteria {
    pub text: Option<String>,
    pub tags: Vec<String>,
    pub platform: Option<String>,
    pub author: Option<String>,
    pub requires: Option<String>,
}

/// List formatting options
#[derive(Debug, Clone)]
pub struct ListOptions {
    pub sort: Option<String>,
    pub desc: bool,
    pub format: Option<String>,
    pub show_requirements: bool,
    pub json: bool,
}

/// Search ranking for templates
#[derive(Debug, Clone)]
pub struct SearchRanking {
    pub relevance_score: f32,
    pub download_count: u32,
    pub last_updated: DateTime<Utc>,
    pub quality_score: f32,
    pub community_rating: f32,
}

impl Default for SearchRanking {
    fn default() -> Self {
        Self {
            relevance_score: 0.0,
            download_count: 0,
            last_updated: Utc::now(),
            quality_score: 0.0,
            community_rating: 0.0,
        }
    }
}

/// Enhanced server information for display
#[derive(Debug, Clone, Serialize)]
pub struct ServerInfo {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub env: Option<HashMap<String, String>>,
    pub template: Option<String>,
    pub tags: Vec<String>,
    pub platform: String,
    pub author: Option<String>,
    pub requirements: Option<HashMap<String, String>>,
}

impl From<(String, McpServer)> for ServerInfo {
    fn from((name, server): (String, McpServer)) -> Self {
        Self {
            name,
            command: server.command,
            args: server.args,
            env: server.env,
            template: None, // Will be enriched if available
            tags: vec![],   // Will be enriched if available
            platform: get_current_platform(),
            author: None,       // Will be enriched if available
            requirements: None, // Will be enriched if available
        }
    }
}

/// Filter servers based on search criteria
pub fn filter_servers(
    servers: Vec<(String, McpServer)>,
    criteria: &SearchCriteria,
) -> Vec<ServerInfo> {
    let mut filtered: Vec<ServerInfo> = servers
        .into_iter()
        .map(ServerInfo::from)
        .filter(|server| matches_criteria(server, criteria))
        .collect();

    // Apply text search if specified
    if let Some(text) = &criteria.text {
        let text_lower = text.to_lowercase();
        filtered.retain(|server| {
            server.name.to_lowercase().contains(&text_lower)
                || server.command.to_lowercase().contains(&text_lower)
                || server
                    .args
                    .iter()
                    .any(|arg| arg.to_lowercase().contains(&text_lower))
        });
    }

    filtered
}

/// Check if server matches the search criteria
fn matches_criteria(server: &ServerInfo, criteria: &SearchCriteria) -> bool {
    // Check platform filter
    if let Some(platform) = &criteria.platform {
        if &server.platform != platform {
            return false;
        }
    }

    // Check author filter
    if let Some(author) = &criteria.author {
        if server.author.as_ref() != Some(author) {
            return false;
        }
    }

    // Check requirements filter
    if let Some(req) = &criteria.requires {
        if let Some(requirements) = &server.requirements {
            if !requirements.contains_key(req) {
                return false;
            }
        } else {
            return false;
        }
    }

    // Check tags filter
    if !criteria.tags.is_empty() && !criteria.tags.iter().any(|tag| server.tags.contains(tag)) {
        return false;
    }

    true
}

/// Sort servers based on specified field
pub fn sort_servers(mut servers: Vec<ServerInfo>, options: &ListOptions) -> Vec<ServerInfo> {
    if let Some(sort_field) = &options.sort {
        match sort_field.as_str() {
            "name" => {
                servers.sort_by(|a, b| a.name.cmp(&b.name));
            }
            "command" => {
                servers.sort_by(|a, b| a.command.cmp(&b.command));
            }
            "author" => {
                servers.sort_by(|a, b| {
                    a.author
                        .as_deref()
                        .unwrap_or("")
                        .cmp(b.author.as_deref().unwrap_or(""))
                });
            }
            _ => {
                // Default to name sorting for unknown fields
                servers.sort_by(|a, b| a.name.cmp(&b.name));
            }
        }

        if options.desc {
            servers.reverse();
        }
    }

    servers
}

/// Format servers for output
pub fn format_servers(servers: &[ServerInfo], options: &ListOptions) -> String {
    if options.json {
        return serde_json::to_string_pretty(servers).unwrap_or_else(|_| "[]".to_string());
    }

    match options.format.as_deref() {
        Some("table") => format_as_table(servers, options),
        Some("json") => serde_json::to_string_pretty(servers).unwrap_or_else(|_| "[]".to_string()),
        _ => format_as_default(servers, options),
    }
}

/// Format servers as a table
fn format_as_table(servers: &[ServerInfo], options: &ListOptions) -> String {
    if servers.is_empty() {
        return "No servers found.".to_string();
    }

    let mut output = String::new();

    // Header
    output.push_str("┌─────────────────────┬─────────────────────┬─────────────────────┐\n");
    output.push_str("│ Name                │ Command             │ Arguments           │\n");
    output.push_str("├─────────────────────┼─────────────────────┼─────────────────────┤\n");

    // Rows
    for server in servers {
        let name = truncate_string(&server.name, 19);
        let command = truncate_string(&server.command, 19);
        let args = truncate_string(&server.args.join(" "), 19);

        output.push_str(&format!(
            "│ {:<19} │ {:<19} │ {:<19} │\n",
            name, command, args
        ));
    }

    // Footer
    output.push_str("└─────────────────────┴─────────────────────┴─────────────────────┘\n");

    if options.show_requirements {
        output.push('\n');
        for server in servers {
            if let Some(requirements) = &server.requirements {
                output.push_str(&format!("Requirements for {}:\n", server.name));
                for (req, version) in requirements {
                    output.push_str(&format!("  • {}: {}\n", req, version));
                }
                output.push('\n');
            }
        }
    }

    output
}

/// Format servers in default style
fn format_as_default(servers: &[ServerInfo], options: &ListOptions) -> String {
    if servers.is_empty() {
        return "No servers found.".to_string();
    }

    let mut output = String::new();
    output.push_str("Configured MCP Servers:\n");
    output.push_str("─────────────────────\n");

    for server in servers {
        output.push_str(&format!("• {}\n", server.name));
        output.push_str(&format!("  Command: {}\n", server.command));

        if !server.args.is_empty() {
            output.push_str(&format!("  Args: {}\n", server.args.join(" ")));
        }

        if let Some(env) = &server.env {
            if !env.is_empty() {
                output.push_str("  Environment:\n");
                for (key, value) in env {
                    let masked_value = crate::utils::mask_sensitive_env_value(key, value);
                    output.push_str(&format!("    {}={}\n", key, masked_value));
                }
            }
        }

        if !server.tags.is_empty() {
            output.push_str(&format!("  Tags: {}\n", server.tags.join(", ")));
        }

        if let Some(author) = &server.author {
            output.push_str(&format!("  Author: {}\n", author));
        }

        if options.show_requirements {
            if let Some(requirements) = &server.requirements {
                output.push_str("  Requirements:\n");
                for (req, version) in requirements {
                    output.push_str(&format!("    • {}: {}\n", req, version));
                }
            }
        }

        output.push('\n');
    }

    output.push_str(&format!("Total: {} server(s)\n", servers.len()));
    output
}

/// Calculate search ranking for templates
pub fn calculate_ranking(
    template_name: &str,
    search_term: &str,
    metadata: Option<&crate::templates::TemplateMetadata>,
) -> SearchRanking {
    let mut ranking = SearchRanking::default();

    // Calculate relevance score based on name and description matches
    let name_match = if template_name
        .to_lowercase()
        .contains(&search_term.to_lowercase())
    {
        if template_name.to_lowercase() == search_term.to_lowercase() {
            1.0 // Exact match
        } else if template_name
            .to_lowercase()
            .starts_with(&search_term.to_lowercase())
        {
            0.8 // Prefix match
        } else {
            0.6 // Contains match
        }
    } else {
        0.0
    };

    let description_match = if let Some(meta) = metadata {
        if meta
            .description
            .to_lowercase()
            .contains(&search_term.to_lowercase())
        {
            0.4
        } else {
            0.0
        }
    } else {
        0.0
    };

    ranking.relevance_score = name_match + description_match;

    // Creative ranking factors based on template characteristics
    if let Some(meta) = metadata {
        // Official templates get higher quality score
        ranking.quality_score = match meta.category.as_str() {
            "official" => 1.0,
            "community" => 0.7,
            "experimental" => 0.4,
            _ => 0.5,
        };

        // Popular tags get bonus points
        let popular_tags = ["database", "filesystem", "search", "api", "web", "core"];
        let tag_bonus: f32 = meta
            .tags
            .iter()
            .map(|tag| {
                if popular_tags.contains(&tag.as_str()) {
                    0.1
                } else {
                    0.05
                }
            })
            .sum();
        ranking.quality_score += tag_bonus;

        // Cross-platform templates get higher score
        let platform_bonus = match meta.platforms.len() {
            3 => 0.2, // All platforms
            2 => 0.1, // Two platforms
            _ => 0.0, // Single platform
        };
        ranking.quality_score += platform_bonus;

        // Simulate download count based on template characteristics
        ranking.download_count = match meta.category.as_str() {
            "official" => {
                let base = match template_name {
                    "filesystem" => 10000,
                    "brave-search" => 7500,
                    "sqlite" => 5000,
                    "postgres" => 4500,
                    "github" => 6000,
                    _ => 1000,
                };
                base + (ranking.quality_score * 1000.0) as u32
            }
            "community" => (ranking.quality_score * 2000.0) as u32 + 100,
            _ => (ranking.quality_score * 500.0) as u32 + 10,
        };

        // Simulate community rating
        ranking.community_rating = ranking.quality_score * 5.0; // Scale to 0-5 stars
    }

    ranking
}

/// Rank and sort templates by relevance and quality
pub fn rank_templates(
    templates: Vec<crate::templates::TemplateMetadata>,
    search_term: &str,
    rank_by: Option<&str>,
) -> Vec<(crate::templates::TemplateMetadata, SearchRanking)> {
    let mut ranked: Vec<_> = templates
        .into_iter()
        .map(|template| {
            let ranking = calculate_ranking(&template.name, search_term, Some(&template));
            (template, ranking)
        })
        // Filter out templates with zero relevance (no match to search term)
        .filter(|(_, ranking)| ranking.relevance_score > 0.0)
        .collect();

    // Sort by specified ranking criteria
    match rank_by {
        Some("downloads") => {
            ranked.sort_by(|a, b| b.1.download_count.cmp(&a.1.download_count));
        }
        Some("rating") => {
            ranked.sort_by(|a, b| {
                b.1.community_rating
                    .partial_cmp(&a.1.community_rating)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        }
        Some("updated") => {
            ranked.sort_by(|a, b| b.1.last_updated.cmp(&a.1.last_updated));
        }
        Some("relevance") => {
            ranked.sort_by(|a, b| {
                let score_a = a.1.relevance_score + a.1.quality_score;
                let score_b = b.1.relevance_score + b.1.quality_score;
                score_b
                    .partial_cmp(&score_a)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        }
        _ => {
            ranked.sort_by(|a, b| {
                let score_a = a.1.relevance_score + a.1.quality_score;
                let score_b = b.1.relevance_score + b.1.quality_score;
                score_b
                    .partial_cmp(&score_a)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        }
    }

    ranked
}

/// Get current platform name
fn get_current_platform() -> String {
    #[cfg(target_os = "windows")]
    return "windows".to_string();
    #[cfg(target_os = "macos")]
    return "macos".to_string();
    #[cfg(target_os = "linux")]
    return "linux".to_string();
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    return "unknown".to_string();
}

/// Truncate string to specified length with ellipsis
fn truncate_string(s: &str, max_length: usize) -> String {
    if s.len() <= max_length {
        format!("{:<width$}", s, width = max_length)
    } else {
        format!("{}...", &s[..max_length.saturating_sub(3)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::McpServer;
    use std::collections::HashMap;

    #[test]
    fn test_filter_servers() {
        let servers = vec![
            (
                "test1".to_string(),
                McpServer {
                    command: "npx".to_string(),
                    args: vec!["filesystem".to_string()],
                    env: None,
                    other: HashMap::new(),
                },
            ),
            (
                "database".to_string(),
                McpServer {
                    command: "psql".to_string(),
                    args: ["-h", "localhost"].iter().map(|s| s.to_string()).collect(),
                    env: None,
                    other: HashMap::new(),
                },
            ),
        ];

        let criteria = SearchCriteria {
            text: Some("database".to_string()),
            tags: vec![],
            platform: None,
            author: None,
            requires: None,
        };

        let filtered = filter_servers(servers, &criteria);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "database");
    }

    #[test]
    fn test_sort_servers() {
        let servers = vec![
            ServerInfo {
                name: "zebra".to_string(),
                command: "z".to_string(),
                args: vec![],
                env: None,
                template: None,
                tags: vec![],
                platform: "macos".to_string(),
                author: None,
                requirements: None,
            },
            ServerInfo {
                name: "alpha".to_string(),
                command: "a".to_string(),
                args: vec![],
                env: None,
                template: None,
                tags: vec![],
                platform: "macos".to_string(),
                author: None,
                requirements: None,
            },
        ];

        let options = ListOptions {
            sort: Some("name".to_string()),
            desc: false,
            format: None,
            show_requirements: false,
            json: false,
        };

        let sorted = sort_servers(servers, &options);
        assert_eq!(sorted[0].name, "alpha");
        assert_eq!(sorted[1].name, "zebra");
    }

    #[test]
    fn test_calculate_ranking() {
        let ranking = calculate_ranking("filesystem", "file", None);
        assert!(ranking.relevance_score > 0.0);

        let ranking_exact = calculate_ranking("filesystem", "filesystem", None);
        assert!(ranking_exact.relevance_score > ranking.relevance_score);
    }

    #[test]
    fn test_truncate_string() {
        assert_eq!(truncate_string("hello", 10), "hello     ");
        assert_eq!(truncate_string("hello world", 8), "hello...");
    }

    #[test]
    fn test_rank_templates_filters_non_matching() {
        use crate::templates::TemplateMetadata;

        let templates = vec![
            TemplateMetadata {
                name: "rightmove".to_string(),
                version: "1.0.0".to_string(),
                description: "UK property search".to_string(),
                author: "test".to_string(),
                tags: vec!["property".to_string()],
                platforms: vec!["linux".to_string()],
                category: "community".to_string(),
                path: "test.json".to_string(),
            },
            TemplateMetadata {
                name: "filesystem".to_string(),
                version: "1.0.0".to_string(),
                description: "File access".to_string(),
                author: "test".to_string(),
                tags: vec!["files".to_string()],
                platforms: vec!["linux".to_string()],
                category: "official".to_string(),
                path: "test.json".to_string(),
            },
        ];

        // Search for "rightmove" should only return rightmove template
        let ranked = rank_templates(templates.clone(), "rightmove", None);
        assert_eq!(ranked.len(), 1);
        assert_eq!(ranked[0].0.name, "rightmove");

        // Search for non-existent term should return no results
        let ranked = rank_templates(templates, "nonexistent", None);
        assert_eq!(ranked.len(), 0);
    }
}
