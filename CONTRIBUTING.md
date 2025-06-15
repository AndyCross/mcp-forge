# Contributing to MCP-Forge

Thank you for your interest in contributing to MCP-Forge! This guide will help you get started with contributing to this project.

## 🚀 Quick Start

1. **Fork the repository** on GitHub
2. **Clone your fork** locally
3. **Create a feature branch** from `main`
4. **Make your changes** with tests
5. **Submit a pull request**

## 📋 Development Setup

### Prerequisites

- **Rust 1.70+** (2021 edition)
- **Git** for version control
- **Claude Desktop** for testing (optional but recommended)

### Local Development

```bash
# Clone your fork
git clone https://github.com/your-username/mcp-forge.git
cd mcp-forge

# Build the project
cargo build

# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run -- --help

# Check formatting
cargo fmt --check

# Run linter
cargo clippy -- -D warnings
```

## 🎯 Types of Contributions

### 🐛 Bug Reports

When reporting bugs, please include:

- **Clear description** of the issue
- **Steps to reproduce** the problem
- **Expected vs actual behavior**
- **Environment details** (OS, Rust version, etc.)
- **Relevant logs** with `RUST_LOG=debug`

### ✨ Feature Requests

For new features, please:

- **Check existing issues** to avoid duplicates
- **Describe the use case** and motivation
- **Provide examples** of how it would work
- **Consider backwards compatibility**

### 🔧 Code Contributions

We welcome:

- **Bug fixes**
- **New features**
- **Performance improvements**
- **Documentation improvements**
- **Test coverage improvements**

### 📚 Documentation

Help improve:

- **README.md** - Keep it current with features
- **Command help text** - Ensure all commands have good help
- **Code comments** - Explain complex logic
- **Examples** - Real-world usage scenarios

### 🎨 Templates

Contribute new MCP server templates:

- **Follow template schema** in `schemas/template-schema.json`
- **Include comprehensive documentation**
- **Test on multiple platforms**
- **Provide setup instructions**

## 📝 Development Guidelines

### Code Style

We follow standard Rust conventions:

```bash
# Format code
cargo fmt

# Check linting
cargo clippy -- -D warnings

# Run all checks
cargo test && cargo fmt --check && cargo clippy -- -D warnings
```

### Commit Messages

Use conventional commit format:

```
type(scope): description

[optional body]

[optional footer]
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Test additions/changes
- `chore`: Maintenance tasks

**Examples:**
```
feat(cli): add bulk operations support
fix(config): resolve profile switching issue
docs(readme): update installation instructions
```

### Branch Naming

Use descriptive branch names:

```
feature/bulk-operations
fix/profile-switching-bug
docs/update-readme
refactor/config-management
```

### Testing

- **Write tests** for new functionality
- **Update existing tests** when changing behavior
- **Ensure all tests pass** before submitting PR
- **Add integration tests** for complex features

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run tests with debug logging
RUST_LOG=debug cargo test
```

## 🏗️ Project Structure

```
mcp-forge/
├── src/
│   ├── main.rs           # Entry point
│   ├── cli.rs            # CLI command handlers
│   ├── config.rs         # Configuration management
│   ├── templates.rs      # Template system
│   ├── github.rs         # GitHub integration
│   ├── utils.rs          # Utility functions
│   ├── search.rs         # Search and filtering
│   ├── validation.rs     # Configuration validation
│   ├── backup.rs         # Backup and restore
│   ├── bulk.rs           # Bulk operations
│   └── profiles.rs       # Profile management
├── templates/            # Built-in templates
├── schemas/              # JSON schemas
├── tests/                # Integration tests
└── docs/                 # Documentation
```

### Module Responsibilities

- **`main.rs`**: CLI parsing and command routing
- **`cli.rs`**: Command implementations and user interaction
- **`config.rs`**: Configuration file management
- **`templates.rs`**: Template loading and processing
- **`github.rs`**: GitHub API integration
- **`utils.rs`**: Shared utility functions
- **`search.rs`**: Advanced filtering and search
- **`validation.rs`**: Configuration validation
- **`backup.rs`**: Backup and restore operations
- **`bulk.rs`**: Bulk operation management
- **`profiles.rs`**: Profile management

