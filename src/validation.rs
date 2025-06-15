use crate::config::{Config, McpServer};
use crate::utils;
use anyhow::{anyhow, Result};
use colored::Colorize;
use serde::Serialize;
use std::path::Path;
use std::process::Command;

/// Validation status levels
#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum ValidationStatus {
    Valid,
    Warning,
    Error,
    RequirementsMissing,
}

impl ValidationStatus {
    pub fn color(&self) -> colored::Color {
        match self {
            ValidationStatus::Valid => colored::Color::Green,
            ValidationStatus::Warning => colored::Color::Yellow,
            ValidationStatus::Error => colored::Color::Red,
            ValidationStatus::RequirementsMissing => colored::Color::Magenta,
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            ValidationStatus::Valid => "✓",
            ValidationStatus::Warning => "⚠",
            ValidationStatus::Error => "✗",
            ValidationStatus::RequirementsMissing => "📦",
        }
    }
}

/// Individual validation issue
#[derive(Debug, Clone, Serialize)]
pub struct ValidationIssue {
    pub issue_type: String,
    pub message: String,
    pub severity: ValidationStatus,
    pub fix_suggestion: Option<String>,
}

/// Validation result for a single server
#[derive(Debug, Clone, Serialize)]
pub struct ValidationResult {
    pub server_name: String,
    pub status: ValidationStatus,
    pub issues: Vec<ValidationIssue>,
    pub suggestions: Vec<String>,
    pub requirements_checked: bool,
}

/// System diagnostic information
#[derive(Debug, Clone, Serialize)]
pub struct SystemDiagnostic {
    pub platform: String,
    pub node_version: Option<String>,
    pub python_version: Option<String>,
    pub config_file_exists: bool,
    pub config_file_path: String,
    pub config_file_writable: bool,
    pub backup_directory_exists: bool,
    pub total_servers: usize,
    pub issues: Vec<ValidationIssue>,
}

/// Handle validate command
pub async fn handle_validate(
    deep: bool,
    requirements: bool,
    server_name: Option<String>,
    profile: Option<String>,
) -> Result<()> {
    let config = Config::load(profile.as_deref()).await?;

    println!("{}", "Configuration Validation".cyan().bold());
    println!("{}", "────────────────────────".cyan());

    let results = if let Some(name) = server_name {
        if let Some(server) = config.mcp_servers.get(&name) {
            vec![validate_server(&name, server, deep, requirements).await]
        } else {
            return Err(anyhow!("Server '{}' not found", name));
        }
    } else {
        let mut results = Vec::new();
        for (name, server) in &config.mcp_servers {
            results.push(validate_server(name, server, deep, requirements).await);
        }
        results
    };

    display_validation_results(&results);

    let has_errors = results
        .iter()
        .any(|r| matches!(r.status, ValidationStatus::Error));
    let has_warnings = results
        .iter()
        .any(|r| matches!(r.status, ValidationStatus::Warning));

    println!();
    if has_errors {
        println!("{}", "❌ Validation completed with errors".red().bold());
        std::process::exit(1);
    } else if has_warnings {
        println!(
            "{}",
            "⚠️  Validation completed with warnings".yellow().bold()
        );
    } else {
        println!("{}", "✅ All validations passed".green().bold());
    }

    Ok(())
}

