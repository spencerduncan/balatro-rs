/// DEPRECATED TEST MODULE: Square Joker was not part of original specification
///
/// Per Issue #644 analysis, SquareJoker was implemented as feature creep in PR #605,
/// violating the specification that only requested Castle, Wee, and Stuntman jokers.
///
/// These tests validated wrong behavior and should be removed to maintain specification compliance.
///
/// Original Issue #191 only requested:
/// - Castle Joker: suit-specific discard tracking with suit rotation
/// - Wee Joker: chips when 2s are scored
/// - Stuntman Joker: static chips + hand size reduction
///
/// SquareJoker was NOT requested and represents implementation outside specification.

#[cfg(test)]
mod deprecated_square_joker_tests {
    // NOTE: These tests are kept for historical reference but should not be extended
    // The Square joker functionality was not part of the original specification

    #[test]
    #[should_panic(expected = "Square joker deprecated - not in original specification")]
    fn test_square_joker_deprecated() {
        // This test documents that Square joker should not be used
        // as it was not part of the original Issue #191 specification
        panic!("Square joker deprecated - not in original specification");
    }
}
