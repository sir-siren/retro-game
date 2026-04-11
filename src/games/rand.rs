#[must_use]
pub fn fast_rand(seed: u64) -> u64 {
    let mut x = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    x
}
