# Pre-Commit Hooks Guide

This document outlines the pre-commit hook system for balatro-rs, designed to enforce code quality standards and prevent technical debt accumulation.

## Overview

Pre-commit hooks automatically run quality checks before each commit, catching issues early and maintaining consistent code standards across the project. This system mirrors our CI pipeline, ensuring that code quality checks happen locally before pushing to remote repositories.

## Philosophy

Following Uncle Bob's clean code principles:
- **Automation is professionalism**: Manual quality checks are prone to human error
- **Fail fast**: Catch issues at commit time, not in CI
- **Boy Scout Rule**: Leave the code cleaner than you found it
- **No broken windows**: Never allow poorly formatted or linting-failed code to be committed

## Quick Start

### 1. One-Time Setup

Run the automated setup script:

```bash
./scripts/setup-precommit.sh
```

This script will:
- Install the pre-commit framework
- Install security tools (cargo-audit, cargo-deny)
- Install the git hooks
- Run an initial validation

### 2. Verify Installation

Check that hooks are working:

```bash
pre-commit run --all-files
```

All checks should pass. If not, fix the issues and run again.

## What Gets Checked

### Formatting Checks
- **Trailing Whitespace**: Removes trailing whitespace from all files
- **File Endings**: Ensures files end with a newline
- **Rust Formatting**: `cargo fmt --check` for consistent code formatting

### Code Quality Checks
- **Clippy Linting**: `cargo clippy` with warnings treated as errors
- **Compilation**: `cargo check` to ensure code compiles
- **Fast Tests**: Subset of test suite (`cargo test --lib --bins`)

### File Validation
- **YAML Syntax**: Validates YAML files (workflows, configs)
- **TOML Syntax**: Validates TOML files (Cargo.toml, etc.)
- **JSON Syntax**: Validates JSON files
- **Merge Conflicts**: Prevents commits with merge conflict markers
- **Large Files**: Prevents accidentally committing large files (>500KB)

### Security Checks (Optional)
- **Cargo Audit**: Scans for security vulnerabilities in dependencies
- **Cargo Deny**: Checks dependency licenses and policies

### Manual Checks (Run When Needed)
- **Documentation**: `cargo doc` to check documentation builds
- **Missing Docs**: Warns about missing documentation on public items

```bash
# Run manual checks
pre-commit run --hook-stage manual cargo-doc
pre-commit run --hook-stage manual missing-docs-check
```

## Workflow Integration

### Normal Development Flow

1. Make your changes
2. Run `git add <files>` to stage changes
3. Run `git commit -m "Your message"`
4. Pre-commit hooks run automatically
5. If hooks pass: commit succeeds
6. If hooks fail: commit is blocked, fix issues and try again

### Fixing Common Issues

#### Formatting Issues
```bash
# Fix formatting automatically
cargo fmt --all

# Commit again
git commit -m "Your message"
```

#### Clippy Warnings
```bash
# Show clippy issues
cargo clippy --all-targets --all-features

# Fix automatically where possible
cargo clippy --fix --all-targets --all-features

# Fix remaining issues manually, then commit
git commit -m "Your message"
```

#### Test Failures
```bash
# Run full test suite to see failures
cargo test

# Fix failing tests, then commit
git commit -m "Your message"
```

## Advanced Usage

### Skip Hooks (Use Sparingly)

**Warning**: Only use this in emergencies. It bypasses all quality checks.

```bash
git commit --no-verify -m "Emergency commit"
```

### Run Specific Hooks

```bash
# Run just formatting check
pre-commit run cargo-fmt

# Run just clippy
pre-commit run cargo-clippy

# Run all hooks on specific files
pre-commit run --files src/lib.rs
```

### Update Hook Versions

```bash
# Update to latest hook versions
pre-commit autoupdate

# Review and commit the updated .pre-commit-config.yaml
git add .pre-commit-config.yaml
git commit -m "Update pre-commit hook versions"
```

## Configuration

### Main Configuration File

`.pre-commit-config.yaml` contains all hook definitions. Key sections:

- **Standard Hooks**: File formatting, syntax validation
- **Rust Hooks**: cargo fmt, clippy, check, test
- **Security Hooks**: audit, deny (optional tools)
- **Documentation Hooks**: doc building, missing docs warnings

### Customizing Checks

To modify which checks run, edit `.pre-commit-config.yaml`:

```yaml
# Disable a hook by commenting it out
# - id: cargo-audit

# Add arguments to a hook
- id: cargo-clippy
  args: [--all-targets, --all-features, --, -D, warnings, -D, clippy::unwrap_used]
```

## Troubleshooting

### Pre-commit Not Found

