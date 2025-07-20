//! Display formatting utilities for f64 values in the game UI.
//!
//! This module provides formatting functions to display f64 values as integers
//! while preserving full precision internally. It handles edge cases like
//! very large numbers, NaN, and Infinity, and provides different display modes
//! for various UI contexts.

use std::fmt;

/// Threshold above which numbers are displayed in scientific notation
const SCIENTIFIC_NOTATION_THRESHOLD: f64 = 1_000_000_000.0; // 1 billion

/// Threshold below which very small positive numbers are displayed as 0
const ZERO_THRESHOLD: f64 = 0.0001;

/// Display mode for formatting f64 values
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayMode {
    /// Standard integer display (default)
    Integer,
    /// Scientific notation for large values
    Scientific,
    /// Debug mode showing full precision
    Debug,
    /// Compact mode with k/M/B suffixes
    Compact,
}

/// Configuration for number formatting
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FormatConfig {
    /// Display mode to use
    pub mode: DisplayMode,
    /// Whether to use thousand separators (commas)
    pub use_separators: bool,
    /// Maximum decimal places for scientific notation
    pub max_decimal_places: usize,
    /// Whether to show sign for positive numbers
    pub show_positive_sign: bool,
}

impl Default for FormatConfig {
    fn default() -> Self {
        Self {
            mode: DisplayMode::Integer,
            use_separators: true,
            max_decimal_places: 2,
            show_positive_sign: false,
        }
    }
}

/// Main trait for f64 display formatting
pub trait F64Display {
    /// Format the f64 value according to the given configuration
    fn format_display(&self, config: &FormatConfig) -> String;

    /// Format as integer (default mode)
    fn format_integer(&self) -> String {
        self.format_display(&FormatConfig::default())
    }

    /// Format in scientific notation
    fn format_scientific(&self) -> String {
        self.format_display(&FormatConfig {
            mode: DisplayMode::Scientific,
            ..Default::default()
        })
    }

    /// Format in debug mode with full precision
    fn format_debug(&self) -> String {
        self.format_display(&FormatConfig {
            mode: DisplayMode::Debug,
            use_separators: false,
            ..Default::default()
        })
    }

    /// Format in compact mode with k/M/B suffixes
    fn format_compact(&self) -> String {
        self.format_display(&FormatConfig {
            mode: DisplayMode::Compact,
            use_separators: false,
            ..Default::default()
        })
    }
}

impl F64Display for f64 {
    fn format_display(&self, config: &FormatConfig) -> String {
        // Handle special cases first
        if self.is_nan() {
            return "NaN".to_string();
        }

        if self.is_infinite() {
            return if *self > 0.0 { "∞" } else { "-∞" }.to_string();
        }

        // Handle very small positive numbers (only for non-scientific modes)
        if self.abs() < ZERO_THRESHOLD
            && *self != 0.0
            && config.mode != DisplayMode::Scientific
            && config.mode != DisplayMode::Debug
        {
            return "0".to_string();
        }

        match config.mode {
            DisplayMode::Integer => format_as_integer(*self, config),
            DisplayMode::Scientific => format_as_scientific(*self, config),
            DisplayMode::Debug => format_as_debug(*self, config),
            DisplayMode::Compact => format_as_compact(*self, config),
        }
    }
}

/// Format f64 as integer with optional thousand separators
fn format_as_integer(value: f64, config: &FormatConfig) -> String {
    // Round to nearest integer
    let rounded = value.round();

    // If the value is too large, fall back to scientific notation
    if rounded.abs() >= SCIENTIFIC_NOTATION_THRESHOLD {
        return format_as_scientific(value, config);
    }

    let integer_part = rounded as i64;
    let mut result = integer_part.abs().to_string();

    // Add thousand separators if requested
    if config.use_separators && result.len() > 3 {
        result = add_thousand_separators(&result);
    }

    // Handle negative sign
    if integer_part < 0 {
        result = format!("-{result}");
    } else if config.show_positive_sign && integer_part > 0 {
        result = format!("+{result}");
    }

    result
}

/// Format f64 in scientific notation
fn format_as_scientific(value: f64, config: &FormatConfig) -> String {
    if value == 0.0 {
        return "0".to_string();
    }

    let abs_value = value.abs();
    let exponent = abs_value.log10().floor() as i32;
    let mantissa = abs_value / 10_f64.powi(exponent);

    // Round mantissa to specified decimal places
    let factor = 10_f64.powi(config.max_decimal_places as i32);
    let rounded_mantissa = (mantissa * factor).round() / factor;

    let sign = if value < 0.0 {
        "-"
    } else if config.show_positive_sign {
        "+"
    } else {
        ""
    };

    format!("{sign}{rounded_mantissa}e{exponent:+}")
}

/// Format f64 in debug mode with full precision
fn format_as_debug(value: f64, _config: &FormatConfig) -> String {
    format!("{value:.15}")
}

/// Format f64 in compact mode with k/M/B suffixes
fn format_as_compact(value: f64, config: &FormatConfig) -> String {
    let abs_value = value.abs();

    let (divisor, suffix) = if abs_value >= 1_000_000_000.0 {
        (1_000_000_000.0, "B")
    } else if abs_value >= 1_000_000.0 {
        (1_000_000.0, "M")
    } else if abs_value >= 1_000.0 {
        (1_000.0, "k")
    } else {
        return format_as_integer(value, config);
    };

    let scaled = value / divisor;
    let sign = if value < 0.0 {
        "-"
    } else if config.show_positive_sign {
        "+"
    } else {
        ""
    };

    // Use up to 1 decimal place for compact display
    if (scaled * 10.0).round() / 10.0 == scaled.round() {
        format!("{sign}{:.0}{suffix}", scaled.abs().round())
    } else {
        format!("{sign}{:.1}{suffix}", scaled.abs())
    }
}

