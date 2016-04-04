// TODO: COPYRIGHT, USE & AUTHORS

use std::cmp::Ordering;

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
    fn new() -> Self;
    fn evaluate(&mut self) -> f32;
    fn crossover(&self, other : &Self) -> Self;
    fn mutate(&mut self, pMutation : f32);
    // Scaled fitness score
    fn fitness(&self) -> f32;
    // Raw objective score
    fn score(&self) -> f32;
}

// Better name than 'Basis'?
pub enum GAPopulationSortBasis
{
    RAW, 
    SCALED,
}

// Isn't a boolean enough?
pub enum GAPopulationSortOrder
{
    LOW_IS_BEST,
    HIGH_IS_BEST,
}

/// Genetic Algorithm Population
pub struct GAPopulation<'a, T: 'a>
{
    population: Vec<T>,

    sort_order: GAPopulationSortOrder,

    // 'population' ordered by RAW score. 
    population_raw: Vec<&'a T>,
    // Is 'population_raw' sorted?
    is_raw_sorted: bool,

    // 'population' ordered by SCALED score.
    population_scaled: Vec<&'a T>,
    // Is 'population_scaled' sorted?
    is_scaled_sorted: bool,

    // GALib keeps 2 lists of solutions, one sorted by RAW and one by SCALED.

}
impl<'a, T: GASolution> GAPopulation<'a, T>
{
    // Need best() and sort() for Selectors.

    // TODO: New should use some parameters, maybe a Config
    pub fn new(p: Vec<T>, order: GAPopulationSortOrder) -> GAPopulation<'a, T>
    {
        return GAPopulation 
        {
            population: p,
            sort_order: order,
            population_raw: vec![],
            is_raw_sorted: false,
            population_scaled: vec![],
            is_scaled_sorted: false
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

    pub fn sort(&self, force_sort: bool, sort_basis: GAPopulationSortBasis)
    {
        match sort_basis
        {
            GAPopulationSortBasis::RAW 
            =>  if self.is_raw_sorted || force_sort 
                {
                    match self.sort_order
                    {
                        GAPopulationSortOrder::LOW_IS_BEST
                        =>  self.population_raw.sort_by(|s1: &&T, s2: &&T|
                                                        s1.score().partial_cmp(&s2.score()).unwrap_or(Ordering::Equal)),
                        GAPopulationSortOrder::HIGH_IS_BEST
                        =>  self.population_raw.sort_by(|s1: &&T, s2: &&T|
                                                        s2.score().partial_cmp(&s1.score()).unwrap_or(Ordering::Equal))
                    };

                    self.is_raw_sorted = true;
                },

            GAPopulationSortBasis::SCALED
            =>  if self.is_scaled_sorted || force_sort 
                {
                    match self.sort_order
                    {
                        GAPopulationSortOrder::LOW_IS_BEST
                        =>  self.population_scaled.sort_by(|s1: &&T, s2: &&T|
                                                           s1.fitness().partial_cmp(&s2.fitness()).unwrap_or(Ordering::Equal)),
                        GAPopulationSortOrder::HIGH_IS_BEST
                        =>  self.population_scaled.sort_by(|s1: &&T, s2: &&T|
                                                           s2.fitness().partial_cmp(&s1.fitness()).unwrap_or(Ordering::Equal))
                    };

                    self.is_raw_sorted = true;
                },

            // Default?
        };
    }
}


/// Genetic Algorithm Solution Factory
pub trait GAFactory<T: GASolution>
{
    fn initial_population(&mut self) -> GAPopulation<T> 
    {
        GAPopulation::new(vec![], GAPopulationSortOrder::HIGH_IS_BEST)
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
