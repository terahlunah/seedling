use seedling::{rand_core::RngCore, TreeRng64};

#[test]
fn test_basic_determinism() {
    let mut rng_1 = TreeRng64::new(42);
    let mut rng_2 = TreeRng64::new(42);
    assert_eq!(rng_1.next_u64(), rng_2.next_u64());
    assert_eq!(rng_1.next_u64(), rng_2.next_u64());
    assert_eq!(rng_1.next_u64(), rng_2.next_u64());

    let mut rng_1 = TreeRng64::new(69);
    let mut rng_2 = TreeRng64::new(69);
    assert_eq!(rng_1.next_u64(), rng_2.next_u64());
    assert_eq!(rng_1.next_u64(), rng_2.next_u64());
    assert_eq!(rng_1.next_u64(), rng_2.next_u64());
}

#[test]
fn test_basic_randomness() {
    let mut rng_1 = TreeRng64::new(42);
    let mut rng_2 = TreeRng64::new(69);
    assert_ne!(rng_1.next_u64(), rng_2.next_u64());
    assert_ne!(rng_1.next_u64(), rng_2.next_u64());
    assert_ne!(rng_1.next_u64(), rng_2.next_u64());

    let mut rng_1 = TreeRng64::new(1);
    let mut rng_2 = TreeRng64::new(2);
    assert_ne!(rng_1.next_u64(), rng_2.next_u64());
    assert_ne!(rng_1.next_u64(), rng_2.next_u64());
    assert_ne!(rng_1.next_u64(), rng_2.next_u64());
}

#[test]
fn test_child_basic_determinism() {
    let rng_1 = TreeRng64::new(42);
    let rng_2 = TreeRng64::new(42);

    assert_eq!(rng_1.child(0).seed(), rng_2.child(0).seed());
    assert_eq!(rng_1.child(1).seed(), rng_2.child(1).seed());

    let mut child_1 = rng_1.child(0);
    let mut child_2 = rng_2.child(0);

    assert_eq!(child_1.next_u64(), child_2.next_u64());
    assert_eq!(child_1.next_u64(), child_2.next_u64());
    assert_eq!(child_1.next_u64(), child_2.next_u64());
}

#[test]
fn test_basic_child_randomness() {
    let rng = TreeRng64::new(42);

    let mut rng_1 = rng.child(0);
    let mut rng_2 = rng.child(1);
    assert_ne!(rng_1.next_u64(), rng_2.next_u64());
    assert_ne!(rng_1.next_u64(), rng_2.next_u64());
    assert_ne!(rng_1.next_u64(), rng_2.next_u64());
}

#[test]
fn test_child_creation_isolation() {
    let mut rng_1 = TreeRng64::new(42);
    let mut rng_2 = TreeRng64::new(42);

    let v1 = rng_1.next_u64();
    assert_eq!(rng_1.child(0).seed(), rng_2.child(0).seed());
    let v2 = rng_2.next_u64();
    assert_eq!(v1, v2);
    assert_eq!(rng_1.child(1).seed(), rng_2.child(1).seed());
}
