#!/bin/bash
# setup-precommit.sh - Professional pre-commit hook setup script
# Following Uncle Bob's principle: "Automation is professionalism"

set -euo pipefail

echo "🔧 Setting up pre-commit hooks for balatro-rs"
echo "================================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're in the right directory
if [[ ! -f "Cargo.toml" ]] || [[ ! -f ".pre-commit-config.yaml" ]]; then
    print_error "This script must be run from the balatro-rs root directory"
    print_error "Make sure you have both Cargo.toml and .pre-commit-config.yaml"
    exit 1
fi

print_status "Checking system requirements..."

# Check for Python 3
if ! command -v python3 &> /dev/null; then
    print_error "Python 3 is required but not installed"
    exit 1
fi
print_success "Python 3 found: $(python3 --version)"

# Check for Rust and Cargo
if ! command -v cargo &> /dev/null; then
    print_error "Rust/Cargo is required but not installed"
    print_error "Install from: https://rustup.rs/"
    exit 1
fi
print_success "Cargo found: $(cargo --version)"

# Check for Git
if ! command -v git &> /dev/null; then
    print_error "Git is required but not installed"
    exit 1
fi
print_success "Git found: $(git --version)"

print_status "Installing pre-commit framework..."

# Install pre-commit using pip
if ! command -v pre-commit &> /dev/null; then
    print_status "Installing pre-commit..."
    if command -v pip3 &> /dev/null; then
        pip3 install --user pre-commit
    elif command -v pip &> /dev/null; then
        pip install --user pre-commit
    else
        print_error "Neither pip nor pip3 found. Please install pip first."
        exit 1
    fi

    # Add user bin to PATH if not already there
    if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
        print_warning "Adding ~/.local/bin to PATH"
        export PATH="$HOME/.local/bin:$PATH"
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
    fi
else
    print_success "pre-commit already installed: $(pre-commit --version)"
fi

print_status "Installing recommended Rust security tools..."

# Install cargo-audit for security scanning
if ! command -v cargo-audit &> /dev/null; then
    print_status "Installing cargo-audit for security vulnerability scanning..."
    cargo install cargo-audit
    print_success "cargo-audit installed"
else
    print_success "cargo-audit already installed"
fi

# Install cargo-deny for dependency policy checking
if ! command -v cargo-deny &> /dev/null; then
    print_status "Installing cargo-deny for dependency policy checking..."
    cargo install cargo-deny
    print_success "cargo-deny installed"
else
    print_success "cargo-deny already installed"
fi

print_status "Installing pre-commit hooks..."

# Install the pre-commit hooks
pre-commit install

print_success "Pre-commit hooks installed successfully!"

print_status "Testing pre-commit configuration..."

# Run pre-commit on all files to validate setup
print_status "Running pre-commit checks on all files (this may take a moment)..."
if pre-commit run --all-files; then
    print_success "All pre-commit checks passed!"
else
    print_warning "Some pre-commit checks failed. This is normal for the first run."
    print_warning "Run 'pre-commit run --all-files' again after fixing any issues."
fi

echo ""
echo "================================================"
print_success "Pre-commit setup complete!"
echo ""
echo "📋 What happens now:"
echo "  • Pre-commit hooks will run automatically before each commit"
echo "  • Commits will be blocked if formatting or linting issues are found"
echo "  • Run 'cargo fmt' to fix formatting issues"
echo "  • Run 'cargo clippy --fix' to fix some linting issues"
echo ""
echo "🔧 Useful commands:"
echo "  • pre-commit run --all-files  # Run all hooks on all files"
echo "  • pre-commit run <hook-name>  # Run specific hook"
echo "  • pre-commit autoupdate       # Update hook versions"
echo "  • pre-commit uninstall        # Remove hooks (if needed)"
echo ""
echo "📚 Manual checks (run when needed):"
echo "  • pre-commit run --hook-stage manual cargo-doc"
echo "  • pre-commit run --hook-stage manual missing-docs-check"
echo ""
print_success "Happy coding! 🦀"