/// Handle health check command
pub async fn handle_health_check(profile: Option<String>) -> Result<()> {
    let config = Config::load(profile.as_deref()).await?;

    println!("{}", "System Health Check".cyan().bold());
    println!("{}", "───────────────────".cyan());

    let mut health_issues = Vec::new();
    let mut healthy_count = 0;

    for (name, server) in &config.mcp_servers {
        print!("Checking {} ... ", name);
        let result = validate_server(name, server, true, true).await;

        match result.status {
            ValidationStatus::Valid => {
                println!("{}", "✓ Healthy".green());
                healthy_count += 1;
            }
            ValidationStatus::Warning => {
                println!("{}", "⚠ Issues detected".yellow());
                health_issues.extend(result.issues);
            }
            ValidationStatus::Error | ValidationStatus::RequirementsMissing => {
                println!("{}", "✗ Unhealthy".red());
                health_issues.extend(result.issues);
            }
        }
    }

    println!();
    println!("Health Summary:");
    println!(
        "  Healthy servers: {}/{}",
        healthy_count,
        config.mcp_servers.len()
    );

    if !health_issues.is_empty() {
        println!("  Issues found: {}", health_issues.len());
        println!();
        println!("Issues requiring attention:");
        for issue in &health_issues {
            println!(
                "  {} {}: {}",
                issue.severity.symbol().color(issue.severity.color()),
                issue.issue_type.bold(),
                issue.message
            );
            if let Some(suggestion) = &issue.fix_suggestion {
                println!("    💡 {}", suggestion.italic());
            }
        }
    }

    Ok(())
}

/// Handle validate-all command
pub async fn handle_validate_all(profile: Option<String>) -> Result<()> {
    println!("{}", "Comprehensive Validation".cyan().bold());
    println!("{}", "───────────────────────".cyan());

    // First run health check
    handle_health_check(profile.clone()).await?;

    println!();
    println!("{}", "Configuration Details".cyan().bold());
    println!("{}", "────────────────────".cyan());

    // Then run detailed validation
    handle_validate(true, true, None, profile).await?;

    Ok(())
}

/// Handle doctor command (system diagnostic)
pub async fn handle_doctor(profile: Option<String>) -> Result<()> {
    println!("{}", "System Diagnostic".cyan().bold());
    println!("{}", "─────────────────".cyan());

    let diagnostic = run_system_diagnostic(profile.as_deref()).await?;
    display_diagnostic(&diagnostic);

    Ok(())
}

/// Validate a single server
async fn validate_server(
    name: &str,
    server: &McpServer,
    deep: bool,
    check_requirements: bool,
) -> ValidationResult {
    let mut result = ValidationResult {
        server_name: name.to_string(),
        status: ValidationStatus::Valid,
        issues: Vec::new(),
        suggestions: Vec::new(),
        requirements_checked: check_requirements,
    };

    // Basic validation - command exists and is executable
    validate_command_exists(server, &mut result);

    // Validate arguments
    validate_arguments(server, &mut result);

    // Validate environment variables
    validate_environment(server, &mut result);

    // Check requirements if requested
    if check_requirements {
        validate_requirements(server, &mut result).await;
    }

    // Deep validation if requested
    if deep {
        perform_deep_validation(server, &mut result).await;
    }

    // Determine overall status
    if result
        .issues
        .iter()
        .any(|i| matches!(i.severity, ValidationStatus::Error))
    {
        result.status = ValidationStatus::Error;
    } else if result
        .issues
        .iter()
        .any(|i| matches!(i.severity, ValidationStatus::RequirementsMissing))
    {
        result.status = ValidationStatus::RequirementsMissing;
    } else if result
        .issues
        .iter()
        .any(|i| matches!(i.severity, ValidationStatus::Warning))
    {
        result.status = ValidationStatus::Warning;
    }

    result
}

/// Check if the command exists and is executable
fn validate_command_exists(server: &McpServer, result: &mut ValidationResult) {
    let command = &server.command;

    // Check if it's a full path
    if Path::new(command).is_absolute() {
        if !Path::new(command).exists() {
            result.issues.push(ValidationIssue {
                issue_type: "Command Not Found".to_string(),
                message: format!("Command path '{}' does not exist", command),
                severity: ValidationStatus::Error,
                fix_suggestion: Some("Verify the command path is correct".to_string()),
            });
            return;
        }

        if !is_executable(Path::new(command)) {
            result.issues.push(ValidationIssue {
                issue_type: "Not Executable".to_string(),
                message: format!("Command '{}' is not executable", command),
                severity: ValidationStatus::Error,
                fix_suggestion: Some("Check file permissions".to_string()),
            });
        }
    } else {
        // Check if command is in PATH
        if !command_in_path(command) {
            result.issues.push(ValidationIssue {
                issue_type: "Command Not in PATH".to_string(),
                message: format!("Command '{}' not found in PATH", command),
                severity: ValidationStatus::Error,
                fix_suggestion: Some(format!("Install {} or add it to your PATH", command)),
            });
        }
    }
}

