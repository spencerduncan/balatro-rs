# PyO3 Binding Patterns and Best Practices

This document outlines the modern PyO3 patterns used in the balatro-rs Python bindings and provides guidance for future development and maintenance.

## Overview

The balatro-rs project uses PyO3 0.24.1 with modern patterns for maximum performance, safety, and maintainability. This document covers the specific patterns implemented during the PyO3 integration cleanup and modernization (Issue #504).

## Table of Contents

- [PyClass Standardization](#pyclass-standardization)
- [Deprecation Warning Patterns](#deprecation-warning-patterns)
- [Error Handling](#error-handling)
- [Module Organization](#module-organization)
- [Performance Patterns](#performance-patterns)
- [Testing Patterns](#testing-patterns)
- [Migration Guidelines](#migration-guidelines)

## PyClass Standardization

### Module Attribution

All `#[pyclass]` structs and enums should specify the target Python module for proper introspection:

```rust
// ✅ GOOD: Specify module for proper Python introspection
#[pyclass(module = "pylatro")]
struct GameEngine {
    game: Game,
}

#[cfg_attr(feature = "python", pyclass(eq, module = "pylatro"))]
#[derive(Debug, PartialEq, Eq)]
pub enum JokerId {
    Joker,
    GreedyJoker,
    // ...
}

// ❌ BAD: Missing module specification
#[pyclass]
struct GameEngine {
    game: Game,
}
```

### Consistent Attribute Patterns

For enums that need equality comparison:
```rust
#[cfg_attr(feature = "python", pyclass(eq, module = "pylatro"))]
```

For structs and enums without equality:
```rust
#[cfg_attr(feature = "python", pyclass(module = "pylatro"))]
```

This pattern ensures:
- Consistent Python module attribution
- Proper feature gating for optional Python support
- Correct equality implementation where needed

## Deprecation Warning Patterns

### Modern Deprecation in pymethods

**Modern Pattern (PyO3 0.20+):**
```rust
#[pymethods]
impl GameState {
    /// DEPRECATED: Use GameEngine.gen_actions() instead
    fn gen_actions(&self, py: Python<'_>) -> PyResult<Vec<Action>> {
        // Issue deprecation warning using modern PyO3 pattern
        let warnings = py.import("warnings")?;
        warnings.call_method1(
            "warn",
            ("GameState.gen_actions() is deprecated. Use GameEngine.gen_actions() instead.",),
        )?;
        
        // Return error indicating the method is non-functional
        Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
            "This method is deprecated and non-functional."
        ))
    }
}
```

**Legacy Pattern (Avoid):**
```rust
// ❌ DEPRECATED: Don't use Python::with_gil in pymethods
fn gen_actions(&self) -> PyResult<Vec<Action>> {
    Python::with_gil(|py| -> PyResult<()> {
        let warnings = py.import("warnings")?;
        // ... warning logic
        Ok(())
    })?;
    // ... rest of method
}
```

### Key Benefits of Modern Pattern

1. **Direct GIL Access**: Methods in `#[pymethods]` can directly accept `py: Python<'_>` parameter
2. **No Nested GIL Acquisition**: Eliminates unnecessary `Python::with_gil` calls
3. **Better Performance**: Reduces overhead of GIL re-acquisition
4. **Cleaner Code**: More straightforward control flow

## Error Handling

### Python Exception Mapping

The project uses a sophisticated error sanitization system:

```rust
#[cfg(feature = "python")]
impl std::convert::From<DeveloperGameError> for PyErr {
    fn from(err: DeveloperGameError) -> PyErr {
        use pyo3::exceptions::{PyValueError, PyRuntimeError};
        
        // Use development mode in debug builds, production mode otherwise
        let detail_level = if cfg!(debug_assertions) {
            ErrorDetailLevel::Development
        } else {
            ErrorDetailLevel::Production
        };
        
        let sanitizer = ErrorSanitizer::new(detail_level);
        let user_error = sanitizer.sanitize_game_error(&err);
        
        // Map to appropriate Python exception types
        match user_error {
            UserError::InvalidInput => PyValueError::new_err(user_error.to_string()),
            UserError::InvalidOperation => PyRuntimeError::new_err(user_error.to_string()),
            UserError::ResourceNotFound => PyRuntimeError::new_err(user_error.to_string()),
            UserError::OperationFailed => PyRuntimeError::new_err(user_error.to_string()),
            UserError::ConfigurationError => PyValueError::new_err(user_error.to_string()),
            UserError::SystemError => PyRuntimeError::new_err(user_error.to_string()),
        }
    }
}
```

### Exception Type Guidelines

- `PyValueError`: For invalid input parameters, configuration errors
- `PyRuntimeError`: For operational failures, system errors, resource issues
- `PyException`: Generic fallback (avoid in specific error handling)

## Module Organization

### Clean Import Structure

The pylatro module follows a clean organization pattern:

```rust
#[pymodule]
fn pylatro(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Core game classes
    m.add_class::<Config>()?;
    m.add_class::<GameEngine>()?;
    m.add_class::<GameState>()?;
    m.add_class::<Stage>()?;

    // Joker system types
    m.add_class::<JokerId>()?;
    m.add_class::<JokerRarity>()?;
    m.add_class::<JokerDefinition>()?;
    m.add_class::<JokerMetadata>()?;
    m.add_class::<UnlockCondition>()?;
    
    // Other types
    m.add_class::<ConsumableType>()?;

    Ok(())
}
```

### Avoiding Naming Conflicts

The project resolved naming conflicts by using specific naming conventions:

```rust
// ❌ PROBLEMATIC: Generic naming causing conflicts
pub struct GameState { /* voucher-specific state */ }

// ✅ RESOLVED: Specific naming preventing conflicts  
pub struct VoucherGameState { /* voucher-specific state */ }
```

**Resolution Strategy:**
1. Rename conflicting types to be more specific
2. Update documentation to reflect resolution
3. Ensure no external dependencies are broken

## Performance Patterns

### Efficient Python Object Creation

For methods that create Python objects, the modern pattern avoids unnecessary GIL acquisition:

```rust
#[pymethods]
impl GameEngine {
    // ✅ GOOD: Accept py parameter for object creation
    fn get_joker_metadata(&self, joker_id: JokerId, py: Python<'_>) -> PyResult<Option<JokerMetadata>> {
        // Direct object creation without GIL re-acquisition
        let dict = pyo3::types::PyDict::new(py);
        dict.set_item("id", joker_id.to_string())?;
        // ...
    }
    
    // ❌ AVOID: Methods returning PyObject without py parameter require Python::with_gil
    fn get_joker_properties(&self, joker_id: JokerId) -> Option<pyo3::PyObject> {
        pyo3::Python::with_gil(|py| {
            // This pattern is still valid but less efficient
            let dict = pyo3::types::PyDict::new(py);
            // ...
        })
    }
}
```

### Safe JSON Conversion

The project includes a modern JSON-to-Python conversion utility:

```rust
/// Safely convert serde_json::Value to Python object using modern PyO3 APIs
fn safe_json_to_py(py: Python, value: &serde_json::Value) -> PyResult<pyo3::PyObject> {
    use pyo3::types::{PyDict, PyList};
    
    match value {
        serde_json::Value::Null => Ok(py.None()),
        serde_json::Value::Bool(b) => Ok((*b).into_py(py)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.into_py(py))
            } else if let Some(f) = n.as_f64() {
                Ok(f.into_py(py))
            } else {
                Ok(py.None())  // Handle edge cases gracefully
            }
        }
        serde_json::Value::String(s) => Ok(s.into_py(py)),
        serde_json::Value::Array(arr) => {
            let py_list = PyList::empty(py);
            for item in arr {
                py_list.append(safe_json_to_py(py, item)?)?;
            }
            Ok(py_list.into())
        }
        serde_json::Value::Object(obj) => {
            let py_dict = PyDict::new(py);
            for (key, value) in obj {
                py_dict.set_item(key, safe_json_to_py(py, value)?)?;
            }
            Ok(py_dict.into())
        }
    }
}
```

This pattern provides:
- Proper error handling for all JSON value types
- Graceful handling of edge cases (very large numbers)
- Memory-safe recursive conversion
- Modern PyO3 object creation patterns

## Testing Patterns

### Feature-Gated Tests

```rust
#[cfg(feature = "python")]
mod python_tests {
    use super::*;
    use pyo3::prelude::*;

    #[test]
    fn test_pyclass_creation() {
        Python::with_gil(|py| {
            let config = Config::default();
            let py_config = Py::new(py, config).unwrap();
            
            // Test module attribution
            assert_eq!(py_config.as_ref(py).get_type().module(), Some("pylatro"));
        });
    }
}
```

## Migration Guidelines

### When Updating PyO3 Versions

1. **Check Breaking Changes**: Review PyO3 changelog for breaking changes
2. **Update Patterns**: Apply new recommended patterns systematically
3. **Test Thoroughly**: Ensure both Rust tests and Python integration tests pass
4. **Update Documentation**: Keep this document current with new patterns

### Adding New Python Bindings

When adding new types to Python bindings:

1. **Use Standard Attributes**:
   ```rust
   #[cfg_attr(feature = "python", pyclass(module = "pylatro"))]
   ```

2. **Follow Naming Conventions**: Use specific names to avoid conflicts

3. **Add to Module**: Include in the `pylatro` function in `lib.rs`

4. **Document Public API**: Update relevant documentation files

5. **Add Tests**: Include both unit tests and integration tests

### Deprecating APIs

When deprecating Python-exposed methods:

1. **Add py Parameter**: Enable modern warning patterns
2. **Clear Warning Messages**: Explain what to use instead
3. **Version Information**: Indicate when removal will occur
4. **Migration Path**: Provide clear upgrade instructions

## Best Practices Summary

### DO:
- ✅ Always specify `module = "pylatro"` in pyclass attributes
- ✅ Use `py: Python<'_>` parameter in pymethods for deprecation warnings
- ✅ Map errors to appropriate Python exception types
- ✅ Use feature gates `#[cfg_attr(feature = "python", ...)]` for optional bindings
- ✅ Provide clear deprecation messages with migration guidance
- ✅ Test both Rust and Python interfaces thoroughly

### DON'T:
- ❌ Use `Python::with_gil` unnecessarily in pymethods blocks
- ❌ Create generic type names that cause conflicts
- ❌ Expose internal error types directly to Python
- ❌ Skip module attribution in pyclass definitions
- ❌ Leave deprecated methods without clear alternatives

## Future Considerations

### Performance Optimizations

Future PyO3 versions may provide:
- Better automatic GIL management
- More efficient object conversion patterns
- Improved memory management for large objects

### API Stability

Maintain backward compatibility by:
- Using feature flags for experimental APIs
- Providing clear migration paths for deprecated features
- Following semantic versioning for breaking changes

This document should be updated whenever significant changes are made to the PyO3 binding patterns or when upgrading to new PyO3 versions.