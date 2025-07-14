# Dependency Justification for balatro-rs

This document provides comprehensive justification for all dependencies used in the balatro-rs workspace, including security audit results and WASM compatibility analysis.

## Core Dependencies

The following dependencies are essential to the balatro-rs game engine functionality.

## Security Audit

Last audit performed: 2025-07-14

### Vulnerabilities Found:
- **pyo3 0.23.3** - RUSTSEC-2025-0020: Risk of buffer overflow in `PyString::from_object`
  - **Action**: Upgrade to 0.24.1 or higher
  - **Status**: Pending upgrade

### Warnings:
- **atty 0.2.14** - Unmaintained (used by criterion)
- **serde_cbor 0.11.2** - Unmaintained (used by criterion)


## Dependency Justifications

### rand
**Version**: ~0.8.5  
**Purpose**: Random number generation for game mechanics  
**Justification**: Essential for card shuffling, shop item generation, and all RNG-based game mechanics. Industry standard for cryptographically secure random numbers.

### thiserror
**Version**: ~1.0.61  
**Purpose**: Error handling derive macros  
**Justification**: Provides ergonomic error type definitions with automatic Display and Error trait implementations. Reduces boilerplate code and improves error message consistency.

### anyhow
**Version**: To be added  
**Purpose**: Flexible error handling for application-level code  
**Justification**: Complements thiserror by providing context-aware error handling for CLI and high-level game logic. Enables better error reporting and debugging.

### itertools
**Version**: 0.13.0  
**Purpose**: Iterator utility functions  
**Justification**: Provides essential iterator combinators like `combinations` and `permutations` used in hand evaluation and move generation logic.

### indexmap
**Version**: 2.6.0  
**Purpose**: Order-preserving hash map  
**Justification**: Required for maintaining consistent ordering of game elements (cards, jokers) while providing O(1) lookups. Critical for deterministic game state serialization.

### strum
**Version**: 0.26  
**Purpose**: Enum utilities and derive macros  
**Justification**: Enables iteration over enum variants and string conversions for game actions and stages. Reduces boilerplate for enum handling throughout the codebase.

### serde
**Version**: ~1.0.215  
**Purpose**: Serialization framework  
**Justification**: Core requirement for game state persistence, save/load functionality, and Python bindings data exchange. De facto standard for Rust serialization.

### serde_json
**Version**: ~1.0.118  
**Purpose**: JSON serialization  
**Justification**: Human-readable format for save files and debugging. Required for interfacing with Python/JavaScript frontends.

### pyo3
**Version**: 0.23.1 (upgrading to 0.24.1)  
**Purpose**: Python bindings  
**Justification**: Enables the pylatro package for Python-based reinforcement learning frameworks. Core requirement for ML/AI integration.

### colored
**Version**: 2.2.0  
**Purpose**: Terminal color output  
**Justification**: Enhances CLI usability by providing colored output for game state visualization and debugging. Optional feature for better developer experience.

### text_io
**Version**: 0.1.9  
**Purpose**: Simple text input parsing  
**Justification**: Used in CLI for parsing user input commands. Provides convenient macros for reading formatted input.

### criterion
**Version**: 0.3  
**Purpose**: Benchmarking framework  
**Justification**: Development dependency for performance testing. Essential for ensuring move generation performance meets RL training requirements.

### uuid
**Version**: ~1.9.1  
**Purpose**: Unique identifier generation  
**Justification**: Used for game instance identification and save file management. v7 UUIDs provide time-ordered identifiers.

### tracing
**Version**: ~0.1.40  
**Purpose**: Structured logging and instrumentation  
**Justification**: Optional dependency for debugging and performance analysis. Provides hierarchical, structured logging for complex game flow tracking.

## Dependency Management Policy

1. **Security First**: Address all security vulnerabilities immediately
2. **Minimal Dependencies**: Only add dependencies that provide significant value
3. **Performance Critical**: Consider impact on move generation speed for RL training
4. **Version Pinning**: Use tilde requirements (~) for controlled updates
5. **Regular Audits**: Run `cargo audit` before each release

## Future Considerations

- Consider replacing criterion with a lighter benchmark framework to eliminate unmaintained dependencies
- Evaluate alternative terminal color libraries if colored becomes unmaintained
- Monitor pyo3 for continued security updates and performance improvements