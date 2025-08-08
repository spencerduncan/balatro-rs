//! Demonstration that the Arc::get_mut issue is fixed with RwLock pattern
//!
//! This example shows that with the RwLock pattern, we can have multiple
//! Arc references and still get mutable access, unlike Arc::get_mut which
//! would always fail and panic.

use balatro_rs::domain::services::game_service::DomainError;
use balatro_rs::domain::services::{
    ActionHistoryRepository, GameRepository, GameService, SessionRepository,
};
use balatro_rs::domain::SessionId;
use std::borrow::Cow;
use std::collections::HashMap;

/// Simple in-memory implementations for demonstration
struct SimpleGameRepo {
    storage: HashMap<SessionId, Vec<u8>>,
}

impl SimpleGameRepo {
    fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }
}

impl GameRepository for SimpleGameRepo {
    fn save(&mut self, session_id: &SessionId, state: Vec<u8>) -> Result<(), DomainError> {
        self.storage.insert(session_id.clone(), state);
        Ok(())
    }

    fn load(&self, session_id: &SessionId) -> Result<Vec<u8>, DomainError> {
        self.storage
            .get(session_id)
            .cloned()
            .ok_or(DomainError::RepositoryError(Cow::Borrowed("Not found")))
    }

    fn delete(&mut self, session_id: &SessionId) -> Result<(), DomainError> {
        self.storage.remove(session_id);
        Ok(())
    }
}

struct SimpleSessionRepo {
    sessions: HashMap<SessionId, bool>,
}

impl SimpleSessionRepo {
    fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }
}

impl SessionRepository for SimpleSessionRepo {
    fn create(&mut self) -> Result<SessionId, DomainError> {
        let id = SessionId::new();
        self.sessions.insert(id.clone(), true);
        Ok(id)
    }

    fn exists(&self, session_id: &SessionId) -> Result<bool, DomainError> {
        Ok(self.sessions.contains_key(session_id))
    }

    fn touch(&mut self, _session_id: &SessionId) -> Result<(), DomainError> {
        Ok(())
    }
}

struct SimpleHistoryRepo {
    history: HashMap<SessionId, Vec<Vec<u8>>>,
}

impl SimpleHistoryRepo {
    fn new() -> Self {
        Self {
            history: HashMap::new(),
        }
    }
}

impl ActionHistoryRepository for SimpleHistoryRepo {
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

fn main() {
    println!("=== Arc::get_mut Pattern Fix Demonstration ===\n");

    // Create the service with our RwLock-based implementation
    let service = GameService::new(
        SimpleGameRepo::new(),
        SimpleSessionRepo::new(),
        SimpleHistoryRepo::new(),
    );

    println!("✓ Created GameService with RwLock pattern");

    // Clone the service multiple times - creates multiple Arc references
    let service_clone1 = service.clone();
    let service_clone2 = service.clone();
    println!("✓ Created multiple Arc references (3 total)");

    // Start a new game
    let session_id = service.start_new_game().expect("Failed to start game");
    println!("✓ Started new game session: {}", session_id.as_str());

    // CRITICAL: This is where Arc::get_mut would FAIL
    // With Arc::get_mut, having multiple references would make it return None
    // causing a panic. Our RwLock pattern handles this correctly.
    println!("\n⚠️  CRITICAL OPERATION - This would panic with Arc::get_mut!");

    let state = vec![1, 2, 3, 4];
    service_clone1
        .save_game(&session_id, state.clone())
        .expect("Failed to save game");
    println!("✓ Successfully saved game state from clone #1");
    println!("  (Arc::get_mut would have returned None and panicked here!)");

    // Load from a different clone
    let loaded = service_clone2
        .load_game(&session_id)
        .expect("Failed to load game");
    assert_eq!(loaded, state);
    println!("✓ Successfully loaded game state from clone #2");

    // Record actions from original service
    for i in 1..=3 {
        let action = vec![i; 2];
        service
            .record_action(&session_id, action)
            .expect("Failed to record action");
    }
    println!("✓ Recorded 3 actions from original service");

    // Get history from first clone
    let history = service_clone1
        .get_action_history(&session_id)
        .expect("Failed to get history");
    println!("✓ Retrieved {} actions from clone #1", history.len());

    println!("\n=== Performance Characteristics ===");
    println!("• RwLock allows multiple concurrent readers");
    println!("• Write locks are exclusive but infrequent");
    println!("• No runtime panics from Arc::get_mut failures");
    println!("• Thread-safe across multiple service instances");

    println!("\n✅ DEMONSTRATION COMPLETE");
    println!("The RwLock pattern successfully replaces the broken Arc::get_mut pattern.");
    println!("Production code will no longer panic at runtime!");
}
