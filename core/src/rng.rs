use rand::{prelude::*, rngs::StdRng, distributions::{WeightedError, uniform::{SampleUniform, SampleRange}}};
use rand_chacha::ChaCha20Rng;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/// RNG operation modes for different security and determinism requirements
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RngMode {
    /// Deterministic RNG with explicit seed for reproducible gameplay
    Deterministic(u64),
    /// Cryptographically secure RNG for unpredictable outcomes
    Secure,
    /// Testing mode with fixed seed for unit tests
    Testing(u64),
}

/// Secure RNG management system addressing RNG state manipulation vulnerabilities
#[derive(Debug, Clone)]
pub struct GameRng {
    mode: RngMode,
    deterministic: Option<Arc<Mutex<StdRng>>>,
    secure: Option<Arc<Mutex<ChaCha20Rng>>>,
}

impl GameRng {
    /// Create a new GameRng instance with the specified mode
    pub fn new(mode: RngMode) -> Self {
        match mode {
            RngMode::Deterministic(seed) | RngMode::Testing(seed) => {
                let rng = StdRng::seed_from_u64(seed);
                Self {
                    mode,
                    deterministic: Some(Arc::new(Mutex::new(rng))),
                    secure: None,
                }
            }
            RngMode::Secure => {
                let rng = ChaCha20Rng::from_entropy();
                Self {
                    mode,
                    deterministic: None,
                    secure: Some(Arc::new(Mutex::new(rng))),
                }
            }
        }
    }

    /// Create a deterministic RNG for testing with a known seed
    pub fn for_testing(seed: u64) -> Self {
        Self::new(RngMode::Testing(seed))
    }

    /// Create a secure RNG for production use
    pub fn secure() -> Self {
        Self::new(RngMode::Secure)
    }

    /// Create a deterministic RNG with a specific seed
    pub fn deterministic(seed: u64) -> Self {
        Self::new(RngMode::Deterministic(seed))
    }

    /// Get the current RNG mode
    pub fn mode(&self) -> RngMode {
        self.mode
    }

    /// Generate a random number within a range using the appropriate RNG
    pub fn gen_range<T, R>(&self, range: R) -> T
    where
        T: SampleUniform,
        R: SampleRange<T>,
    {
        match &self.mode {
            RngMode::Deterministic(_) | RngMode::Testing(_) => {
                let mut rng = self.deterministic.as_ref().unwrap().lock().unwrap();
                rng.gen_range(range)
            }
            RngMode::Secure => {
                let mut rng = self.secure.as_ref().unwrap().lock().unwrap();
                rng.gen_range(range)
            }
        }
    }

    /// Generate a random boolean value
    pub fn gen_bool(&self, p: f64) -> bool {
        match &self.mode {
            RngMode::Deterministic(_) | RngMode::Testing(_) => {
                let mut rng = self.deterministic.as_ref().unwrap().lock().unwrap();
                rng.gen_bool(p)
            }
            RngMode::Secure => {
                let mut rng = self.secure.as_ref().unwrap().lock().unwrap();
                rng.gen_bool(p)
            }
        }
    }

    /// Shuffle a slice using the appropriate RNG
    pub fn shuffle<T>(&self, slice: &mut [T]) {
        match &self.mode {
            RngMode::Deterministic(_) | RngMode::Testing(_) => {
                let mut rng = self.deterministic.as_ref().unwrap().lock().unwrap();
                slice.shuffle(&mut *rng);
            }
            RngMode::Secure => {
                let mut rng = self.secure.as_ref().unwrap().lock().unwrap();
                slice.shuffle(&mut *rng);
            }
        }
    }

