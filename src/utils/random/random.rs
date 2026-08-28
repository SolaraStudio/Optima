use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

pub struct Random {
    rng: StdRng,
}

impl Random {
    pub fn new() -> Self {
        Self {
            rng: StdRng::from_entropy(),
        }
    }

    pub fn seeded(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
        }
    }

    pub fn next_bool(&mut self) -> bool {
        self.rng.gen()
    }

    pub fn next_f32(&mut self) -> f32 {
        self.rng.gen()
    }

    pub fn next_f64(&mut self) -> f64 {
        self.rng.gen()
    }

    pub fn next_i32(&mut self) -> i32 {
        self.rng.gen()
    }

    pub fn next_u32(&mut self) -> u32 {
        self.rng.gen()
    }

    pub fn next_usize(&mut self) -> usize {
        self.rng.gen()
    }

    pub fn next_f32_range(&mut self, min: f32, max: f32) -> f32 {
        min + self.rng.gen::<f32>() * (max - min)
    }

    pub fn next_i32_range(&mut self, min: i32, max: i32) -> i32 {
        self.rng.gen_range(min..=max)
    }

    pub fn choose<'a, T>(&mut self, items: &'a [T]) -> Option<&'a T> {
        if items.is_empty() {
            None
        } else {
            Some(&items[self.rng.gen_range(0..items.len())])
        }
    }

    pub fn shuffle<T>(&mut self, items: &mut [T]) {
        use rand::seq::SliceRandom;
        items.shuffle(&mut self.rng);
    }

    pub fn seed(&mut self, seed: u64) {
        self.rng = StdRng::seed_from_u64(seed);
    }
}

impl Default for Random {
    fn default() -> Self {
        Self::new()
    }
}
