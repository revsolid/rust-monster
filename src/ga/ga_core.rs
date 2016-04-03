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


/// Genetic Algorithm Configuration
pub trait GAConfig
{
    fn flags(&self) -> GAFlags;
    fn max_generations(&self) -> i32;
    fn probability_crossover(&self) -> f32;
    fn probability_mutation(&self) -> f32;
}


/// Genetic Algorithm Solution
pub trait GASolution
{
    //Static
    fn new() -> Self;

    // Instance
    fn clone(&self) -> Self;
    fn crossover(&self, other : &Self) -> Self;
    fn evaluate(&mut self) -> f32;
    fn fitness(&self) -> f32;
    fn mutate(&mut self, pMutation : f32);
}


/// Genetic Algorithm Population
pub struct GAPopulation<T>
{
    population: Vec<T>
}
impl<T: GASolution> GAPopulation<T>
{
    // TODO: New should use some parameters, maybe a Config
    pub fn new(p: Vec<T>) -> GAPopulation<T>
    {
        return GAPopulation {
                   population: p 
               }
    }

    pub fn evaluate(&mut self)
    {
        for ref mut ind in &mut self.population
        {
            ind.evaluate();
        }
    }

    pub fn size(&self) -> usize
    {
        self.population.len()
    }

    //TODO: this is a temporary implementation
    pub fn select(&self) -> &T
    {
        &self.population[0]
    }

    //TODO: This is a temporary implementation 
    pub fn best(&self) -> &T
    {
        &self.population[0]
    }

    //TODO: This is a temporary implementation 
    pub fn worst(&self) -> &T
    {
        &self.population[self.population.len()-1]
    }
}


/// Genetic Algorithm Solution Factory
pub trait GAFactory<T: GASolution>
{
    fn initial_population(&mut self) -> GAPopulation<T> 
    {
        GAPopulation::new(vec![])
    }
}


/// Genetic Algorithm
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

    fn population(&mut self) -> &mut GAPopulation<T>;

    fn initialize_internal(&mut self) {}
    fn step_internal(&mut self) -> i32 { 0 }
    fn done_internal(&mut self) -> bool { true }
}
