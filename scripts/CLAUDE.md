# CLAUDE.md - Scripts Directory

## Directory Purpose

Contains automation scripts for development workflow, CI/CD preparation, and code quality enforcement. These scripts ensure consistency across development environments and maintain project standards.

## Key Scripts

### setup-precommit.sh (148 lines)
**Purpose**: Automates pre-commit hook installation and configuration

**Features**:
- Checks system requirements (Python3, Rust/Cargo, Git)
- Installs pre-commit framework via pip
- Installs security tools (cargo-audit, cargo-deny)
- Configures PATH for user binaries
- Validates setup with test run

**Usage**:
```bash
./scripts/setup-precommit.sh
```

**Professional Standards**:
- Colored output for clarity
- Comprehensive error handling
- Status reporting at each step
- Rollback on failure

### test-ci-local.sh (96 lines)
**Purpose**: Replicates CI pipeline checks locally before pushing

**Checks Performed**:
1. Format validation (`cargo fmt --check`)
2. Linting (`cargo clippy`)
3. Test execution (with Python library fallback)
4. Build verification for all workspace members
5. Benchmark compilation check

**Usage**:
```bash
./scripts/test-ci-local.sh
```

**Smart Features**:
- Detects Python library issues
- Provides fallback testing strategy
- Progressive enhancement (continues where possible)
- Clear error reporting

## Automation Philosophy

### Core Principles
- **"Automation is professionalism"** (Uncle Bob's principle)
- **Fail-fast approach** with clear error messages
- **Progressive enhancement** - continue where possible
- **Developer-friendly** output with actionable suggestions

### Script Standards
```bash
#!/bin/bash
set -euo pipefail  # Strict error handling

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function-based organization
check_requirements() {
    # Validation logic
}

# Main execution with error trapping
main() {
    trap 'echo -e "${RED}Setup failed${NC}"' ERR
    # Script logic
}
```

## Development Workflow Integration

### Pre-Commit Workflow
```
Developer makes changes
    ↓
Pre-commit hooks run automatically
    ↓
Format, lint, test locally
    ↓
Only clean code gets committed
```

### CI Preparation Workflow
```
Before pushing to remote
    ↓
Run test-ci-local.sh
    ↓
Catch CI failures locally
    ↓
Fix issues before push
    ↓
CI passes on first try
```

## Error Handling Patterns

### Requirement Checking
```bash
if ! command -v python3 &> /dev/null; then
    echo -e "${RED}Error: Python 3 is not installed${NC}"
    echo "Please install Python 3 and try again"
    exit 1
fi
```

### Progressive Enhancement
```bash
if ! cargo test -p pylatro 2>/dev/null; then
    echo -e "${YELLOW}Warning: Python library issue detected${NC}"
    echo "Falling back to core tests only..."
    cargo test -p balatro-rs
fi
```

## Security Tool Integration

### cargo-audit
- Checks for known security vulnerabilities
- Integrated into pre-commit workflow
- Database updated automatically

### cargo-deny
- Enforces dependency policies
- License compliance checking
- Duplicate dependency detection

## Best Practices

1. **Always use set -euo pipefail** for safety
2. **Provide colored output** for readability
3. **Check requirements** before operations
4. **Trap errors** for cleanup
5. **Give actionable feedback** on failures
6. **Test scripts** in clean environment

## Running Scripts

```bash
# Make executable
chmod +x scripts/*.sh

# Run setup
./scripts/setup-precommit.sh

# Test CI locally
./scripts/test-ci-local.sh
```

## Maintenance

Scripts should be:
- **Idempotent**: Safe to run multiple times
- **Portable**: Work across Unix-like systems
- **Documented**: Clear comments and output
- **Versioned**: Track changes in git
