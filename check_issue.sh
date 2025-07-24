#!/bin/bash

# Script to check GitHub issue details with formatted output

if [ $# -eq 0 ]; then
    echo "Usage: $0 <issue_number> [issue_number2 ...]"
    echo "Example: $0 341 342 343"
    exit 1
fi

for issue in "$@"; do
    echo "=== Issue #$issue ==="
    gh issue view "$issue" --json title,state,body,labels 2>/dev/null | jq -r '
        "Title: " + .title + 
        "\nState: " + .state +
        "\nLabels: " + (.labels | map(.name) | join(", ")) +
        "\n\nDescription:\n" + (.body | split("\n") | .[0:10] | join("\n"))
    ' || echo "Error: Could not fetch issue #$issue"
    echo ""
done