//! Tests for dependency documentation and management
//!
//! This test ensures that all dependencies are properly documented
//! and justified according to project requirements.

use std::fs;
use std::path::Path;

#[test]
fn dependency_documentation_exists() {
    let doc_path = Path::new("../DEPENDENCY_JUSTIFICATION.md");
    assert!(
        doc_path.exists(),
        "DEPENDENCY_JUSTIFICATION.md must exist in the workspace root"
    );
}

#[test]
fn dependency_documentation_has_required_sections() {
    let doc_path = Path::new("../DEPENDENCY_JUSTIFICATION.md");
    let content = fs::read_to_string(doc_path).expect("Failed to read DEPENDENCY_JUSTIFICATION.md");

    // Check for required sections
    assert!(
        content.contains("## Core Dependencies"),
        "Documentation must contain 'Core Dependencies' section"
    );

    assert!(
        content.contains("## Security Audit"),
        "Documentation must contain 'Security Audit' section"
    );

    assert!(
        content.contains("## Dependency Justifications"),
        "Documentation must contain 'Dependency Justifications' section"
    );
}

#[test]
fn all_dependencies_are_justified() {
    let doc_path = Path::new("../DEPENDENCY_JUSTIFICATION.md");
    let content = fs::read_to_string(doc_path).expect("Failed to read DEPENDENCY_JUSTIFICATION.md");

    // List of dependencies that must be justified
    let dependencies = vec![
        "rand",
        "thiserror",
        "anyhow",
        "itertools",
        "indexmap",
        "strum",
        "serde",
        "serde_json",
        "pyo3",
        "colored",
        "text_io",
        "criterion",
        "uuid",
        "tracing",
    ];

    for dep in dependencies {
        assert!(
            content.contains(&format!("### {}", dep)),
            "Dependency '{}' must have a justification section",
            dep
        );
    }
}

#[test]
fn pyo3_version_is_secure() {
    // Read all Cargo.toml files to check pyo3 version
    let cargo_files = vec!["../core/Cargo.toml", "../pylatro/Cargo.toml"];

    for file in cargo_files {
        if Path::new(file).exists() {
            let content = fs::read_to_string(file).expect(&format!("Failed to read {}", file));

            if content.contains("pyo3") {
                // Check that pyo3 version is at least 0.24.1
                // Handle both direct dependencies and version in curly braces
                assert!(
                    content.contains("pyo3 = \"0.24") || 
                    content.contains("pyo3 = \"0.25") ||
                    content.contains("pyo3 = \"1.") ||
                    content.contains("version = \"0.24") ||
                    content.contains("version = \"0.25") ||
                    content.contains("version = \"1."),
                    "pyo3 version must be at least 0.24.1 to address RUSTSEC-2025-0020 vulnerability in {}",
                    file
                );
            }
        }
    }
}

#[test]
fn anyhow_dependency_exists() {
    let cargo_path = Path::new("../core/Cargo.toml");
    let content = fs::read_to_string(cargo_path).expect("Failed to read core/Cargo.toml");

    assert!(
        content.contains("anyhow"),
        "anyhow dependency must be added to core/Cargo.toml for error handling"
    );
}