/// Validate command arguments
fn validate_arguments(server: &McpServer, result: &mut ValidationResult) {
    // Check for common problematic argument patterns
    for (i, arg) in server.args.iter().enumerate() {
        // Check for unquoted spaces in file paths
        if arg.contains(' ') && !arg.starts_with('"') && !arg.starts_with('\'') {
            result.issues.push(ValidationIssue {
                issue_type: "Unquoted Argument".to_string(),
                message: format!(
                    "Argument {} '{}' contains spaces but isn't quoted",
                    i + 1,
                    arg
                ),
                severity: ValidationStatus::Warning,
                fix_suggestion: Some("Consider quoting arguments with spaces".to_string()),
            });
        }

        // Check for file/directory arguments that don't exist
        if (arg.starts_with('/') || arg.starts_with("./") || arg.contains(":\\"))
            && !Path::new(arg).exists()
        {
            result.issues.push(ValidationIssue {
                issue_type: "Path Not Found".to_string(),
                message: format!("Path argument '{}' does not exist", arg),
                severity: ValidationStatus::Warning,
                fix_suggestion: Some(
                    "Verify the path exists or will be created at runtime".to_string(),
                ),
            });
        }
    }
}

/// Validate environment variables
fn validate_environment(server: &McpServer, result: &mut ValidationResult) {
    if let Some(env) = &server.env {
        for (key, value) in env {
            // Check for empty values that might be problematic
            if value.is_empty() {
                result.issues.push(ValidationIssue {
                    issue_type: "Empty Environment Variable".to_string(),
                    message: format!("Environment variable '{}' is empty", key),
                    severity: ValidationStatus::Warning,
                    fix_suggestion: Some(
                        "Consider removing unused environment variables".to_string(),
                    ),
                });
            }

            // Check for potential file path environment variables
            if (key.to_uppercase().contains("PATH")
                || key.to_uppercase().contains("DIR")
                || key.to_uppercase().contains("FILE"))
                && !value.is_empty()
            {
                if !Path::new(value).exists() {
                    result.issues.push(ValidationIssue {
                        issue_type: "Environment Path Not Found".to_string(),
                        message: format!(
                            "Environment variable '{}' points to non-existent path '{}'",
                            key, value
                        ),
                        severity: ValidationStatus::Warning,
                        fix_suggestion: Some(
                            "Verify the path exists or will be created at runtime".to_string(),
                        ),
                    });
                }
            }
        }
    }
}

/// Check system requirements for the server
async fn validate_requirements(server: &McpServer, result: &mut ValidationResult) {
    let command = &server.command;

    match command.as_str() {
        "node" | "npx" => {
            if let Some(version) = get_node_version() {
                result
                    .suggestions
                    .push(format!("Node.js version: {}", version));
            } else {
                result.issues.push(ValidationIssue {
                    issue_type: "Missing Requirement".to_string(),
                    message: "Node.js is required but not found".to_string(),
                    severity: ValidationStatus::RequirementsMissing,
                    fix_suggestion: Some("Install Node.js from https://nodejs.org/".to_string()),
                });
            }
        }
        "python" | "python3" => {
            if let Some(version) = get_python_version() {
                result
                    .suggestions
                    .push(format!("Python version: {}", version));
            } else {
                result.issues.push(ValidationIssue {
                    issue_type: "Missing Requirement".to_string(),
                    message: "Python is required but not found".to_string(),
                    severity: ValidationStatus::RequirementsMissing,
                    fix_suggestion: Some("Install Python from https://python.org/".to_string()),
                });
            }
        }
        "uvx" => {
            if !command_in_path("uvx") {
                result.issues.push(ValidationIssue {
                    issue_type: "Missing Requirement".to_string(),
                    message: "uvx is required but not found".to_string(),
                    severity: ValidationStatus::RequirementsMissing,
                    fix_suggestion: Some("Install uvx: pip install uvx".to_string()),
                });
            }
        }
        _ => {}
    }
}

