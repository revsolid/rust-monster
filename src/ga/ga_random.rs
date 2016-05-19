// TODO: COPYRIGHT, USE & AUTHORS
// TODO: RUST DOCS!

/// GA Random Numbers Util
///
/// Wrapper around the rand crate
///
use rand::{random, XorShiftRng, Rng, SeedableRng, thread_rng};
use rand::distributions::range::SampleRange;

const kGASeedSize : usize = 4;


/// GARandomCtx
///
/// Seeded and Stateful Random Number Generator.
/// Uses rand::XorShiftRng for speed purposes.
///
struct GARandomCtx
{
    seed: [u32; kGASeedSize],
    rng:  XorShiftRng,
    name: String,
    seeded: bool,
    values_genarated: u32
}

impl GARandomCtx
{
// Constructors 
    pub fn new_unseeded(name: String) -> GARandomCtx
    {
        let std_rng = XorShiftRng::new_unseeded();
        GARandomCtx
        {
            seed: [0 ; kGASeedSize],
            rng: std_rng,
            name: name,
            seeded: false 
        }
    }

    pub fn from_seed(seed: [u32; kGASeedSize], name : String) -> GARandomCtx
    {
        let std_rng = SeedableRng::from_seed(seed); 
        GARandomCtx
        {
            seed: seed,
            rng:  std_rng,
            name: name,
            seeded: true
        }
    }

// Random Values
    pub fn random_float(&mut self) -> f32
    {
        self.rng.gen::<f32>() 
    }

    pub fn random_float_test(&mut self, v: f32) -> bool
    {
        self.random_float() < v
    }

    pub fn random_range<T: PartialOrd + SampleRange>(&mut self, low: T, high: T) -> T
    {
        self.rng.gen_range(low, high)
    }


// Reset State
    pub fn reseed(&mut self, seed: [u32; kGASeedSize])
    {
        self.seed = seed;
        self.rng.reseed(self.seed);
    }

    pub fn reset(&mut self)
    {
        if self.seeded
        {
            self.rng = SeedableRng::from_seed(self.seed);
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
        write!(f, "GARandomCtx {{ x: {}, y: {} }}", self.x, self.y)
    }
}


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
