//! Execute Game Action Use Case
//!
//! Encapsulates the complete workflow for executing a game action,
//! including validation, state updates, and result processing.

use crate::application::{
    config::SessionId,
    errors::ApplicationError,
    services::GameApplicationService,
    container::ActionResult,
};
use crate::action::Action;
use std::sync::Arc;

/// Request for executing a game action
#[derive(Debug, Clone)]
pub struct ExecuteActionRequest {
    pub session_id: SessionId,
    pub action: Action,
}

/// Response for game action execution
#[derive(Debug, Clone)]
pub struct ExecuteActionResponse {
    pub result: ActionResult,
    pub executed: bool,
}

/// Execute Game Action Use Case
///
/// Orchestrates action execution with proper validation,
/// state management, and result tracking.
pub struct ExecuteGameActionUseCase {
    game_service: Arc<GameApplicationService>,
}

impl ExecuteGameActionUseCase {
    /// Create a new use case instance
    pub fn new(game_service: Arc<GameApplicationService>) -> Self {
        Self { game_service }
    }

    /// Execute the game action use case
    ///
    /// # Arguments
    /// * `request` - Action execution request
    ///
    /// # Returns
    /// * `Ok(ExecuteActionResponse)` - Successful action execution
    /// * `Err(ApplicationError)` - Execution failure
    pub async fn execute(&self, request: ExecuteActionRequest) -> Result<ExecuteActionResponse, ApplicationError> {
        // Delegate to game application service
        let result = self.game_service.execute_action(&request.session_id, request.action).await?;
        
        Ok(ExecuteActionResponse {
            result,
            executed: true,
        })
    }
}