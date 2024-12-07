# Seedling

---

[![Crates.io](https://img.shields.io/crates/v/seedling.svg?color=orange)](https://crates.io/crates/seedling)
[![docs.rs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/seedling)
[![CI Checks](https://github.com/terahlunah/seedling/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/terahlunah/seedling/actions/workflows/rust.yml)

---

**Seedling** is a Rust crate for creating hierarchical pseudo-random number generators (PRNGs).
It provides a simple way to organize RNGs into a tree-like structure, ensuring reproducible and independent random
sequences.

```toml
[dependencies]
seedling = "1.0.1"
```

## Features

- **Hierarchical PRNGs**: Organize RNGs in a tree structure with independent child generators.
- **Stable Outputs**: Random sequences remain stable across code changes by using consistent subseeds.
- **Flexible RNGs**: Supports 32-bit (`TreeRng32`), 64-bit (`TreeRng64`), and fast (`TreeRngFast`) PRNGs out of the box.
- **Standard Traits**: Implements `RngCore` and integrates with the `rand` ecosystem (including `rand::Rng` trait).

## Example

```rust
// `seedling` reexports `rand_core` for convenience
use seedling::{rand_core::RngCore, TreeRng64};

// Use `rand::Rng` for a higher level api
use rand::Rng;

fn main() {
    // It is recommended to use constants for subtree indexes to prevent accidental index changes
    const FEATURE_1_INDEX: u64 = 1;
    const FEATURE_2_INDEX: u64 = 2;

    // Indexes are arbitrary and just need to be unique per `TreeRng`
    const FEATURE_2_1_INDEX: u64 = 1;

    // The root seed of the tree
    let root_seed = 42;

    // Create a new top-level RNG
    let mut root_rng = TreeRng64::new(root_seed);

    // Create a child RNG with a constant index
    let mut feature_1_rng = root_rng.child(FEATURE_1_INDEX);
    println!("Feature 1 RNG u64: {}", feature_1_rng.next_u64());

    // Create another child RNG with a different index
    let mut feature_2_rng = root_rng.child(FEATURE_2_INDEX);
    println!("Feature 2 RNG u64: {}", feature_2_rng.next_u64());

    // Children of `TreeRng` are themselves `TreeRng` and the hierarchy can grow to arbitrary depth
    let feature_2_1_rng = feature_2_rng.child(FEATURE_2_1_INDEX);
}
```

## Provided RNG

`seedling` includes out of the box support for the `rand_pcg` generators via the following type aliases.

- `TreeRng32`: A 32-bit RNG based on `Pcg32`.
- `TreeRng64`: A 64-bit RNG based on `Pcg64`.
- `TreeRngFast`: A fast RNG based on `Pcg64Mcg`.

`seedling` is also easily extensible and `TreeRng<T>` will work with any type implementing `RngCore` and `SeedableRng`
from `rand_core`.
