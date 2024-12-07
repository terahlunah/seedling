//! **Seedling** is a Rust crate for creating hierarchical pseudo-random number generators (PRNGs) based on the `rand_pcg`
//! algorithm. It provides a simple way to organize RNGs into a tree-like structure, ensuring reproducible and independent
//! random sequences.

use rand_core::{Error, RngCore, SeedableRng};
use rand_pcg::{Pcg32, Pcg64, Pcg64Mcg};

pub use rand_core;

/// A type alias for a `TreeRng` using `Pcg32` for 32-bit random number generation.
pub type TreeRng32 = TreeRng<Pcg32>;

/// A type alias for a `TreeRng` using `Pcg64` for 64-bit random number generation.
pub type TreeRng64 = TreeRng<Pcg64>;

/// A type alias for a `TreeRng` using `Pcg64Mcg` for fast 64-bit random number generation.
pub type TreeRng64Fast = TreeRng<Pcg64Mcg>;

/// A hierarchical pseudo-random number generator (PRNG).
///
/// `TreeRng` organizes RNGs into a tree structure, allowing you to create independent
/// child RNGs from a parent RNG. This ensures stability and reproducibility in random number
/// generation across code changes.
///
/// # Type Parameters
///
/// - `Rng`: A PRNG type that implements both `SeedableRng` and `RngCore`.
pub struct TreeRng<Rng> {
    seed: u64,
    rng: Rng,
}

impl<Rng: SeedableRng + RngCore> TreeRng<Rng> {
    /// Creates a new `TreeRng` initialized with the given seed.
    pub fn new(seed: u64) -> TreeRng<Rng> {
        Self {
            seed,
            rng: Rng::seed_from_u64(seed),
        }
    }

    /// Creates a child `TreeRng` from this RNG using the given index.
    pub fn child(&self, index: u64) -> TreeRng<Rng> {
        let sub_seed = Rng::seed_from_u64(self.seed + index).next_u64();
        TreeRng::new(sub_seed)
    }

    /// Returns the seed used to initialize this RNG.
    pub fn seed(&self) -> u64 {
        self.seed
    }
}

impl<Rng: RngCore> RngCore for TreeRng<Rng> {
    fn next_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.rng.fill_bytes(dest);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.rng.try_fill_bytes(dest)
    }
}
