//! `SplitMix64` — the portable, byte-exact PRNG that makes the evolution
//! reproducible across every language binding. The bindings never implement a
//! random generator; they forward `command_json`, and the RNG lives here.

/// A `SplitMix64` generator (Vigna's reference constants).
#[derive(Debug, Clone)]
pub struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    /// Seed the generator.
    #[must_use]
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// The next 64-bit output.
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// A uniform integer in `[0, n)` via a fixed mul-shift (byte-exact).
    pub fn below(&mut self, n: u64) -> u64 {
        if n == 0 {
            return 0;
        }
        ((u128::from(self.next_u64()) * u128::from(n)) >> 64) as u64
    }

    /// A uniform float in `[0, 1)` with 53 bits of mantissa (fixed formula).
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / 9_007_199_254_740_992.0)
    }
}

#[cfg(test)]
mod tests {
    use super::SplitMix64;

    #[test]
    fn stream_is_seed_deterministic_and_seed_sensitive() {
        // Two generators with the same seed must produce identical streams (the
        // cross-language reproducibility contract); different seeds must not.
        let mut a = SplitMix64::new(1);
        let mut b = SplitMix64::new(1);
        let sa: Vec<u64> = (0..8).map(|_| a.next_u64()).collect();
        let sb: Vec<u64> = (0..8).map(|_| b.next_u64()).collect();
        assert_eq!(sa, sb);

        let mut c = SplitMix64::new(2);
        let sc: Vec<u64> = (0..8).map(|_| c.next_u64()).collect();
        assert_ne!(sa, sc);
    }

    #[test]
    fn below_stays_in_range() {
        let mut rng = SplitMix64::new(42);
        for _ in 0..1000 {
            assert!(rng.below(7) < 7);
        }
        assert_eq!(rng.below(0), 0);
        assert_eq!(rng.below(1), 0);
    }

    #[test]
    fn next_f64_in_unit_interval() {
        let mut rng = SplitMix64::new(7);
        for _ in 0..1000 {
            let x = rng.next_f64();
            assert!((0.0..1.0).contains(&x));
        }
    }
}
