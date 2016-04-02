// TODO: COPYRIGHT, USE & AUTHORS

/// Bit Flags for Genetic Algorithm Configuration 
/// 
///
bitflags!
{
    pub flags GAFlags: u32
    {
        const DEBUG_FLAG = 0b00000001
    }
}
impl Default for GAFlags
{
    fn default() -> GAFlags { GAFlags {bits : 0} }
}


// Genetic Algorithm Configuration
// TODO: RUST DOCS!
pub trait GAConfig
{
    fn flags(&self) -> GAFlags;
    fn max_generations(&self) -> i32;
    fn percentage_crossover(&self) -> f32;
    fn probability_mutation(&self) -> f32;
}


// Genetic Algorithm Solution
// TODO: RUST DOCS!
pub trait GASolution
{
    fn evaluate(&mut self) -> f32;
    fn crossover(&self, other : &Self) -> &Self;
    fn mutate(&mut self);
    fn fitness(&self) -> f32;
}

// Genetic Algorithm Population
// TODO: RUST DOCS!
// TODO: Move other traits to use population
pub struct GAPopulation<T: GASolution>
{
    population : Vec<T>
}

impl<T: GASolution> GAPopulation<T>
{
}


// Genetic Algorithm Solution Factory
// TODO: RUST DOCS!
pub trait GAFactory<T: GASolution>
{
    fn initial_population(&mut self) -> Vec<T>
    {
        let v: Vec<T> = vec![];
        v
    }
}


// Genetic Algorithm
// TODO: RUST DOCS!
pub trait GeneticAlgorithm<T: GASolution>
{
    // GENERIC GA METHODS - Should not be overriden frequently
    fn initialize(&mut self)
    {
        debug!("Genetic Algorithm - Initialized");
        self.initialize_internal()
    }

    fn step(&mut self) -> i32
    { 
        debug!("Genetic Algorithm - Step");
        self.step_internal()
    }

    fn done(&mut self) -> bool
    {
        debug!("Genetic Algorithm - Done");
        self.done_internal()
    }

    // IMPLEMENTATION SPECIFIC
    fn config(&mut self) -> &GAConfig;

    fn population(&mut self) -> &Vec<T>;

    fn initialize_internal(&mut self) {}
    fn step_internal(&mut self) -> i32 { 0 }
    fn done_internal(&mut self) -> bool { true }
}
