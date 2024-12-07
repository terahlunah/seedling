use rand_core::{Error, RngCore, SeedableRng};
use rand_pcg::{Pcg32, Pcg64, Pcg64Mcg};

pub use rand_core;

pub type TreeRng32 = TreeRng<Pcg32>;
pub type TreeRng64 = TreeRng<Pcg64>;
pub type TreeRngFast = TreeRng<Pcg64Mcg>;

pub struct TreeRng<Rng> {
    pub seed: u64,
    rng: Rng,
}

impl<Rng: SeedableRng + RngCore> TreeRng<Rng> {
    pub fn new(seed: u64) -> TreeRng<Rng> {
        Self {
            seed,
            rng: Rng::seed_from_u64(seed),
        }
    }

    pub fn child(&self, index: u64) -> TreeRng<Rng> {
        let sub_seed = Rng::seed_from_u64(self.seed + index).next_u64();
        TreeRng::new(sub_seed)
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
