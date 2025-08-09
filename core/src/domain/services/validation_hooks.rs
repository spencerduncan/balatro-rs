//! # Validation Integration Hooks
//!
//! This module provides hooks for integrating the ActionValidator
//! with the game action handling system.

use crate::action::Action;
use crate::domain::services::action_validator::{ActionValidator, ValidationContext};
use crate::domain::value_objects::ValidationResult;
use crate::domain::DomainResult;
use crate::game::Game;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Trait for pre-validation hooks
pub trait PreValidationHook: Send + Sync {
    /// Execute before validation
    fn execute(&self, action: &Action, game: &Game) -> DomainResult<()>;
}

/// Trait for post-validation hooks
pub trait PostValidationHook: Send + Sync {
    /// Execute after validation
    fn execute(&self, action: &Action, game: &Game, result: &ValidationResult) -> DomainResult<()>;
}

/// Async validation support
pub trait AsyncValidator: Send + Sync {
    /// Validate action asynchronously
    fn validate_async(
        &self,
        action: Action,
        game: Game,
    ) -> Pin<Box<dyn Future<Output = ValidationResult> + Send>>;
}

/// Integration point for game action handling
pub struct ValidationIntegration {
    validator: Arc<ActionValidator>,
    pre_hooks: Vec<Arc<dyn PreValidationHook>>,
    post_hooks: Vec<Arc<dyn PostValidationHook>>,
}

impl ValidationIntegration {
    /// Create new validation integration
    pub fn new(validator: Arc<ActionValidator>) -> Self {
        Self {
            validator,
            pre_hooks: Vec::new(),
            post_hooks: Vec::new(),
        }
    }

    /// Add a pre-validation hook
    pub fn add_pre_hook(&mut self, hook: Arc<dyn PreValidationHook>) {
        self.pre_hooks.push(hook);
    }

    /// Add a post-validation hook
    pub fn add_post_hook(&mut self, hook: Arc<dyn PostValidationHook>) {
        self.post_hooks.push(hook);
    }

    /// Validate an action with hooks
    pub fn validate_with_hooks(
        &self,
        action: &Action,
        game: &Game,
    ) -> DomainResult<ValidationResult> {
        // Run pre-validation hooks
        for hook in &self.pre_hooks {
            hook.execute(action, game)?;
        }

        // Perform validation
        let result = self.validator.validate(action, game);

        // Run post-validation hooks
        for hook in &self.post_hooks {
            hook.execute(action, game, &result)?;
        }

        Ok(result)
    }
}

/// Async implementation of ActionValidator
pub struct AsyncActionValidator {
    validator: Arc<ActionValidator>,
}

impl AsyncActionValidator {
    pub fn new(validator: Arc<ActionValidator>) -> Self {
        Self { validator }
    }
}

impl AsyncValidator for AsyncActionValidator {
    fn validate_async(
        &self,
        action: Action,
        game: Game,
    ) -> Pin<Box<dyn Future<Output = ValidationResult> + Send>> {
        let validator = self.validator.clone();

        Box::pin(async move {
            // In a real implementation, this could perform async operations
            // like checking external services, rate limits, etc.
            validator.validate(&action, &game)
        })
    }
}

/// Validation middleware for request processing
pub struct ValidationMiddleware {
    validator: Arc<ActionValidator>,
    context: ValidationContext,
}

impl ValidationMiddleware {
    pub fn new(validator: Arc<ActionValidator>) -> Self {
        Self {
            validator,
            context: ValidationContext::default(),
        }
    }

    /// Process action through validation
    pub fn process(&self, action: &Action, game: &Game) -> ValidationResult {
        self.validator
            .validate_with_context(action, game, &self.context)
    }
}
