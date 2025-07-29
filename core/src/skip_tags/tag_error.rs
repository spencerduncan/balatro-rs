use std::fmt;

/// Errors that can occur during skip tag operations
#[derive(Debug, Clone, PartialEq)]
pub struct TagError {
    pub kind: TagErrorKind,
    pub message: String,
}

impl TagError {
    /// Create a new TagError with the specified kind and message
    pub fn new(kind: TagErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }
    
    /// Create a validation error
    pub fn validation(message: impl Into<String>) -> Self {
        Self::new(TagErrorKind::Validation, message)
    }
    
    /// Create an application error
    pub fn application(message: impl Into<String>) -> Self {
        Self::new(TagErrorKind::Application, message)
    }
    
    /// Create a state error
    pub fn state(message: impl Into<String>) -> Self {
        Self::new(TagErrorKind::InvalidState, message)
    }
    
    /// Create a resource error
    pub fn resource(message: impl Into<String>) -> Self {
        Self::new(TagErrorKind::InsufficientResources, message)
    }
}

/// Categories of tag-related errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagErrorKind {
    /// Tag cannot be applied due to failed validation
    Validation,
    
    /// Error occurred while applying tag effect
    Application,
    
    /// Game state is invalid for tag operation
    InvalidState,
    
    /// Insufficient resources to apply tag effect
    InsufficientResources,
    
    /// Tag not found in registry
    NotFound,
    
    /// Internal system error
    Internal,
}

impl fmt::Display for TagError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)
    }
}

impl fmt::Display for TagErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TagErrorKind::Validation => write!(f, "Validation Error"),
            TagErrorKind::Application => write!(f, "Application Error"), 
            TagErrorKind::InvalidState => write!(f, "Invalid State"),
            TagErrorKind::InsufficientResources => write!(f, "Insufficient Resources"),
            TagErrorKind::NotFound => write!(f, "Tag Not Found"),
            TagErrorKind::Internal => write!(f, "Internal Error"),
        }
    }
}

impl std::error::Error for TagError {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tag_error_creation() {
        let error = TagError::validation("Test validation error");
        assert_eq!(error.kind, TagErrorKind::Validation);
        assert_eq!(error.message, "Test validation error");
    }
    
    #[test]
    fn test_tag_error_display() {
        let error = TagError::application("Test application error");
        let display_string = format!("{}", error);
        assert!(display_string.contains("Application Error"));
        assert!(display_string.contains("Test application error"));
    }
}