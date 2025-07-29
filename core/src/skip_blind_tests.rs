#[cfg(test)]
mod skip_blind_tests {
    use crate::action::Action;
    use crate::config::Config;
    use crate::game::Game;
    use crate::stage::{Blind, Stage};

    #[test]
    fn test_skip_blind_basic_functionality() {
        let mut game = Game::new(Config::default());
        game.stage = Stage::PreBlind();
        game.blind = None;
        
        // Skip small blind at game start
        let result = game.handle_action(Action::SkipBlind(Blind::Small));
        assert!(result.is_ok(), "Should be able to skip small blind at game start");
        
        // Should progress to PostBlind stage
        assert_eq!(game.stage, Stage::PostBlind());
        assert_eq!(game.blind, Some(Blind::Small));
        
        // Should award half reward for skipping
        let expected_skip_reward = Blind::Small.reward() as f64 / 2.0;
        assert_eq!(game.money, Config::default().money_start as f64 + expected_skip_reward);
    }

    #[test]
    fn test_skip_blind_progression() {
        let mut game = Game::new(Config::default());
        game.stage = Stage::PreBlind();
        game.blind = Some(Blind::Small);
        
        // Skip big blind (next after small)
        let result = game.handle_action(Action::SkipBlind(Blind::Big));
        assert!(result.is_ok(), "Should be able to skip big blind");
        
        // Should progress correctly
        assert_eq!(game.stage, Stage::PostBlind());
        assert_eq!(game.blind, Some(Blind::Big));
    }

    #[test]
    fn test_skip_blind_invalid_stage() {
        let mut game = Game::new(Config::default());
        game.stage = Stage::Blind(Blind::Small); // Wrong stage
        
        let result = game.handle_action(Action::SkipBlind(Blind::Small));
        assert!(result.is_err(), "Should not be able to skip blind during blind stage");
    }

    #[test]
    fn test_skip_blind_invalid_blind() {
        let mut game = Game::new(Config::default());
        game.stage = Stage::PreBlind();
        game.blind = Some(Blind::Small);
        
        // Try to skip small blind when big blind is expected
        let result = game.handle_action(Action::SkipBlind(Blind::Small));
        assert!(result.is_err(), "Should not be able to skip wrong blind");
    }

    #[test]
    fn test_skip_blind_action_generation() {
        let mut game = Game::new(Config::default());
        game.stage = Stage::PreBlind();
        game.blind = None;
        
        let actions: Vec<Action> = game.gen_actions().collect();
        
        // Should generate both select and skip blind actions
        let has_select_blind = actions.iter().any(|a| matches!(a, Action::SelectBlind(Blind::Small)));
        let has_skip_blind = actions.iter().any(|a| matches!(a, Action::SkipBlind(Blind::Small)));
        
        assert!(has_select_blind, "Should generate select blind action");
        assert!(has_skip_blind, "Should generate skip blind action");
    }

    #[test]
    fn test_skip_blind_action_space() {
        let mut game = Game::new(Config::default());  
        game.stage = Stage::PreBlind();
        game.blind = None;
        
        let space = game.gen_action_space();
        let space_vec = space.to_vec();
        
        // Both select blind and skip blind should be unmasked
        let select_blind_index = space_vec.len() - 2;
        let skip_blind_index = space_vec.len() - 1;
        
        assert_eq!(space_vec[select_blind_index], 1, "Select blind should be unmasked");
        assert_eq!(space_vec[skip_blind_index], 1, "Skip blind should be unmasked");
        
        // Test action conversion
        let select_action = space.to_action(select_blind_index, &game).unwrap();
        let skip_action = space.to_action(skip_blind_index, &game).unwrap();
        
        assert_eq!(select_action, Action::SelectBlind(Blind::Small));
        assert_eq!(skip_action, Action::SkipBlind(Blind::Small));
    }

    #[test]
    fn test_skip_blind_display() {
        let action = Action::SkipBlind(Blind::Boss);
        assert_eq!(format!("{}", action), "SkipBlind: Boss Blind");
    }

    #[test] 
    fn test_skip_blind_all_types() {
        let mut game = Game::new(Config::default());
        
        // Test skipping each blind type
        for blind in [Blind::Small, Blind::Big, Blind::Boss] {
            game.stage = Stage::PreBlind();
            // Set current blind to the previous one in sequence (or None for Small)
            game.blind = match blind {
                Blind::Small => None,
                Blind::Big => Some(Blind::Small),
                Blind::Boss => Some(Blind::Big),
            };
            
            let initial_money = game.money;
            let result = game.handle_action(Action::SkipBlind(blind));
            
            assert!(result.is_ok(), "Should be able to skip {:?}", blind);
            assert_eq!(game.stage, Stage::PostBlind());
            assert_eq!(game.blind, Some(blind));
            
            let expected_reward = blind.reward() as f64 / 2.0;
            assert_eq!(game.money, initial_money + expected_reward, 
                      "Should award correct skip reward for {:?}", blind);
        }
    }
}