## 🧪 Testing Strategy

### Unit Tests

Test individual functions and modules:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        // Test implementation
        assert_eq!(expected, actual);
    }

    #[tokio::test]
    async fn test_async_function() {
        // Async test implementation
    }
}
```

### Integration Tests

Test complete workflows in `tests/` directory:

```rust
// tests/integration_test.rs
use mcp_forge::*;

#[tokio::test]
async fn test_complete_workflow() {
    // Test end-to-end functionality
}
```

### Manual Testing

Test with real Claude Desktop configurations:

```bash
# Test with temporary config
export CLAUDE_CONFIG_PATH="/tmp/test_config.json"
cargo run -- list

# Test profile functionality
cargo run -- profile create test
cargo run -- --profile test list
```

## 📋 Pull Request Process

### Before Submitting

1. **Ensure tests pass**: `cargo test`
2. **Check formatting**: `cargo fmt --check`
3. **Run linter**: `cargo clippy -- -D warnings`
4. **Update documentation** if needed
5. **Add tests** for new functionality

### PR Description Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Manual testing completed

## Checklist
- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] Tests pass locally
```

### Review Process

1. **Automated checks** must pass
2. **Code review** by maintainers
3. **Testing** on multiple platforms
4. **Documentation review**
5. **Merge** after approval

## 🎨 Template Contributions

### Template Guidelines

Templates should:

- **Follow the schema** in `schemas/template-schema.json`
- **Include comprehensive documentation**
- **Work cross-platform** (Windows, macOS, Linux)
- **Use semantic versioning**
- **Include setup instructions**

### Template Structure

```json
{
  "name": "template-name",
  "version": "1.0.0",
  "description": "Clear description of what this template does",
  "author": "Your Name",
  "tags": ["category", "type"],
  "platforms": ["windows", "macos", "linux"],
  "variables": {
    "variable_name": {
      "type": "string",
      "description": "Clear description",
      "required": true,
      "default": "optional_default"
    }
  },
  "config": {
    "command": "command_to_run",
    "args": ["--arg", "{{variable_name}}"]
  },
  "requirements": {
    "nodejs": ">=18.0.0"
  },
  "setup_instructions": "Step-by-step setup instructions"
}
```

### Template Testing

```bash
# Validate template
cargo run -- template validate templates/my-template.json

# Test template usage
cargo run -- add test-server my-template --vars "var=value"

# Test on multiple platforms
# (Windows, macOS, Linux)
```

## 🚀 Release Process

### Version Numbering

We use [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking changes
- **MINOR**: New features (backwards compatible)
- **PATCH**: Bug fixes (backwards compatible)

### Release Checklist

- [ ] Update version in `Cargo.toml`
- [ ] Update `CHANGELOG.md`
- [ ] Run full test suite
- [ ] Update documentation
- [ ] Create release PR
- [ ] Tag release after merge
- [ ] Publish to crates.io (when ready)

## 🤝 Community Guidelines

### Code of Conduct

- **Be respectful** and inclusive
- **Help others** learn and contribute
- **Give constructive feedback**
- **Focus on the code**, not the person

### Communication

- **GitHub Issues** for bugs and feature requests
- **GitHub Discussions** for questions and ideas
- **Pull Requests** for code contributions
- **Clear communication** in all interactions

## 📚 Resources

### Documentation

- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Claude Desktop MCP Documentation](https://claude.ai/docs)
- [JSON Schema](https://json-schema.org/)

### Tools

- [rustfmt](https://github.com/rust-lang/rustfmt) - Code formatting
- [clippy](https://github.com/rust-lang/rust-clippy) - Linting
- [cargo-watch](https://github.com/watchexec/cargo-watch) - Auto-rebuild
- [cargo-edit](https://github.com/killercup/cargo-edit) - Dependency management

## ❓ Getting Help

If you need help:

1. **Check existing issues** and documentation
2. **Search discussions** for similar questions
3. **Create a new issue** with detailed information
4. **Join discussions** for broader questions

## 🙏 Recognition

Contributors are recognized in:

- **README.md** acknowledgments
- **Release notes** for significant contributions
- **GitHub contributors** page

Thank you for contributing to MCP-Forge! 🚀 