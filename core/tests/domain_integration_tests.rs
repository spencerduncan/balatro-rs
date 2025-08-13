//! Integration tests for domain layer

use balatro_rs::domain::{DomainError, GameSession, Money, Score, SessionId};
use balatro_rs::game::Game;

#[test]
fn test_domain_error_conversions() {
    let domain_err = DomainError::NotFound("test".into());
    let _dev_err: balatro_rs::error::DeveloperGameError = domain_err.clone().into();
    let _user_err: balatro_rs::error::UserError = domain_err.into();
}

#[test]
fn test_money_value_object() {
    let money = Money::new(100);
    assert_eq!(money.amount(), 100);

    let added = money.add(Money::new(50));
    assert_eq!(added.amount(), 150);

    let subtracted = added.subtract(Money::new(25)).unwrap();
    assert_eq!(subtracted.amount(), 125);

    assert!(Money::try_new(-10).is_err());
}

#[test]
fn test_score_value_object() {
    let score = Score::new(1000);
    assert_eq!(score.value(), 1000);

    let new_score = score.add(Score::new(500));
    assert_eq!(new_score.value(), 1500);
}

#[test]
fn test_session_id_uniqueness() {
    let id1 = SessionId::new();
    let id2 = SessionId::new();
    assert_ne!(id1.value(), id2.value());
}

#[test]
fn test_game_session_lifecycle() {
    let game = Game::default();
    let mut session = GameSession::new(game);

    assert_eq!(
        session.state(),
        balatro_rs::domain::entities::SessionState::Active
    );
    assert!(!session.is_expired());

    session.suspend();
    assert_eq!(
        session.state(),
        balatro_rs::domain::entities::SessionState::Suspended
    );

    assert!(session.resume().is_ok());
    assert_eq!(
        session.state(),
        balatro_rs::domain::entities::SessionState::Active
    );

    session.complete();
    assert_eq!(
        session.state(),
        balatro_rs::domain::entities::SessionState::Completed
    );
}

#[test]
fn test_game_session_persistence() {
    let game = Game::default();
    let session = GameSession::new(game);
    let id = session.id();

    let serialized = session.serialize().unwrap();
    assert!(!serialized.is_empty());

    let deserialized = GameSession::deserialize(&serialized).unwrap();
    assert_eq!(deserialized.id(), id);
}

#[test]
fn test_session_repository() {
    let repo = balatro_rs::domain::entities::SessionRepository::new();
    let session = GameSession::new(Game::default());
    let id = session.id();

    assert!(repo.save(session).is_ok());

    let loaded = repo.load(id).unwrap();
    assert!(!loaded.is_empty());

    assert!(repo.delete(id).is_ok());
}
