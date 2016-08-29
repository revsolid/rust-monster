// Copyright 2016 Revolution Solid & Contributors.
// author(s): sysnett
// rust-monster is licensed under a MIT License.

//! GA Random Numbers Util
//!
//! Wrapper around the rand crate that provides a Seeded
//! and Stateful Random Number Generator.
//!
//! Internally uses rand::XorShiftRng for speed purposes.
//!
//! # Examples
//!
//! GARandomCtx - Basic Use case
//! 
//! ```rust
//! extern crate rust_monster;
//! use rust_monster::ga; 
//! use rust_monster::ga::ga_random; 
//! 
//! fn main ()
//! {
//!     let seed : ga_random::GASeed = [1,2,3,4];
//!
//!     let mut ga_ctx = ga_random::GARandomCtx::from_seed(seed, String::from("MyRandomCtx")); 
//!
//!     println!("{:?}", ga_ctx.gen::<f32>());
//!
//!     println!("{:?}", ga_ctx);
//! }
//! ```
//!
use rand::{Rng, Rand, SeedableRng, XorShiftRng};
use rand::distributions::range::SampleRange;

use std::fmt;

pub type GASeed = [u32; 4];
pub struct GARandomCtx
{
    seed: GASeed,
    rng:  XorShiftRng,
    name: String,
    seeded: bool,
    values_generated: u32
}

impl GARandomCtx
{
// Constructors 
    pub fn new_unseeded(name: String) -> GARandomCtx
    {
        let std_rng = XorShiftRng::new_unseeded();
        GARandomCtx
        {
            seed: [0; 4],
            rng: std_rng,
            name: name,
            seeded: false,
            values_generated: 0
        }
    }

    pub fn from_seed(seed: GASeed, name: String) -> GARandomCtx
    {
        let std_rng = SeedableRng::from_seed(seed); 
        GARandomCtx
        {
            seed: seed,
            rng:  std_rng,
            name: name,
            seeded: true,
            values_generated: 0
        }
    }

// Random Values - Subset of the RNG Trait
    pub fn gen<T: Rand>(&mut self) -> T where Self: Sized
    {
        self.values_generated += 1;
        self.rng.gen()
    }

    pub fn gen_range<T: PartialOrd + SampleRange>(&mut self, low: T, high: T) -> T
    {
        self.values_generated += 1;
        self.rng.gen_range(low, high)
    }

    pub fn next_u32(&mut self) -> u32 { self.gen::<u32>() }
    pub fn next_u64(&mut self) -> u64 { self.gen::<u64>() }
    pub fn next_f32(&mut self) -> f32 { self.gen::<f32>() }
    pub fn next_f64(&mut self) -> f64 { self.gen::<f64>() }

    pub fn shuffle<T>(&mut self, values: &mut [T]) where Self: Sized, T: Copy
    {
        for i in 0..values.len()-2
        {
            let j = self.gen_range(i, values.len());
            let t = values[i];
            values[i] = values[j];
            values[j] = t;
        }
    }

// Random Values - GARandomCtx functions
    pub fn test_value<T: PartialOrd + Rand>(&mut self, value: T) -> bool 
    {
        self.gen::<T>() < value
    }


// Reset State
    pub fn reseed(&mut self, seed: GASeed)
    {
        self.seed = seed;
        self.seeded = true;
        self.reset();
    }

    pub fn reset(&mut self)
    {
        self.values_generated = 0;
        if self.seeded
        {
            self.rng.reseed(self.seed);
        }
        else
        {
            self.rng = XorShiftRng::new_unseeded(); 
        }
    }
}

impl fmt::Debug for GARandomCtx
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let seeded_str = if self.seeded
            {
                "Seeded"
            }
            else
            {
                "Not Seeded"
            };

        write!(f, "GARandomCtx {} - {} {{ seed: {:?}, values_generated: {:?} }}",
               self.name,
               seeded_str,
               self.seed,
               self.values_generated)
    }
}

////////////////////////////////////////
// Tests
#[cfg(test)]
mod test
{
    use super::{GASeed, GARandomCtx};
    use ::ga::ga_test::{ga_test_setup, ga_test_teardown};

    #[test]
    fn same_seed()
    {
        ga_test_setup("ga_random::same_seed");
        let seed : GASeed = [1,2,3,4];
        let mut ga_ctx = GARandomCtx::from_seed(seed, String::from("TestRandomCtx")); 
        let mut ga_ctx_2 = GARandomCtx::from_seed(seed, String::from("TestRandomCtx2")); 
        debug!("{:?}", ga_ctx);
        debug!("{:?}", ga_ctx_2);

        for _ in 0..100
        {
            assert_eq!(ga_ctx.gen::<f64>(), ga_ctx_2.gen::<f64>());
        }
        debug!("{:?}", ga_ctx);
        debug!("{:?}", ga_ctx_2);

        for _ in 0..100
        {
            assert_eq!(ga_ctx.gen::<u32>(), ga_ctx_2.gen::<u32>());
        }
        debug!("{:?}", ga_ctx);
        debug!("{:?}", ga_ctx_2);
        ga_test_teardown();
    }

    #[test]
    fn diff_seed()
    {
        ga_test_setup("ga_random::diff_seed");
        let seed_1 : GASeed = [1,2,3,4];
        let seed_2 : GASeed = [4,3,2,1];
        let mut ga_ctx = GARandomCtx::from_seed(seed_1, String::from("TestRandomCtx")); 
        let mut ga_ctx_2 = GARandomCtx::from_seed(seed_2, String::from("TestRandomCtx2")); 
        debug!("{:?}", ga_ctx);
        debug!("{:?}", ga_ctx_2);

        for _ in 0..100
        {
            assert!(ga_ctx.gen::<f32>() != ga_ctx_2.gen::<f32>());
        }
        debug!("{:?}", ga_ctx);
        debug!("{:?}", ga_ctx_2);

        for _ in 0..100
        {
            assert!(ga_ctx.gen::<u64>() != ga_ctx_2.gen::<u64>());
        }
        debug!("{:?}", ga_ctx);
        debug!("{:?}", ga_ctx_2);
        ga_test_teardown();
    }

    #[test]
    fn same_seed_different_types()
    {
        ga_test_setup("ga_random::same_seed_different_types");
        let seed_1 = [1; 4];
        let mut ga_ctx = GARandomCtx::from_seed(seed_1, String::from("TestRandomCtx")); 
        let mut ga_ctx_2 = GARandomCtx::from_seed(seed_1, String::from("TestRandomCtx")); 
        debug!("{:?}", ga_ctx.gen::<f32>()); 
        debug!("{:?}", ga_ctx_2.gen::<i8>()); 
        assert_eq!(ga_ctx.gen::<f32>(), ga_ctx_2.gen::<f32>());
        ga_test_teardown();
    }
}
