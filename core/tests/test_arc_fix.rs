//! Test to verify Arc::get_mut fix with RwLock pattern

use balatro_rs::domain::services::game_service::DomainError;
use balatro_rs::domain::services::{
    ActionHistoryRepository, GameRepository, GameService, SessionRepository,
};
use balatro_rs::domain::SessionId;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

/// Simple in-memory game repository
struct InMemoryGameRepo {
    storage: HashMap<SessionId, Vec<u8>>,
}

impl InMemoryGameRepo {
    fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }
}

impl GameRepository for InMemoryGameRepo {
    fn save(&mut self, session_id: &SessionId, state: Vec<u8>) -> Result<(), DomainError> {
        self.storage.insert(session_id.clone(), state);
        Ok(())
    }

    fn load(&self, session_id: &SessionId) -> Result<Vec<u8>, DomainError> {
        self.storage
            .get(session_id)
            .cloned()
            .ok_or(DomainError::RepositoryError(Cow::Borrowed(
                "Game state not found",
            )))
    }

    fn delete(&mut self, session_id: &SessionId) -> Result<(), DomainError> {
        self.storage.remove(session_id);
        Ok(())
    }
}

/// Simple in-memory session repository
struct InMemorySessionRepo {
    sessions: HashMap<SessionId, bool>,
}

impl InMemorySessionRepo {
    fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }
}

impl SessionRepository for InMemorySessionRepo {
    fn create(&mut self) -> Result<SessionId, DomainError> {
        let session_id = SessionId::new();
        self.sessions.insert(session_id.clone(), true);
        Ok(session_id)
    }

    fn exists(&self, session_id: &SessionId) -> Result<bool, DomainError> {
        Ok(self.sessions.contains_key(session_id))
    }

    fn touch(&mut self, session_id: &SessionId) -> Result<(), DomainError> {
        if !self.sessions.contains_key(session_id) {
            return Err(DomainError::RepositoryError(Cow::Borrowed(
                "Session not found",
            )));
        }
        Ok(())
    }
}

/// Simple in-memory action history repository
struct InMemoryHistoryRepo {
    history: HashMap<SessionId, Vec<Vec<u8>>>,
}

impl InMemoryHistoryRepo {
    fn new() -> Self {
        Self {
            history: HashMap::new(),
        }
    }
}

impl ActionHistoryRepository for InMemoryHistoryRepo {
    fn record(&mut self, session_id: &SessionId, action: Vec<u8>) -> Result<(), DomainError> {
        self.history
            .entry(session_id.clone())
            .or_default()
            .push(action);
        Ok(())
    }

    fn get_history(&self, session_id: &SessionId) -> Result<Vec<Vec<u8>>, DomainError> {
        Ok(self.history.get(session_id).cloned().unwrap_or_default())
    }

    fn clear(&mut self, session_id: &SessionId) -> Result<(), DomainError> {
        self.history.remove(session_id);
        Ok(())
    }
}

#[test]
fn test_arc_rwlock_pattern_no_panic() {
    // This test demonstrates that with RwLock, we can have multiple Arc references
    // and still get mutable access, unlike Arc::get_mut which would panic

    let game_repo = InMemoryGameRepo::new();
    let session_repo = InMemorySessionRepo::new();
    let history_repo = InMemoryHistoryRepo::new();

    let service = GameService::new(game_repo, session_repo, history_repo);

    // Clone the service - creates multiple Arc references
    let service_clone1 = service.clone();
    let service_clone2 = service.clone();

    // Start a new game
    let session_id = service.start_new_game().expect("Failed to start game");

    // Save game state from first clone - this would panic with Arc::get_mut
    let state1 = vec![1, 2, 3, 4];
    service_clone1
        .save_game(&session_id, state1.clone())
        .expect("Failed to save game - Arc::get_mut would have panicked here!");

    // Load game state from second clone
    let loaded_state = service_clone2
        .load_game(&session_id)
        .expect("Failed to load game");
    assert_eq!(loaded_state, state1);

    // Record action from original service
    let action = vec![5, 6, 7];
    service
        .record_action(&session_id, action.clone())
        .expect("Failed to record action");

    // Get history from first clone
    let history = service_clone1
        .get_action_history(&session_id)
        .expect("Failed to get history");
    assert_eq!(history.len(), 1);
    assert_eq!(history[0], action);

    println!("✓ RwLock pattern works correctly with multiple Arc references");
}

#[test]
fn test_concurrent_access_no_deadlock() {
    // This test verifies that concurrent access doesn't cause deadlocks

    let game_repo = InMemoryGameRepo::new();
    let session_repo = InMemorySessionRepo::new();
    let history_repo = InMemoryHistoryRepo::new();

    let service = Arc::new(GameService::new(game_repo, session_repo, history_repo));

    // Start a session
    let session_id = service.start_new_game().expect("Failed to start game");

    // Spawn multiple threads accessing the service concurrently
    let mut handles = vec![];

    for i in 0..10 {
        let service_clone = Arc::clone(&service);
        let session_id_clone = session_id.clone();

        let handle = thread::spawn(move || {
            // Each thread performs multiple operations
            let state = vec![i as u8; 4];

            // Save game state
            service_clone
                .save_game(&session_id_clone, state.clone())
                .expect("Failed to save game");

            // Record an action
            let action = vec![i as u8; 2];
            service_clone
                .record_action(&session_id_clone, action)
                .expect("Failed to record action");

            // Load game state
            let _loaded = service_clone
                .load_game(&session_id_clone)
                .expect("Failed to load game");
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    // Verify we have 10 actions recorded
    let history = service
        .get_action_history(&session_id)
        .expect("Failed to get history");
    assert_eq!(history.len(), 10);

    println!("✓ Concurrent access works without deadlocks");
}

#[test]
fn test_performance_comparison() {
    // This test demonstrates the performance characteristics of RwLock
    use std::time::Instant;

    let game_repo = InMemoryGameRepo::new();
    let session_repo = InMemorySessionRepo::new();
    let history_repo = InMemoryHistoryRepo::new();

    let service = GameService::new(game_repo, session_repo, history_repo);

    let session_id = service.start_new_game().expect("Failed to start game");

    // Measure read performance (multiple readers allowed)
    let start = Instant::now();
    let mut read_handles = vec![];

    for _ in 0..100 {
        let service_clone = service.clone();
        let session_id_clone = session_id.clone();

        let handle = thread::spawn(move || {
            // Save initial state
            service_clone
                .save_game(&session_id_clone, vec![1, 2, 3, 4])
                .ok();

            // Perform 100 reads
            for _ in 0..100 {
                let _ = service_clone.load_game(&session_id_clone);
            }
        });

        read_handles.push(handle);
    }

    for handle in read_handles {
        handle.join().expect("Thread panicked");
    }

    let read_duration = start.elapsed();
    println!("✓ RwLock allows concurrent reads - 10,000 reads in {read_duration:?}");

    // Verify the service is still functional
    let _final_history = service
        .get_action_history(&session_id)
        .expect("Failed to get final history");
    println!("✓ Service remains functional after stress test");
}