If `pre-commit` command isn't found:

```bash
# Install using pip
pip3 install --user pre-commit

# Add to PATH if needed
export PATH="$HOME/.local/bin:$PATH"
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
```

### Hooks Not Running

```bash
# Reinstall hooks
pre-commit uninstall
pre-commit install
```

### Slow Hook Performance

Some hooks may be slow on first run due to compilation. Subsequent runs are faster due to caching.

To skip slow checks during development:

```bash
# Skip tests during rapid development
SKIP=cargo-test-fast git commit -m "WIP: rapid development"

# Run full checks before pushing
pre-commit run --all-files
```

### Security Tools Missing

If cargo-audit or cargo-deny aren't installed:

```bash
# Install security tools
cargo install cargo-audit cargo-deny

# Or run setup script again
./scripts/setup-precommit.sh
```

## Integration with CI

The pre-commit hooks mirror the CI pipeline checks:

| Pre-commit Hook | CI Job | Purpose |
|-----------------|--------|---------|
| `cargo-fmt` | `fmt` | Code formatting |
| `cargo-clippy` | `clippy` | Linting |
| `cargo-check` | `build-all` | Compilation |
| `cargo-test-fast` | `test` | Unit tests |
| `cargo-audit` | `security-scan` | Security vulnerabilities |
| `cargo-deny` | `security-scan` | Dependency policies |

This ensures that pre-commit catches the same issues that would fail in CI, providing faster feedback.

## Best Practices

### For Developers

1. **Run hooks before pushing**: Even though hooks run on commit, run manually before pushing large changes
2. **Fix issues immediately**: Don't accumulate formatting or linting debt
3. **Keep commits atomic**: Small, focused commits are easier to fix when hooks fail
4. **Update tools regularly**: Keep pre-commit and security tools updated

### For Maintainers

1. **Monitor hook performance**: If hooks become too slow, optimize or move to manual stage
2. **Keep configs synchronized**: Ensure pre-commit hooks match CI requirements
3. **Update hook versions**: Regularly run `pre-commit autoupdate`
4. **Document exceptions**: If skipping hooks is necessary, document why

## Performance Considerations

### Hook Timing (Typical)

- **File checks**: <1 second
- **Cargo fmt**: 1-2 seconds
- **Cargo clippy**: 5-15 seconds (first run), 2-5 seconds (subsequent)
- **Cargo check**: 3-10 seconds (cached), 30+ seconds (clean)
- **Fast tests**: 5-30 seconds depending on changes

### Optimization Tips

1. **Keep changes small**: Smaller commits = faster hooks
2. **Use incremental builds**: Don't `cargo clean` unnecessarily
3. **Cache dependencies**: Let Cargo cache work effectively
4. **Skip slow hooks during WIP**: Use `SKIP` environment variable for rapid iteration

## Testing the Pre-Commit System

The project includes comprehensive tests for the pre-commit system:

```bash
# Run pre-commit integration tests
cargo test precommit -p balatro-rs

# Test individual components
cargo test test_precommit_config_exists_and_valid -p balatro-rs
cargo test test_cargo_fmt_check -p balatro-rs
cargo test test_cargo_clippy_check -p balatro-rs
```

These tests validate:
- Configuration file exists and is valid
- All required hooks are present
- CI and pre-commit configurations are consistent
- Individual tools (fmt, clippy, etc.) work correctly
- Performance is within acceptable bounds

## Security Considerations

### Audit and Deny Tools

The security hooks are optional but recommended:

- **cargo-audit**: Scans for known security vulnerabilities in dependencies
- **cargo-deny**: Checks dependency licenses and prevents banned crates

### Safe Defaults

- Hooks run in isolated environments
- No network access during hook execution
- Configuration is version-controlled
- All tools are standard Rust ecosystem tools

## Maintenance

### Regular Tasks

1. **Weekly**: Check for pre-commit updates
2. **Monthly**: Review hook performance metrics
3. **Per release**: Validate all hooks still work
4. **As needed**: Add new hooks for new code quality requirements

### Version Updates

```bash
# Update pre-commit itself
pip3 install --upgrade pre-commit

# Update hook versions
pre-commit autoupdate

# Test updated configuration
pre-commit run --all-files
```

## References

- [Pre-commit Documentation](https://pre-commit.com/)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)
- [Clean Code by Robert C. Martin](https://www.oreilly.com/library/view/clean-code-a/9780136083238/)
- [The Pragmatic Programmer](https://pragprog.com/titles/tpp20/the-pragmatic-programmer-20th-anniversary-edition/)

---

*Remember: Pre-commit hooks are your first line of defense against technical debt. Embrace them as tools that make you a more professional developer.*