/// Perform deep validation (not network-level as per requirements)
async fn perform_deep_validation(server: &McpServer, result: &mut ValidationResult) {
    // Check for common configuration issues

    // Validate port numbers in arguments
    for arg in &server.args {
        if let Ok(port) = arg.parse::<u16>() {
            if port < 1024 {
                result.issues.push(ValidationIssue {
                    issue_type: "Privileged Port".to_string(),
                    message: format!("Port {} requires elevated privileges", port),
                    severity: ValidationStatus::Warning,
                    fix_suggestion: Some("Consider using a port > 1024".to_string()),
                });
            }
        }
    }

    // Check for potential resource issues
    if server.args.len() > 20 {
        result.issues.push(ValidationIssue {
            issue_type: "Many Arguments".to_string(),
            message: format!(
                "Server has {} arguments, which might indicate complexity",
                server.args.len()
            ),
            severity: ValidationStatus::Warning,
            fix_suggestion: Some(
                "Consider using configuration files instead of many arguments".to_string(),
            ),
        });
    }
}

/// Run comprehensive system diagnostic
async fn run_system_diagnostic(profile: Option<&str>) -> Result<SystemDiagnostic> {
    let mut diagnostic = SystemDiagnostic {
        platform: get_platform_info(),
        node_version: get_node_version(),
        python_version: get_python_version(),
        config_file_exists: false,
        config_file_path: String::new(),
        config_file_writable: false,
        backup_directory_exists: false,
        total_servers: 0,
        issues: Vec::new(),
    };

    // Check configuration file
    match utils::get_claude_config_path() {
        Ok(path) => {
            diagnostic.config_file_path = path.display().to_string();
            diagnostic.config_file_exists = path.exists();
            diagnostic.config_file_writable = is_writable(&path);

            if !diagnostic.config_file_exists {
                diagnostic.issues.push(ValidationIssue {
                    issue_type: "Configuration".to_string(),
                    message: "Claude Desktop configuration file not found".to_string(),
                    severity: ValidationStatus::Warning,
                    fix_suggestion: Some("Run 'mcp-forge config init' to create it".to_string()),
                });
            } else if !diagnostic.config_file_writable {
                diagnostic.issues.push(ValidationIssue {
                    issue_type: "Permissions".to_string(),
                    message: "Configuration file is not writable".to_string(),
                    severity: ValidationStatus::Error,
                    fix_suggestion: Some("Check file permissions".to_string()),
                });
            }
        }
        Err(e) => {
            diagnostic.issues.push(ValidationIssue {
                issue_type: "Configuration".to_string(),
                message: format!("Cannot determine config file location: {}", e),
                severity: ValidationStatus::Error,
                fix_suggestion: None,
            });
        }
    }

    // Check backup directory
    if let Ok(backup_dir) = utils::get_backup_dir() {
        diagnostic.backup_directory_exists = backup_dir.exists();
        if !diagnostic.backup_directory_exists {
            diagnostic.issues.push(ValidationIssue {
                issue_type: "Backup".to_string(),
                message: "Backup directory doesn't exist".to_string(),
                severity: ValidationStatus::Warning,
                fix_suggestion: Some("It will be created automatically when needed".to_string()),
            });
        }
    }

    // Load config to get server count
    if let Ok(config) = Config::load(profile).await {
        diagnostic.total_servers = config.mcp_servers.len();
    }

    Ok(diagnostic)
}

