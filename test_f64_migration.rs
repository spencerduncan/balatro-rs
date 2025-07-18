// Simple test to verify f64 migration works
use balatro_rs::joker::JokerEffect;

fn main() {
    // Test that JokerEffect now accepts f64 values
    let effect = JokerEffect::new()
        .with_chips(123.456)
        .with_mult(45.789)
        .with_money(67.123)
        .with_mult_multiplier(2.123456789);
    
    println!("F64 migration test successful!");
    println!("Chips: {}", effect.chips);
    println!("Mult: {}", effect.mult);
    println!("Money: {}", effect.money);
    println!("Mult multiplier: {}", effect.mult_multiplier);
    
    // Verify the values are correct
    assert_eq!(effect.chips, 123.456);
    assert_eq!(effect.mult, 45.789);
    assert_eq!(effect.money, 67.123);
    assert_eq!(effect.mult_multiplier, 2.123456789);
    
    println!("All f64 precision tests passed!");
}