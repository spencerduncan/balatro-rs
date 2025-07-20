# f64 Display Formatting Utilities

This document describes the formatting utilities provided for displaying f64 game values with appropriate precision and formatting conventions.

## Overview

With the migration to f64 for all numeric game values, proper display formatting becomes crucial. These utilities ensure consistent, user-friendly display of game values while preserving exact precision where needed.

## Format Module API

### Core Formatting Functions

```rust
use balatro_rs::format::{
    format_score,    // Game score formatting
    format_money,    // Currency formatting
    format_chips,    // Chip count formatting
    format_mult,     // Multiplier formatting
    format_number,   // Generic number formatting
};
```

### format_score(score: f64) -> String

Formats game scores with intelligent decimal handling:

```rust
use balatro_rs::format::format_score;

// Integer values display without decimals
assert_eq!(format_score(12345.0), "12,345");
assert_eq!(format_score(1000000.0), "1,000,000");

// Fractional values show appropriate precision
assert_eq!(format_score(12345.5), "12,345.5");
assert_eq!(format_score(12345.75), "12,345.75");

// Very large numbers use scientific notation
assert_eq!(format_score(1e15), "1.00e15");

// Special values
assert_eq!(format_score(f64::INFINITY), "∞");
assert_eq!(format_score(f64::NAN), "—");
```

### format_money(amount: f64) -> String

Formats monetary values with currency symbols:

```rust
use balatro_rs::format::format_money;

// Standard currency formatting
assert_eq!(format_money(100.0), "$100");
assert_eq!(format_money(100.50), "$100.50");
assert_eq!(format_money(1234.56), "$1,234.56");

// Negative values
assert_eq!(format_money(-50.0), "-$50");

// Large amounts
assert_eq!(format_money(1000000.0), "$1,000,000");

// Special values
assert_eq!(format_money(f64::INFINITY), "$∞");
assert_eq!(format_money(f64::NAN), "$—");
```

### format_chips(chips: f64) -> String

Formats chip counts with intelligent precision:

```rust
use balatro_rs::format::format_chips;

// Integer chips display cleanly
assert_eq!(format_chips(1000.0), "1,000");
assert_eq!(format_chips(50.0), "50");

// Fractional chips show decimals
assert_eq!(format_chips(1000.25), "1,000.25");
assert_eq!(format_chips(50.5), "50.5");

// Large chip counts
assert_eq!(format_chips(1000000.0), "1,000,000");

// Special values
assert_eq!(format_chips(f64::INFINITY), "∞");
assert_eq!(format_chips(f64::NAN), "—");
```

### format_mult(multiplier: f64) -> String

Formats multipliers with 'x' prefix:

```rust
use balatro_rs::format::format_mult;

// Standard multipliers
assert_eq!(format_mult(5.0), "x5");
assert_eq!(format_mult(2.5), "x2.5");
assert_eq!(format_mult(1.25), "x1.25");

// Large multipliers
assert_eq!(format_mult(100.0), "x100");
assert_eq!(format_mult(1000.5), "x1,000.5");

// Special values
assert_eq!(format_mult(f64::INFINITY), "x∞");
assert_eq!(format_mult(f64::NAN), "x—");
assert_eq!(format_mult(0.0), "x0");
```

### format_number(value: f64, options: FormatOptions) -> String

Generic number formatting with customizable options:

```rust
use balatro_rs::format::{format_number, FormatOptions};

let options = FormatOptions {
    thousands_separator: true,
    decimal_places: Some(2),
    prefix: Some("$"),
    suffix: None,
    scientific_threshold: 1e12,
};

assert_eq!(format_number(1234.567, options), "$1,234.57");
```

## FormatOptions Structure

```rust
pub struct FormatOptions {
    /// Use thousands separators (commas)
    pub thousands_separator: bool,
    
    /// Fixed number of decimal places (None for intelligent)
    pub decimal_places: Option<usize>,
    
    /// Prefix string (e.g., "$", "x")
    pub prefix: Option<&'static str>,
    
    /// Suffix string (e.g., "%", "pts")
    pub suffix: Option<&'static str>,
    
    /// Threshold for scientific notation
    pub scientific_threshold: f64,
    
    /// How to handle special values
    pub special_handling: SpecialValueHandling,
}

pub enum SpecialValueHandling {
    Symbols,     // Use ∞, —, etc.
    Text,        // Use "Infinity", "NaN", etc.
    Hide,        // Return empty string
    Preserve,    // Return Rust's default format
}
```

## Usage Examples

### In Game UI

```rust
use balatro_rs::format::*;

fn display_game_state(game: &Game) {
    println!("Score: {}", format_score(game.score));
    println!("Money: {}", format_money(game.money));
    println!("Chips: {}", format_chips(game.chips));
    println!("Mult: {}", format_mult(game.mult));
}

// Output:
// Score: 125,450
// Money: $247.50
// Chips: 1,000.25
// Mult: x8.5
```

### In Joker Descriptions

