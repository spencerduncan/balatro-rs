# Testing Framework Salvage - Implementation Notes

## Extraction Strategy for PR #779

### Quick Reference Commands

```bash
# Fetch PR #779 diff to local file
gh pr diff 779 --repo spencerduncan/balatro-rs > pr779.diff

# Extract specific file content from PR
gh api repos/spencerduncan/balatro-rs/pulls/779/contents/web-debug-ui/tests/common/fixtures.rs | \
  jq -r '.content' | base64 -d > extracted_fixtures.rs

# View PR file list
gh api repos/spencerduncan/balatro-rs/pulls/779/files | jq -r '.[].filename'

# Get patch for specific file
gh api repos/spencerduncan/balatro-rs/pulls/779/files | \
  jq -r '.[] | select(.filename == "web-debug-ui/tests/common/fixtures.rs") | .patch'
```

## Day 1: Core Testing Infrastructure

### Extraction Focus
**Source Files from PR #779:**
- `web-debug-ui/tests/common/fixtures.rs` (424 lines)
- `web-debug-ui/tests/common/assertions.rs` (341 lines)
- `web-debug-ui/tests/common/mod.rs` (87 lines)

### Adaptation Required

1. **Namespace Changes**
```rust
// Original (web-debug-ui context)
use crate::domain::entities::GameSession;
use crate::infrastructure::storage::MemoryStore;

// Adapt to (core library context)
use balatro_rs::{Game, Config, Deck, Card};
use balatro_rs::joker::{Joker, JokerId};
```

2. **Test Fixture Patterns to Extract**
```rust
// Pattern 1: Game Factory
pub fn create_test_game() -> Game {
    Game::new(Config {
        ante: 1,
        seed: Some(12345), // Deterministic for tests
        hands: 4,
        discards: 3,
        money: 4,
        ..Default::default()
    })
}

// Pattern 2: Joker Factory
pub fn create_test_joker_set(ids: Vec<JokerId>) -> Vec<Box<dyn Joker>> {
    ids.into_iter()
        .filter_map(|id| JokerFactory::create(id).ok())
        .collect()
}

// Pattern 3: Deck Builder
pub fn create_stacked_deck(cards: Vec<Card>) -> Deck {
    let mut deck = Deck::french_deck();
    // Stack the deck for deterministic testing
    deck.set_order(cards);
    deck
}
```

3. **Assertion Helpers to Create**
```rust
pub mod assertions {
    use approx::assert_relative_eq;

    pub fn assert_score_valid(score: f64, expected: f64) {
        assert_relative_eq\!(score, expected, epsilon = 0.01);
    }

    pub fn assert_action_in_space(game: &Game, action: &Action) {
        let actions = game.gen_actions();
        assert\!(
            actions.contains(action),
            "Action {:?} not in valid action space",
            action
        );
    }
}
```

## Day 2: Advanced Testing Features

### Extraction Focus
**Source Files from PR #779:**
- `web-debug-ui/tests/common/properties.rs` (462 lines)
- `web-debug-ui/tests/common/performance.rs` (506 lines)

### Key Patterns to Salvage

1. **Property-Based Testing Structure**
```rust
use proptest::prelude::*;

// Strategy for generating test games
prop_compose\! {
    fn arb_game_config()
        (ante in 1..10u8,
         hands in 1..8u8,
         discards in 1..8u8,
         money in 0..100i32,
         seed in any::<u64>())
        -> Config {
        Config {
            ante,
            hands,
            discards,
            money,
            seed: Some(seed),
            ..Default::default()
        }
    }
}

// Property test example
proptest\! {
    #[test]
    fn game_state_is_valid_after_any_action(
        config in arb_game_config(),
        action_count in 1..100usize,
    ) {
        let mut game = Game::new(config);
        for _ in 0..action_count {
            let actions = game.gen_actions();
            if let Some(action) = actions.first() {
                game.handle_action(action.clone());
                // Verify invariants
                prop_assert\!(game.is_valid());
            }
        }
    }
}
```

2. **Memory Leak Detection Pattern**
```rust
#[cfg(test)]
mod memory_tests {
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct TrackingAllocator;
    static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

    unsafe impl GlobalAlloc for TrackingAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            ALLOCATED.fetch_add(layout.size(), Ordering::SeqCst);
            System.alloc(layout)
        }

        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            ALLOCATED.fetch_sub(layout.size(), Ordering::SeqCst);
            System.dealloc(ptr, layout)
        }
    }

    #[test]
    fn test_no_memory_leak() {
        let initial = ALLOCATED.load(Ordering::SeqCst);

        // Run test scenario
        for _ in 0..1000 {
            let game = Game::new(Config::default());
            drop(game);
        }

        let final_mem = ALLOCATED.load(Ordering::SeqCst);
        assert\!(final_mem - initial < 10_240); // <10KB tolerance
    }
}
```

## Day 3: Mock Framework

### Extraction Focus
**Source Files from PR #779:**
- `web-debug-ui/tests/common/mocks.rs` (349 lines)

### Mock Patterns to Implement

