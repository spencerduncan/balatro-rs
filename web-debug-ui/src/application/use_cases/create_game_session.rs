//! Create Game Session Use Case
//!
//! Encapsulates the complete workflow for creating a new game session,
//! including validation, resource allocation, and initial state setup.

use crate::application::{
    config::{GameConfig, SessionId},
    errors::ApplicationError,
    services::SessionManagementService,
};
use std::sync::Arc;

/// Request for creating a game session
#[derive(Debug, Clone)]
pub struct CreateSessionRequest {
    pub config: GameConfig,
}

/// Response for game session creation
#[derive(Debug, Clone)]
pub struct CreateSessionResponse {
    pub session_id: SessionId,
    pub created: bool,
}

/// Create Game Session Use Case
///
/// Orchestrates session creation with proper validation,
/// resource management, and error handling.
pub struct CreateGameSessionUseCase {
    session_service: Arc<SessionManagementService>,
}

impl CreateGameSessionUseCase {
    /// Create a new use case instance
    pub fn new(session_service: Arc<SessionManagementService>) -> Self {
        Self { session_service }
    }

    /// Execute the create session use case
    ///
    /// # Arguments
    /// * `request` - Session creation request
    ///
    /// # Returns
    /// * `Ok(CreateSessionResponse)` - Successful session creation
    /// * `Err(ApplicationError)` - Creation failure
    pub async fn execute(
        &self,
        request: CreateSessionRequest,
    ) -> Result<CreateSessionResponse, ApplicationError> {
        // Delegate to session management service
        let session_id = self.session_service.create_session(request.config).await?;

        Ok(CreateSessionResponse {
            session_id,
            created: true,
        })
    }
}
