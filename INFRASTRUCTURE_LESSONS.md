# Lessons Learned - Issue #3

**Scope**: Infrastructure/Dependency Management
**Date**: 2025-07-14
**Context**: Dependency analysis, security audit, and documentation for balatro-rs workspace

## What Worked Well

- **TDD Approach**: Writing tests first for dependency documentation ensured comprehensive coverage and clear requirements
- **Parallel Task Execution**: Using the Task tool to update pyo3 and add anyhow simultaneously saved time
- **Structured Documentation**: Creating DEPENDENCY_JUSTIFICATION.md with clear sections made requirements explicit
- **Work Tree Development**: Using git work trees kept the main branch clean and enabled isolated development

## Pitfalls to Avoid

- **Python Library Dependencies**: The default features include pyo3 which requires Python libraries, causing test failures in some environments
- **Version String Matching**: Tests checking for specific version patterns need to handle multiple formats (e.g., `pyo3 = "0.24"` vs `version = "0.24"`)
- **Formatting Before Tests**: Always run cargo fmt before creating test files to avoid formatting issues later

## Key Insights

- **Security First**: Running `cargo audit` immediately identified the pyo3 vulnerability that needed addressing
- **ML Training Focus**: Dependencies should be evaluated for their impact on performance and memory usage during RL training
- **Dependency Justification**: Every dependency should have a clear purpose and justification documented
- **Version Pinning Strategy**: Using tilde requirements (~) provides a good balance between stability and security updates

## Recommendations for Future Work

1. **Regular Security Audits**: Add `cargo audit` to CI/CD pipeline to catch vulnerabilities early
2. **Dependency Review Process**: When adding new dependencies, always:
   - Evaluate performance impact on move generation
   - Document justification in DEPENDENCY_JUSTIFICATION.md
   - Run cargo audit after adding
3. **Test Infrastructure**: Consider adding tests that verify dependency properties (licenses, security status, etc.)
4. **Alternative to Criterion**: The criterion benchmarking library brings in unmaintained dependencies (atty, serde_cbor) - consider alternatives for future work