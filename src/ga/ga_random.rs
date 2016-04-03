// TODO: COPYRIGHT, USE & AUTHORS
// TODO: RUST DOCS!


/// GA Random Numbers Util
///
/// Wrapper around the rand crate
///
pub mod rand {
    use rand::random;
    pub fn ga_random_float() -> f32
    {
        random()
    }

    pub fn ga_random_float_test(v: f32) -> bool
    {
        ga_random_float() < v
    }
}
