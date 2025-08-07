#!/usr/bin/env bash

# Test with Coverage Script
# Collects code coverage using either cargo-tarpaulin or cargo-llvm-cov
# Usage: ./scripts/test-with-coverage.sh [--html] [--lcov] [--json] [--threshold N]

set -euo pipefail

# Color output for better readability
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Default values
OUTPUT_FORMAT="terminal"
COVERAGE_THRESHOLD=70
COVERAGE_TOOL=""
OUTPUT_DIR="target/coverage"
EXCLUDE_PATTERNS="*/tests/*,*/benches/*,*/examples/*,*/build.rs"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --html)
            OUTPUT_FORMAT="html"
            shift
            ;;
        --lcov)
            OUTPUT_FORMAT="lcov"
            shift
            ;;
        --json)
            OUTPUT_FORMAT="json"
            shift
            ;;
        --threshold)
            COVERAGE_THRESHOLD="$2"
            shift 2
            ;;
        --output-dir)
            OUTPUT_DIR="$2"
            shift 2
            ;;
        --exclude)
            EXCLUDE_PATTERNS="$2"
            shift 2
            ;;
        --tool)
            COVERAGE_TOOL="$2"
            shift 2
            ;;
        --help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --html              Generate HTML coverage report"
            echo "  --lcov              Generate LCOV coverage report"
            echo "  --json              Generate JSON coverage report"
            echo "  --threshold N       Set coverage threshold (default: 70)"
            echo "  --output-dir DIR    Set output directory (default: target/coverage)"
            echo "  --exclude PATTERNS  Comma-separated exclude patterns"
            echo "  --tool TOOL         Force specific tool (tarpaulin or llvm-cov)"
            echo "  --help              Show this help message"
            exit 0
            ;;
        *)
            echo -e "${RED}Error: Unknown option $1${NC}"
            exit 1
            ;;
    esac
done

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Detect available coverage tool
detect_coverage_tool() {
    if [ -n "$COVERAGE_TOOL" ]; then
        if ! command_exists "cargo-$COVERAGE_TOOL"; then
            echo -e "${RED}Error: Specified tool cargo-$COVERAGE_TOOL not found${NC}"
            exit 1
        fi
        echo "$COVERAGE_TOOL"
    elif command_exists cargo-llvm-cov; then
        echo "llvm-cov"
    elif command_exists cargo-tarpaulin; then
        echo "tarpaulin"
    else
        echo -e "${YELLOW}No coverage tool found. Installing cargo-llvm-cov...${NC}"
        cargo install cargo-llvm-cov
        echo "llvm-cov"
    fi
}

# Run coverage with cargo-llvm-cov
run_llvm_cov() {
    local args="--workspace --all-features"

    # Add exclude patterns
    IFS=',' read -ra EXCLUDES <<< "$EXCLUDE_PATTERNS"
    for pattern in "${EXCLUDES[@]}"; do
        args="$args --ignore-filename-regex '$pattern'"
    done

    # Set output format
    case $OUTPUT_FORMAT in
        html)
            args="$args --html --output-dir $OUTPUT_DIR"
            ;;
        lcov)
            args="$args --lcov --output-path $OUTPUT_DIR/lcov.info"
            ;;
        json)
            args="$args --json --output-path $OUTPUT_DIR/coverage.json"
            ;;
        terminal)
            args="$args"
            ;;
    esac

    echo -e "${GREEN}Running tests with cargo-llvm-cov...${NC}"
    eval "cargo llvm-cov $args"

    # Get coverage percentage for threshold check
    local coverage_json=$(cargo llvm-cov report --json 2>/dev/null)
    local coverage_percent=$(echo "$coverage_json" | grep -o '"percent":[0-9.]*' | head -1 | cut -d: -f2)

    check_threshold "$coverage_percent"
}

# Run coverage with cargo-tarpaulin
run_tarpaulin() {
    local args="--workspace --all-features --timeout 300"

    # Add exclude patterns
    args="$args --exclude-files '$EXCLUDE_PATTERNS'"

    # Set output format
    case $OUTPUT_FORMAT in
        html)
            args="$args --out Html --output-dir $OUTPUT_DIR"
            ;;
        lcov)
            args="$args --out Lcov --output-dir $OUTPUT_DIR"
            ;;
        json)
            args="$args --out Json --output-dir $OUTPUT_DIR"
            ;;
        terminal)
            args="$args --print-summary"
            ;;
    esac

    echo -e "${GREEN}Running tests with cargo-tarpaulin...${NC}"
    eval "cargo tarpaulin $args"

    # Get coverage percentage for threshold check
    local coverage_output=$(cargo tarpaulin --print-summary 2>&1 | tail -1)
    local coverage_percent=$(echo "$coverage_output" | grep -o '[0-9.]*%' | sed 's/%//')

    check_threshold "$coverage_percent"
}

