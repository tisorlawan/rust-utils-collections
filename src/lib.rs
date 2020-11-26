use std::cell::Cell;
use std::num::Wrapping;

/// Returns a random number in the interval `0..n`.
pub fn fast_random(n: usize) -> i32 {
    thread_local! {
        static RNG: Cell<Wrapping<u32>> = Cell::new(Wrapping(1));
    }

    RNG.with(|rng| {
        // This is the 32-bit variant of Xorshift: https://en.wikipedia.org/wiki/Xorshift
        let mut x = rng.get();
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        rng.set(x);

        // This is a fast alternative to `x % n`:
        // https://lemire.me/blog/2016/06/27/a-fast-alternative-to-the-modulo-reduction/
        ((x.0 as u64).wrapping_mul(n as u64) >> 32) as i32
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_random() {
        let mut r: [i32; 100] = [0; 100];

        for _ in 0..1000 {
            r[fast_random(100) as usize] += 1;
        }

        assert!(r.iter().all(|n| *n > 0));
    }
}
