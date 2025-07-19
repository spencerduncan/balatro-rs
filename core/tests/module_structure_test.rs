//! Tests for module structure reorganization
//! Validates that new modules can be imported and existing API compatibility is maintained

#[cfg(test)]
mod tests {
    // Test that existing API imports still work after reorganization
    #[test]
    fn test_existing_api_compatibility() {
        // These imports should continue to work after module reorganization
        use balatro_rs::{action::Action, game::Game, stage::Stage};
        use balatro_rs::{config::Config, deck::Deck};

        // Verify basic types can be instantiated
        let _game = Game::default();
        let _deck = Deck::default();
        let _config = Config::default();

        // Verify enums are accessible with correct variants
        let _stage = Stage::PreBlind();
        let _action = Action::Play();

        // If this compiles and runs, existing API compatibility is maintained
        // Verify that default instances can be created (API compatibility)
        assert_eq!(_stage, Stage::PreBlind());
    }

    // Test that new modules can be imported (should pass now - TDD green phase)
    #[test]
    fn test_new_module_imports() {
        // These should now be available after implementing the new modules
        use balatro_rs::boss_blinds::{BossBlindId, BossBlindState, HandModification};
        use balatro_rs::consumables::{ConsumableId, ConsumableType};
        use balatro_rs::vouchers::{VoucherCollection, VoucherId};

        // Test that basic types can be instantiated
        let _consumable_id = ConsumableId::TarotPlaceholder;
        let _consumable_type = ConsumableType::Tarot;
        let _voucher_id = VoucherId::VoucherPlaceholder;
        let _voucher_collection = VoucherCollection::new();
        let _boss_blind_id = BossBlindId::BossBlindPlaceholder;
        let _boss_blind_state = BossBlindState::new();
        let _hand_modification = HandModification::new();

        // Test that enums work correctly
        assert_eq!(_consumable_id.consumable_type(), ConsumableType::Tarot);
        assert_eq!(_voucher_collection.count(), 0);
        assert!(!_boss_blind_state.is_active());
        assert_eq!(_boss_blind_state.active_boss(), None);
    }

    // Test that module hierarchy is correct (will implement as modules are added)
    #[test]
    fn test_module_hierarchy() {
        // Verify that modules don't have circular dependencies
        // This test will be expanded as we add each module

        // Test core module structure
        use balatro_rs::action;
        use balatro_rs::game;
        use balatro_rs::stage;

        // Verify these can be imported without conflicts
        let game_type = std::any::type_name::<game::Game>();
        let action_type = std::any::type_name::<action::Action>();
        let stage_type = std::any::type_name::<stage::Stage>();

        // Verify type names are correct (no module conflicts)
        assert!(game_type.contains("Game"));
        assert!(action_type.contains("Action"));
        assert!(stage_type.contains("Stage"));
    }

    // Test that lib.rs module declarations work correctly
    #[test]
    fn test_lib_module_declarations() {
        // Test that modules can be imported via their full paths
        use balatro_rs::action::Action;
        use balatro_rs::game::Game;
        use balatro_rs::stage::Stage;

        // Verify types can be used
        let _action = Action::Play();
        let game = Game::default();
        let stage = Stage::PreBlind();

        // Verify instances can be created and have expected values
        assert_eq!(stage, Stage::PreBlind());
        assert_eq!(game.stage, stage);

        // Future: When we add re-exports, we'll test those here
        // use balatro_rs::Action;  // Via re-export
    }

    // Test for future Action enum extensions (placeholder)
    #[test]
    fn test_action_enum_extensibility() {
        use balatro_rs::action::{Action, MoveDirection};
        use balatro_rs::card::{Card, Suit, Value};
        use balatro_rs::joker::JokerId;
        use balatro_rs::stage::Blind;

        // Create valid instances for testing
        let test_card = Card::new(Value::King, Suit::Heart);

        // Current actions should still work (using correct variants)
        let _select_card = Action::SelectCard(test_card);
        let _move_card = Action::MoveCard(MoveDirection::Left, test_card);
        let _play = Action::Play();
        let _discard = Action::Discard();
        let _cash_out = Action::CashOut(100.0);
        let _buy_joker = Action::BuyJoker {
            joker_id: JokerId::Joker,
            slot: 0,
        };
        let _next_round = Action::NextRound();
        let _select_blind = Action::SelectBlind(Blind::Small);

        // Verify actions can be created and are distinct
        assert_ne!(_play, _discard);
        assert_ne!(_cash_out, _next_round);

        // Future actions will be added here:
        // let _buy_consumable = Action::BuyConsumable(ConsumableId::...);
        // let _play_consumable = Action::PlayConsumable(ConsumableId::...);
        // let _buy_voucher = Action::BuyVoucher(VoucherId::...);
    }
}
