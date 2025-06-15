# Pull Request

## Description

Brief description of the changes in this PR.

## Type of Change

Please check the type of change your PR introduces:

- [ ] 🐛 Bug fix (non-breaking change which fixes an issue)
- [ ] ✨ New feature (non-breaking change which adds functionality)
- [ ] 💥 Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] 📚 Documentation update
- [ ] 🎨 Code style/formatting changes
- [ ] ♻️ Code refactoring (no functional changes)
- [ ] ⚡ Performance improvements
- [ ] 🧪 Test additions or updates
- [ ] 🔧 Build/CI changes
- [ ] 📝 Template contribution

## Changes Made

### Added
- List new features or functionality

### Changed
- List modifications to existing features

### Fixed
- List bug fixes

### Removed
- List deprecated or removed features

## Testing

### Test Environment
- [ ] Tested on macOS
- [ ] Tested on Windows  
- [ ] Tested on Linux

### Test Cases
- [ ] Unit tests pass (`cargo test`)
- [ ] Integration tests pass
- [ ] Manual testing completed
- [ ] Performance impact assessed

### Test Commands
```bash
# Commands used to test the changes
cargo test
cargo build --release
./target/release/mcp-forge --help
```

## Documentation

- [ ] Code is self-documenting with clear variable names and comments
- [ ] Public API changes are documented
- [ ] README.md updated (if applicable)
- [ ] Command help text updated (if applicable)
- [ ] Examples updated (if applicable)

## Checklist

### Code Quality
- [ ] Code follows the project's style guidelines
- [ ] Self-review of code completed
- [ ] Code is properly formatted (`cargo fmt`)
- [ ] No linting errors (`cargo clippy`)
- [ ] No compiler warnings introduced

### Functionality
- [ ] Changes work as expected
- [ ] Edge cases considered and handled
- [ ] Error handling is appropriate
- [ ] Backward compatibility maintained (or breaking changes documented)

### Security
- [ ] No hardcoded secrets or sensitive information
- [ ] Input validation implemented where needed
- [ ] Security implications considered

### Performance
- [ ] Performance impact assessed
- [ ] No significant performance regressions
- [ ] Memory usage considered
- [ ] Binary size impact acceptable

## Related Issues

Closes #(issue_number)
Relates to #(issue_number)

## Screenshots/Examples

If applicable, add screenshots or command examples to help explain your changes.

```bash
# Example of new functionality
mcp-forge new-command --option value
```

## Additional Notes

Any additional information that reviewers should know about this PR.

## Reviewer Notes

### Focus Areas
Please pay special attention to:
- [ ] Specific area of concern
- [ ] Performance implications
- [ ] Security considerations
- [ ] Breaking changes

### Testing Requests
Please test:
- [ ] Specific functionality
- [ ] Edge cases
- [ ] Cross-platform compatibility 