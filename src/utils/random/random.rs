use rand::distributions::{Distribution, Standard};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub struct Random {
    rng: StdRng,
}

impl Random {
    pub fn new() -> Self {
        Random {
            rng: StdRng::from_entropy(),
        }
    }

    pub fn seeded(seed: u64) -> Self {
        Random {
            rng: StdRng::seed_from_u64(seed),
        }
    }

    pub fn next_bool(&mut self) -> bool {
        Standard.sample(&mut self.rng)
    }

    pub fn next_f32(&mut self) -> f32 {
        Standard.sample(&mut self.rng)
    }

    pub fn next_f64(&mut self) -> f64 {
        Standard.sample(&mut self.rng)
    }

    pub fn next_i32(&mut self) -> i32 {
        Standard.sample(&mut self.rng)
    }

    pub fn next_u32(&mut self) -> u32 {
        Standard.sample(&mut self.rng)
    }

    pub fn next_usize(&mut self) -> usize {
        Standard.sample(&mut self.rng)
    }

    pub fn next_f32_range(&mut self, min: f32, max: f32) -> f32 {
        min + self.rng.gen_range::<f32, _>(0.0..1.0) * (max - min)
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
        Random::new()
    }
}