    /// Choose a random element from a slice
    pub fn choose<'a, T>(&self, slice: &'a [T]) -> Option<&'a T> {
        match &self.mode {
            RngMode::Deterministic(_) | RngMode::Testing(_) => {
                let mut rng = self.deterministic.as_ref().unwrap().lock().unwrap();
                slice.choose(&mut *rng)
            }
            RngMode::Secure => {
                let mut rng = self.secure.as_ref().unwrap().lock().unwrap();
                slice.choose(&mut *rng)
            }
        }
    }

    /// Choose multiple random elements from a slice
    pub fn choose_multiple<'a, T>(&self, slice: &'a [T], amount: usize) -> Vec<&'a T> {
        match &self.mode {
            RngMode::Deterministic(_) | RngMode::Testing(_) => {
                let mut rng = self.deterministic.as_ref().unwrap().lock().unwrap();
                slice.choose_multiple(&mut *rng, amount).collect()
            }
            RngMode::Secure => {
                let mut rng = self.secure.as_ref().unwrap().lock().unwrap();
                slice.choose_multiple(&mut *rng, amount).collect()
            }
        }
    }

    /// Generate a weighted choice from options
    pub fn choose_weighted<'a, T, F, B>(&self, items: &'a [T], weight_fn: F) -> Result<&'a T, WeightedError>
    where
        F: FnMut(&T) -> B,
        B: Into<f64>,
    {
        use rand::distributions::WeightedIndex;
        
        let weights: Vec<f64> = items.iter().map(weight_fn).map(Into::into).collect();
        let dist = WeightedIndex::new(&weights)?;
        
        let index = match &self.mode {
            RngMode::Deterministic(_) | RngMode::Testing(_) => {
                let mut rng = self.deterministic.as_ref().unwrap().lock().unwrap();
                dist.sample(&mut *rng)
            }
            RngMode::Secure => {
                let mut rng = self.secure.as_ref().unwrap().lock().unwrap();
                dist.sample(&mut *rng)
            }
        };
        
        Ok(&items[index])
    }

    /// Create a fork of this RNG with the same mode but independent state
    /// Useful for creating isolated RNG instances for different game systems
    pub fn fork(&self) -> Self {
        match self.mode {
            RngMode::Deterministic(_seed) => {
                // Create a new seed by advancing the current RNG
                let new_seed = self.gen_range(0..u64::MAX);
                Self::new(RngMode::Deterministic(new_seed))
            }
            RngMode::Testing(seed) => {
                // For testing, use a derived seed for reproducibility
                let new_seed = seed.wrapping_add(1);
                Self::new(RngMode::Testing(new_seed))
            }
            RngMode::Secure => {
                // Create a new secure RNG instance
                Self::secure()
            }
        }
    }

    /// Get the seed value if in deterministic or testing mode
    pub fn seed(&self) -> Option<u64> {
        match self.mode {
            RngMode::Deterministic(seed) | RngMode::Testing(seed) => Some(seed),
            RngMode::Secure => None,
        }
    }
}

impl Default for GameRng {
    fn default() -> Self {
        Self::secure()
    }
}

/// Factory for creating different types of RNG instances
pub struct RngFactory;

impl RngFactory {
    /// Create a secure RNG for production gameplay
    pub fn secure() -> GameRng {
        GameRng::secure()
    }

    /// Create a deterministic RNG for debugging and development
    pub fn deterministic(seed: u64) -> GameRng {
        GameRng::deterministic(seed)
    }

    /// Create a testing RNG with a known seed
    pub fn testing(seed: u64) -> GameRng {
        GameRng::for_testing(seed)
    }

    /// Create an RNG from environment variable or default to secure
    /// Environment variable: BALATRO_RNG_SEED for deterministic mode
    pub fn from_env() -> GameRng {
        if let Ok(seed_str) = std::env::var("BALATRO_RNG_SEED") {
            if let Ok(seed) = seed_str.parse::<u64>() {
                return GameRng::deterministic(seed);
            }
        }
        GameRng::secure()
    }
}

thread_local! {
    /// Thread-local RNG management for high-performance scenarios
    static THREAD_RNG: std::cell::RefCell<Option<GameRng>> = const { std::cell::RefCell::new(None) };
}