/// Display validation results
fn display_validation_results(results: &[ValidationResult]) {
    for result in results {
        println!();
        let status_symbol = result.status.symbol().color(result.status.color());
        println!(
            "{} {} ({})",
            status_symbol,
            result.server_name.bold(),
            format!("{:?}", result.status).color(result.status.color())
        );

        for issue in &result.issues {
            println!(
                "  {} {}: {}",
                issue.severity.symbol().color(issue.severity.color()),
                issue.issue_type.bold(),
                issue.message
            );
            if let Some(suggestion) = &issue.fix_suggestion {
                println!("    💡 {}", suggestion.italic());
            }
        }

        if !result.suggestions.is_empty() {
            for suggestion in &result.suggestions {
                println!("  ℹ️  {}", suggestion.dimmed());
            }
        }
    }
}

/// Display system diagnostic
fn display_diagnostic(diagnostic: &SystemDiagnostic) {
    println!("Platform: {}", diagnostic.platform.bold());

    if let Some(node) = &diagnostic.node_version {
        println!("Node.js: {}", node.green());
    } else {
        println!("Node.js: {}", "Not installed".red());
    }

    if let Some(python) = &diagnostic.python_version {
        println!("Python: {}", python.green());
    } else {
        println!("Python: {}", "Not installed".red());
    }

    println!();
    println!("Configuration:");
    println!("  File: {}", diagnostic.config_file_path);
    println!(
        "  Exists: {}",
        if diagnostic.config_file_exists {
            "✓".green()
        } else {
            "✗".red()
        }
    );
    println!(
        "  Writable: {}",
        if diagnostic.config_file_writable {
            "✓".green()
        } else {
            "✗".red()
        }
    );
    println!("  Servers: {}", diagnostic.total_servers);

    println!();
    println!(
        "Backup Directory: {}",
        if diagnostic.backup_directory_exists {
            "✓".green()
        } else {
            "✗".yellow()
        }
    );

    if !diagnostic.issues.is_empty() {
        println!();
        println!("System Issues:");
        for issue in &diagnostic.issues {
            println!(
                "  {} {}: {}",
                issue.severity.symbol().color(issue.severity.color()),
                issue.issue_type.bold(),
                issue.message
            );
            if let Some(suggestion) = &issue.fix_suggestion {
                println!("    💡 {}", suggestion.italic());
            }
        }
    } else {
        println!();
        println!("{}", "✅ System appears healthy".green().bold());
    }
}

/// Helper functions

fn get_platform_info() -> String {
    format!("{} {}", std::env::consts::OS, std::env::consts::ARCH)
}

fn get_node_version() -> Option<String> {
    Command::new("node")
        .arg("--version")
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout)
                    .ok()
                    .map(|s| s.trim().to_string())
            } else {
                None
            }
        })
}

fn get_python_version() -> Option<String> {
    for cmd in &["python3", "python"] {
        if let Ok(output) = Command::new(cmd).arg("--version").output() {
            if output.status.success() {
                if let Ok(version) = String::from_utf8(output.stdout) {
                    return Some(version.trim().to_string());
                }
            }
        }
    }
    None
}

fn command_in_path(command: &str) -> bool {
    Command::new("which")
        .arg(command)
        .output()
        .map_or(false, |output| output.status.success())
}

fn is_executable(path: &Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        path.metadata()
            .map_or(false, |metadata| metadata.permissions().mode() & 0o111 != 0)
    }
    #[cfg(not(unix))]
    {
        // On Windows, assume executability based on extension
        path.extension()
            .and_then(|ext| ext.to_str())
            .map_or(false, |ext| {
                matches!(ext.to_lowercase().as_str(), "exe" | "bat" | "cmd" | "com")
            })
    }
}

