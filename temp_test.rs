#[cfg(test)]
mod temp_tests {
    use crate::joker::JokerEffect;
    
    #[test]
    fn debug_default_values() {
        let effect = JokerEffect::new();
        println\!("Default JokerEffect values:");
        println\!("  chips: {}", effect.chips);
        println\!("  mult: {}", effect.mult);
        println\!("  money: {}", effect.money);
        println\!("  mult_multiplier: {}", effect.mult_multiplier);
        println\!("  retrigger: {}", effect.retrigger);
        println\!("  destroy_self: {}", effect.destroy_self);
        println\!("  destroy_others.len(): {}", effect.destroy_others.len());
        println\!("  transform_cards.len(): {}", effect.transform_cards.len());
        println\!("  hand_size_mod: {}", effect.hand_size_mod);
        println\!("  discard_mod: {}", effect.discard_mod);
        println\!("  sell_value_increase: {}", effect.sell_value_increase);
        println\!("  message.is_none(): {}", effect.message.is_none());
    }
}
EOF < /dev/null
