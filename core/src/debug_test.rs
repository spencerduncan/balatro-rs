#[cfg(test)]
mod debug_test {
    use crate::action::Action;
    use crate::game::Game;
    use crate::stage::Stage;

#[test]
fn debug_pack_system() {
    let mut game = Game::default();
    game.start();
    
    // Directly set game to shop stage for testing (like the pack tests do)
    game.stage = Stage::Shop();
    
    // Ensure player has enough money for pack purchases
    game.money = 20;
    
    println!("Game stage: {:?}", game.stage);
    println!("Game money: {}", game.money);
    
    let actions: Vec<Action> = game.gen_actions().collect();
    println!("Generated actions: {:?}", actions);
    
    let pack_actions: Vec<&Action> = actions.iter()
        .filter(|action| matches!(action, Action::BuyPack { .. }))
        .collect();
    
    println!("Pack actions: {:?}", pack_actions);
}
}