/// Add thousand separators (commas) to a numeric string
fn add_thousand_separators(s: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();

    for (i, &c) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i) % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }

    result
}

/// Convenience functions for common formatting operations
pub mod format {
    use super::*;

    /// Format a score or large integer value
    pub fn score(value: f64) -> String {
        value.format_integer()
    }

    /// Format a monetary value (like chips or dollars)
    pub fn money(value: f64) -> String {
        if value.abs() >= 1_000_000.0 {
            value.format_compact()
        } else {
            value.format_integer()
        }
    }

    /// Format a multiplier value
    pub fn multiplier(value: f64) -> String {
        if value.abs() < ZERO_THRESHOLD && value != 0.0 {
            return "×0".to_string();
        }

        // For multipliers, use a special format that shows decimals for small values
        if value.abs() < 1000.0 {
            // Show up to 1 decimal place, but remove unnecessary .0
            let formatted = format!("{value:.1}");
            let formatted = if formatted.ends_with(".0") {
                formatted[..formatted.len() - 2].to_string()
            } else {
                formatted
            };
            format!("×{formatted}")
        } else {
            let config = FormatConfig {
                mode: DisplayMode::Compact,
                use_separators: false,
                max_decimal_places: 1,
                show_positive_sign: false,
            };
            format!("×{}", value.format_display(&config))
        }
    }

    /// Format a percentage value
    pub fn percentage(value: f64) -> String {
        let percent = value * 100.0;
        if percent.abs() < 0.1 && percent != 0.0 {
            return "0%".to_string();
        }
        format!("{percent:.1}%")
    }

    /// Format for debugging purposes
    pub fn debug(value: f64) -> String {
        value.format_debug()
    }
}

/// A wrapper type that automatically formats f64 values for display
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DisplayF64 {
    value: f64,
    config: FormatConfig,
}

impl DisplayF64 {
    /// Create a new DisplayF64 with default formatting
    pub fn new(value: f64) -> Self {
        Self {
            value,
            config: FormatConfig::default(),
        }
    }

    /// Create a new DisplayF64 with custom formatting configuration
    pub fn with_config(value: f64, config: FormatConfig) -> Self {
        Self { value, config }
    }

    /// Get the raw f64 value
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Update the formatting configuration
    pub fn set_config(&mut self, config: FormatConfig) {
        self.config = config;
    }
}

impl fmt::Display for DisplayF64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value.format_display(&self.config))
    }
}

impl From<f64> for DisplayF64 {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_integer_formatting() {
        assert_eq!(42.7.format_integer(), "43");
        assert_eq!((-42.7).format_integer(), "-43");
        assert_eq!(0.0.format_integer(), "0");
    }

    #[test]
    fn test_thousand_separators() {
        assert_eq!(1234.0.format_integer(), "1,234");
        assert_eq!(1234567.0.format_integer(), "1,234,567");
        assert_eq!((-1234567.0).format_integer(), "-1,234,567");
    }

    #[test]
    fn test_scientific_notation() {
        assert_eq!(1_500_000_000.0.format_scientific(), "1.5e+9");
        assert_eq!((-2_300_000_000.0).format_scientific(), "-2.3e+9");
        assert_eq!(0.000001.format_scientific(), "1e-6");
    }

    #[test]
    fn test_compact_formatting() {
        assert_eq!(1500.0.format_compact(), "1.5k");
        assert_eq!(2_500_000.0.format_compact(), "2.5M");
        assert_eq!(3_200_000_000.0.format_compact(), "3.2B");
        assert_eq!((-1500.0).format_compact(), "-1.5k");
    }

    #[test]
    fn test_special_values() {
        assert_eq!(f64::NAN.format_integer(), "NaN");
        assert_eq!(f64::INFINITY.format_integer(), "∞");
        assert_eq!(f64::NEG_INFINITY.format_integer(), "-∞");
    }

    #[test]
    fn test_very_small_values() {
        assert_eq!(0.00001.format_integer(), "0");
        assert_eq!((-0.00001).format_integer(), "0");
        assert_eq!(0.0001.format_integer(), "0"); // Exactly at threshold
    }

    #[test]
    fn test_convenience_functions() {
        assert_eq!(format::score(12345.7), "12,346");
        assert_eq!(format::money(2_500_000.0), "2.5M");
        assert_eq!(format::multiplier(2.5), "×2.5");
        assert_eq!(format::percentage(0.156), "15.6%");
    }

    #[test]
    fn test_display_f64_wrapper() {
        let display_value = DisplayF64::new(12345.7);
        assert_eq!(display_value.to_string(), "12,346");
        assert_eq!(display_value.value(), 12345.7);
    }

    #[test]
    fn test_large_number_fallback() {
        let large_value = 5_000_000_000.0;
        // Should fall back to scientific notation even in integer mode
        assert_eq!(large_value.format_integer(), "5e+9");
    }

    #[test]
    fn test_precision_preservation() {
        let precise_value = 123.456789123456789;
        // Internal precision should be preserved even if display is rounded
        let display = DisplayF64::new(precise_value);
        assert_eq!(display.value(), precise_value);
        assert_eq!(display.to_string(), "123"); // Displayed as integer
    }
}
