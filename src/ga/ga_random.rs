// TODO: COPYRIGHT, USE & AUTHORS
// TODO: RUST DOCS!

/// GA Random Numbers Util
///
/// Wrapper around the rand crate
///
use rand::{random, Rng, thread_rng};
use rand::distributions::range::SampleRange;

pub fn ga_random_float() -> f32
{
    random()
}

// Return a random number from range [low, high).
pub fn ga_random_range<T: PartialOrd + SampleRange>(low: T, high: T) -> T
{
    thread_rng().gen_range(low, high)
}

pub fn ga_random_float_test(v: f32) -> bool
{
    ga_random_float() < v
}