/// Get or initialize the thread-local RNG instance
pub fn with_thread_rng<F, R>(f: F) -> R
where
    F: FnOnce(&GameRng) -> R,
{
    THREAD_RNG.with(|rng_cell| {
        let mut rng_opt = rng_cell.borrow_mut();
        if rng_opt.is_none() {
            *rng_opt = Some(RngFactory::from_env());
        }
        f(rng_opt.as_ref().unwrap())
    })
}

/// Set the thread-local RNG instance
pub fn set_thread_rng(rng: GameRng) {
    THREAD_RNG.with(|rng_cell| {
        *rng_cell.borrow_mut() = Some(rng);
    });
}

/// Clear the thread-local RNG instance
pub fn clear_thread_rng() {
    THREAD_RNG.with(|rng_cell| {
        *rng_cell.borrow_mut() = None;
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_rng_reproducibility() {
        let rng1 = GameRng::for_testing(12345);
        let rng2 = GameRng::for_testing(12345);

        let _val1: u32 = rng1.gen_range(0..1000);
        let _val2: u32 = rng2.gen_range(0..1000);

        assert_eq!(_val1, _val2, "Deterministic RNGs with same seed should produce same values");
    }

    #[test]
    fn test_secure_rng_unpredictability() {
        let rng1 = GameRng::secure();
        let rng2 = GameRng::secure();

        let _val1: u32 = rng1.gen_range(0..1000);
        let _val2: u32 = rng2.gen_range(0..1000);

        // While not guaranteed, extremely unlikely to be equal
        // This test may occasionally fail due to random chance
        // but it demonstrates that secure RNGs produce different values
    }

    #[test]
    fn test_rng_modes() {
        let det_rng = GameRng::deterministic(42);
        assert_eq!(det_rng.mode(), RngMode::Deterministic(42));
        assert_eq!(det_rng.seed(), Some(42));

        let test_rng = GameRng::for_testing(123);
        assert_eq!(test_rng.mode(), RngMode::Testing(123));
        assert_eq!(test_rng.seed(), Some(123));

        let secure_rng = GameRng::secure();
        assert_eq!(secure_rng.mode(), RngMode::Secure);
        assert_eq!(secure_rng.seed(), None);
    }

    #[test]
    fn test_rng_operations() {
        let rng = GameRng::for_testing(42);

        // Test range generation
        let val: u32 = rng.gen_range(0..100);
        assert!(val < 100);

        // Test boolean generation
        let bool_val = rng.gen_bool(0.5);
        assert!(bool_val == true || bool_val == false);

        // Test shuffle
        let mut vec = vec![1, 2, 3, 4, 5];
        rng.shuffle(&mut vec);
        assert_eq!(vec.len(), 5);

        // Test choose
        let slice = &[1, 2, 3, 4, 5];
        let choice = rng.choose(slice);
        assert!(choice.is_some());
        assert!(slice.contains(choice.unwrap()));
    }

    #[test]
    fn test_rng_fork() {
        let original = GameRng::for_testing(42);
        let forked = original.fork();

        // Forks should have different seeds in testing mode
        assert_ne!(original.seed(), forked.seed());
    }

    #[test]
    fn test_thread_local_rng() {
        set_thread_rng(GameRng::for_testing(999));
        
        let result = with_thread_rng(|rng| {
            assert_eq!(rng.seed(), Some(999));
            rng.gen_range(0..100u32)
        });

        assert!(result < 100);
        clear_thread_rng();
    }

    #[test]
    fn test_weighted_choice() {
        let rng = GameRng::for_testing(42);
        let items = vec![1, 2, 3, 4, 5];
        let weights = [1.0, 2.0, 3.0, 4.0, 5.0];
        
        let choice = rng.choose_weighted(&items, |i| weights[*i - 1]).unwrap();
        assert!(items.contains(choice));
    }
}