fn is_writable(path: &Path) -> bool {
    if path.exists() {
        std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(path)
            .is_ok()
    } else {
        // Check if parent directory is writable
        path.parent().map_or(false, |parent| {
            std::fs::metadata(parent).map_or(false, |metadata| !metadata.permissions().readonly())
        })
    }
}

/// Validate configuration with options
pub async fn validate_config(
    deep: bool,
    requirements: bool,
    server: Option<String>,
    profile: Option<String>,
) -> Result<()> {
    let config = crate::config::Config::load(profile.as_deref()).await?;

    println!("🔍 Validating configuration...");

    if let Some(server_name) = server {
        // Validate specific server
        if let Some(server) = config.get_server(&server_name) {
            let result = validate_server(&server_name, server, deep, requirements).await;
            match result.status {
                ValidationStatus::Valid => println!("✅ Server '{}' is valid", server_name),
                ValidationStatus::Warning => {
                    println!("⚠️  Server '{}' has warnings", server_name);
                    for issue in &result.issues {
                        println!("    {}", issue.message);
                    }
                }
                ValidationStatus::Error | ValidationStatus::RequirementsMissing => {
                    println!("❌ Server '{}' has errors", server_name);
                    for issue in &result.issues {
                        println!("    {}", issue.message);
                    }
                    return Err(anyhow::anyhow!(
                        "Validation failed for server '{}'",
                        server_name
                    ));
                }
            }
        } else {
            return Err(anyhow::anyhow!("Server '{}' not found", server_name));
        }
    } else {
        // Validate all servers
        let servers = config.list_servers();
        if servers.is_empty() {
            println!("⚠️  No servers configured to validate");
            return Ok(());
        }

        let mut has_errors = false;
        for (name, server) in servers {
            let result = validate_server(&name, &server, deep, requirements).await;
            match result.status {
                ValidationStatus::Valid => println!("✅ Server '{}' is valid", name),
                ValidationStatus::Warning => {
                    println!("⚠️  Server '{}' has warnings", name);
                    for issue in &result.issues {
                        println!("    {}", issue.message);
                    }
                }
                ValidationStatus::Error | ValidationStatus::RequirementsMissing => {
                    println!("❌ Server '{}' has errors", name);
                    for issue in &result.issues {
                        println!("    {}", issue.message);
                    }
                    has_errors = true;
                }
            }
        }

        if has_errors {
            return Err(anyhow::anyhow!("Validation failed for one or more servers"));
        }
    }

    println!("✅ Configuration validation completed");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_validation_status_color() {
        assert_eq!(ValidationStatus::Valid.color(), colored::Color::Green);
        assert_eq!(ValidationStatus::Error.color(), colored::Color::Red);
    }

    #[test]
    fn test_command_validation() {
        let server = McpServer {
            command: "nonexistent-command-12345".to_string(),
            args: vec![],
            env: None,
            other: HashMap::new(),
        };

        let mut result = ValidationResult {
            server_name: "test".to_string(),
            status: ValidationStatus::Valid,
            issues: Vec::new(),
            suggestions: Vec::new(),
            requirements_checked: false,
        };

        validate_command_exists(&server, &mut result);
        assert!(!result.issues.is_empty());
        assert!(matches!(result.issues[0].severity, ValidationStatus::Error));
    }

    #[test]
    fn test_argument_validation() {
        let server = McpServer {
            command: "test".to_string(),
            args: vec!["file with spaces".to_string()],
            env: None,
            other: HashMap::new(),
        };

        let mut result = ValidationResult {
            server_name: "test".to_string(),
            status: ValidationStatus::Valid,
            issues: Vec::new(),
            suggestions: Vec::new(),
            requirements_checked: false,
        };

        validate_arguments(&server, &mut result);
        assert!(!result.issues.is_empty());
        assert!(matches!(
            result.issues[0].severity,
            ValidationStatus::Warning
        ));
    }
}