```rust
use balatro_rs::format::*;

impl fmt::Display for JokerEffect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();
        
        if self.chips != 0.0 {
            if self.chips > 0.0 {
                parts.push(format!("+{}", format_chips(self.chips)));
            } else {
                parts.push(format_chips(self.chips));
            }
        }
        
        if self.mult != 0.0 {
            if self.mult > 0.0 {
                parts.push(format!("+{}", format_mult(self.mult)));
            } else {
                parts.push(format_mult(self.mult));
            }
        }
        
        write!(f, "{}", parts.join(", "))
    }
}

// Example output: "+150, x2.5"
```

### Custom Formatting

```rust
use balatro_rs::format::{format_number, FormatOptions, SpecialValueHandling};

// Percentage formatting
fn format_percentage(value: f64) -> String {
    let options = FormatOptions {
        thousands_separator: false,
        decimal_places: Some(1),
        prefix: None,
        suffix: Some("%"),
        scientific_threshold: f64::INFINITY,
        special_handling: SpecialValueHandling::Symbols,
    };
    format_number(value * 100.0, options)
}

assert_eq!(format_percentage(0.75), "75.0%");
assert_eq!(format_percentage(1.25), "125.0%");
```

## Performance Considerations

### Caching
For frequently displayed values, consider caching formatted strings:

```rust
use std::collections::HashMap;

pub struct FormatCache {
    cache: HashMap<(u64, u8), String>, // (bits, format_type)
}

impl FormatCache {
    pub fn format_score(&mut self, score: f64) -> &str {
        let key = (score.to_bits(), 0);
        self.cache.entry(key)
            .or_insert_with(|| format_score(score))
    }
}
```

### Allocation Optimization
For performance-critical code, use `write!` to avoid allocations:

```rust
use std::fmt::Write;

fn write_score(buffer: &mut String, score: f64) {
    buffer.clear();
    if score.fract() == 0.0 && score.is_finite() {
        write!(buffer, "{:,.0}", score).unwrap();
    } else {
        write!(buffer, "{:,.2}", score).unwrap();
    }
}
```

## Localization Support

### Number Format Variations
```rust
pub struct LocaleSettings {
    pub thousands_separator: char,
    pub decimal_separator: char,
    pub currency_symbol: &'static str,
    pub currency_position: CurrencyPosition,
}

pub enum CurrencyPosition {
    Before,  // $100
    After,   // 100$
    Spaced,  // $ 100 or 100 $
}
```

### Usage with Locales
```rust
use balatro_rs::format::{format_with_locale, Locale};

let european = Locale {
    thousands_separator: '.',
    decimal_separator: ',',
    currency_symbol: "€",
    currency_position: CurrencyPosition::After,
};

assert_eq!(format_with_locale(1234.56, &european), "1.234,56€");
```

## Testing Utilities

### Test Helpers
```rust
#[cfg(test)]
pub mod test_utils {
    use super::*;
    
    pub fn assert_formats_to(value: f64, expected: &str) {
        assert_eq!(format_score(value), expected);
    }
    
    pub fn assert_money_formats_to(value: f64, expected: &str) {
        assert_eq!(format_money(value), expected);
    }
}

#[cfg(test)]
mod tests {
    use super::test_utils::*;
    
    #[test]
    fn test_score_formatting() {
        assert_formats_to(1000.0, "1,000");
        assert_formats_to(1000.5, "1,000.5");
        assert_formats_to(1000000.0, "1,000,000");
    }
}
```

## Error Handling

### Robust Formatting
```rust
pub fn safe_format_score(score: f64) -> String {
    if !score.is_finite() {
        if score.is_nan() {
            return "—".to_string();
        } else if score.is_infinite() {
            return if score.is_sign_positive() { "∞" } else { "-∞" }.to_string();
        }
    }
    
    format_score(score)
}
```

### Debug Formatting
```rust
#[cfg(debug_assertions)]
pub fn debug_format_score(score: f64) -> String {
    format!("{} ({})", format_score(score), score)
}

// Example: "1,000 (1000.0)"
```

## Migration from Raw Display

### Before (Raw f64 Display)
```rust
// Problems with raw display
println!("Score: {}", game.score);     // "1000.0" instead of "1,000"
println!("Money: {}", game.money);     // "100.5" instead of "$100.50"
println!("Mult: {}", game.mult);       // "5.0" instead of "x5"
```

### After (Using Format Utilities)
```rust
use balatro_rs::format::*;

// Clean, consistent display
println!("Score: {}", format_score(game.score));  // "1,000"
println!("Money: {}", format_money(game.money));   // "$100.50"
println!("Mult: {}", format_mult(game.mult));      // "x5"
```

## Summary

The format utilities provide:
- **Consistent Display**: Unified formatting across the application
- **Intelligent Precision**: Shows decimals only when needed
- **Special Value Handling**: Proper display of NaN/infinity
- **Performance**: Optimized for frequent formatting operations
- **Localization Ready**: Extensible for international users
- **Type Safety**: Compile-time guarantees for format correctness

These utilities ensure that the transition to f64 maintains excellent user experience while providing the precision benefits of floating-point arithmetic.