# Check coverage against threshold
check_threshold() {
    local coverage=$1

    if [ -z "$coverage" ]; then
        echo -e "${YELLOW}Warning: Could not determine coverage percentage${NC}"
        return
    fi

    echo ""
    echo -e "Coverage: ${GREEN}${coverage}%${NC}"
    echo -e "Threshold: ${YELLOW}${COVERAGE_THRESHOLD}%${NC}"

    if (( $(echo "$coverage < $COVERAGE_THRESHOLD" | bc -l) )); then
        echo -e "${RED}Error: Coverage ${coverage}% is below threshold ${COVERAGE_THRESHOLD}%${NC}"
        exit 1
    else
        echo -e "${GREEN}✓ Coverage meets threshold${NC}"
    fi
}

# Generate coverage badge (if jq is available)
generate_badge() {
    if ! command_exists jq; then
        return
    fi

    local coverage_file="$OUTPUT_DIR/coverage.json"
    if [ ! -f "$coverage_file" ]; then
        return
    fi

    local coverage=$(jq '.percent' "$coverage_file" 2>/dev/null)
    if [ -z "$coverage" ]; then
        return
    fi

    local color
    if (( $(echo "$coverage >= 90" | bc -l) )); then
        color="brightgreen"
    elif (( $(echo "$coverage >= 70" | bc -l) )); then
        color="green"
    elif (( $(echo "$coverage >= 50" | bc -l) )); then
        color="yellow"
    else
        color="red"
    fi

    cat > "$OUTPUT_DIR/coverage-badge.json" <<EOF
{
    "schemaVersion": 1,
    "label": "coverage",
    "message": "${coverage}%",
    "color": "$color"
}
EOF

    echo -e "${GREEN}Coverage badge generated at $OUTPUT_DIR/coverage-badge.json${NC}"
}

# Main execution
main() {
    echo -e "${GREEN}=== Balatro-RS Coverage Report ===${NC}"
    echo ""

    # Create output directory
    mkdir -p "$OUTPUT_DIR"

    # Detect and use coverage tool
    COVERAGE_TOOL=$(detect_coverage_tool)
    echo -e "Using coverage tool: ${GREEN}$COVERAGE_TOOL${NC}"
    echo ""

    # Clean previous coverage data
    echo "Cleaning previous coverage data..."
    cargo clean --doc
    rm -rf "$OUTPUT_DIR"/*

    # Run coverage based on detected tool
    case $COVERAGE_TOOL in
        llvm-cov)
            run_llvm_cov
            ;;
        tarpaulin)
            run_tarpaulin
            ;;
        *)
            echo -e "${RED}Error: Unknown coverage tool $COVERAGE_TOOL${NC}"
            exit 1
            ;;
    esac

    # Generate badge if JSON output
    if [ "$OUTPUT_FORMAT" = "json" ]; then
        generate_badge
    fi

    # Print report location
    echo ""
    echo -e "${GREEN}=== Coverage Report Complete ===${NC}"

    case $OUTPUT_FORMAT in
        html)
            echo -e "HTML report: ${YELLOW}$OUTPUT_DIR/index.html${NC}"
            if command_exists xdg-open; then
                echo "Opening report in browser..."
                xdg-open "$OUTPUT_DIR/index.html" 2>/dev/null || true
            fi
            ;;
        lcov)
            echo -e "LCOV report: ${YELLOW}$OUTPUT_DIR/lcov.info${NC}"
            ;;
        json)
            echo -e "JSON report: ${YELLOW}$OUTPUT_DIR/coverage.json${NC}"
            ;;
    esac
}

# Handle CI environment
if [ "${CI:-false}" = "true" ]; then
    echo "Running in CI environment"
    # CI-specific settings
    export RUST_BACKTRACE=1
    export CARGO_TERM_COLOR=always

    # Use JSON output for CI parsing
    if [ "$OUTPUT_FORMAT" = "terminal" ]; then
        OUTPUT_FORMAT="json"
    fi
fi

# Run main function
main