1. **RNG Mock for Deterministic Testing**
```rust
use mockall::*;

#[automock]
pub trait RngProvider: Send + Sync {
    fn next_u64(&mut self) -> u64;
    fn gen_range(&mut self, low: u64, high: u64) -> u64;
    fn shuffle<T>(&mut self, items: &mut [T]);
}

pub fn create_deterministic_rng(sequence: Vec<u64>) -> MockRngProvider {
    let mut mock = MockRngProvider::new();
    let mut seq = sequence.into_iter().cycle();

    mock.expect_next_u64()
        .returning(move || seq.next().unwrap());

    mock
}
```

2. **Mock Builder Pattern**
```rust
pub struct TestGameBuilder {
    config: Config,
    rng: Option<Box<dyn RngProvider>>,
    jokers: Vec<JokerId>,
    deck_order: Option<Vec<Card>>,
}

impl TestGameBuilder {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            rng: None,
            jokers: Vec::new(),
            deck_order: None,
        }
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.config.seed = Some(seed);
        self
    }

    pub fn with_jokers(mut self, jokers: Vec<JokerId>) -> Self {
        self.jokers = jokers;
        self
    }

    pub fn with_deterministic_rng(mut self, sequence: Vec<u64>) -> Self {
        self.rng = Some(Box::new(create_deterministic_rng(sequence)));
        self
    }

    pub fn build(self) -> Game {
        let mut game = Game::new(self.config);
        // Apply mock configuration
        if let Some(rng) = self.rng {
            game.set_rng_provider(rng);
        }
        // Add jokers
        for joker_id in self.jokers {
            game.add_joker(joker_id);
        }
        game
    }
}
```

## Day 4: CI/CD Enhancement

### Files to Create
1. `.github/workflows/test-coverage.yml`
2. `.github/workflows/ci-enhancements.yml`

### Coverage Workflow Template
```yaml
name: Test Coverage Enforcement

on:
  pull_request:
    types: [opened, synchronize, reopened]
  push:
    branches: [main]

jobs:
  coverage:
    name: Enforce 90% Coverage
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          components: llvm-tools-preview

      - name: Install cargo-llvm-cov
        run: cargo install cargo-llvm-cov

      - name: Generate Coverage Report
        run: |
          cargo llvm-cov clean --workspace
          cargo llvm-cov test --all-features --workspace --lcov --output-path lcov.info
          cargo llvm-cov report --summary-only

      - name: Check Coverage Threshold
        run: |
          COVERAGE=$(cargo llvm-cov report --summary-only | grep TOTAL | awk '{print $10}' | sed 's/%//')
          echo "::notice title=Coverage::Current coverage is ${COVERAGE}%"

          if (( $(echo "$COVERAGE < 90" | bc -l) )); then
            echo "::error title=Coverage Failed::Coverage ${COVERAGE}% is below 90% threshold"
            exit 1
          fi

      - name: Upload Coverage to Codecov
        uses: codecov/codecov-action@v4
        with:
          files: ./lcov.info
          fail_ci_if_error: true
```

## Common Pitfalls to Avoid

1. **Don't Break Existing Tests**
   - Run full test suite after each change
   - Ensure backward compatibility

2. **Avoid Over-Engineering**
   - Extract only what's proven valuable
   - Keep patterns simple and reusable

3. **Maintain CI Stability**
   - Test CI changes in draft PRs first
   - Have rollback plan for workflow changes

4. **Documentation is Critical**
   - Document all test utilities
   - Provide examples for each pattern
   - Update README with testing guide

## Validation Checklist for Each PR

### Before Creating PR
- [ ] All tests pass locally
- [ ] No clippy warnings
- [ ] Code formatted with rustfmt
- [ ] Documentation added for new utilities
- [ ] Examples included in doc comments

### PR Description Template
```markdown
## Salvage Day X: [Component Name]

Part of #907 - Testing Framework Salvage

### What This PR Adds
- Brief description of salvaged components
- Line count salvaged
- Key patterns implemented

### Testing
- How the new code was tested
- Coverage metrics if applicable

### Documentation
- Links to updated docs
- Usage examples

### Next Steps
- What comes in next salvage PR
- Any blockers or dependencies
```

## Success Metrics Tracking

| Day | Target Lines | Actual Lines | Coverage Impact | CI Status |
|-----|--------------|--------------|-----------------|-----------|
| 1   | ~1,200       | TBD          | TBD             | TBD       |
| 2   | ~1,100       | TBD          | TBD             | TBD       |
| 3   | ~600         | TBD          | TBD             | TBD       |
| 4   | ~200         | TBD          | TBD             | TBD       |

## References

- Original PR: https://github.com/spencerduncan/balatro-rs/pull/779
- Parent Issue: https://github.com/spencerduncan/balatro-rs/issues/907
- Child Issues:
  - Day 1: https://github.com/spencerduncan/balatro-rs/issues/916
  - Day 2: https://github.com/spencerduncan/balatro-rs/issues/918
  - Day 3: https://github.com/spencerduncan/balatro-rs/issues/919
  - Day 4: https://github.com/spencerduncan/balatro-rs/issues/920

---

**Ready for Implementation**: These notes provide concrete extraction strategies for each day's salvage work.
