// TODO: COPYRIGHT, USE & AUTHORS
use super::ga_core::{GAConfig, GAFactory, GAFlags, GeneticAlgorithm, GASolution};
use super::ga_population::GAPopulation;
use super::ga_random::ga_random_float_test;

// Simple Genetic Algorithm Config
#[derive(Copy, Clone, Default, Debug)]
// TODO: RUST DOCS! 
pub struct SimpleGeneticAlgorithmCfg
{
    pub d_seed : i32,
    pub pconv  : f32,
    pub is_min : bool,

    // GAConfig Trait
    pub max_generations         : i32, 
    pub flags                   : GAFlags, 
    pub probability_crossover   : f32,
    pub probability_mutation    : f32,

    // Simple GA
    pub elitism : bool
}
impl GAConfig for SimpleGeneticAlgorithmCfg
{
    fn flags(&self) -> GAFlags
    {
        self.flags
    }
    fn max_generations(&self) -> i32
    {
        self.max_generations
    }
    fn probability_crossover(&self) -> f32
    {
        self.probability_crossover
    }
    fn probability_mutation(&self) -> f32
    {
        self.probability_mutation 
    }
}
impl SimpleGeneticAlgorithmCfg
{
    fn elitism(&self) -> bool
    {
        self.elitism
    }
}

/// Simple Genetic Algorithm 
///
/// A basic implementation of a Genetic Algorithm.
///
/// This genetic algorithm is the 'simple' genetic algorithm that Goldberg describes 
/// in his book. It uses non-overlapping populations. When you create a simple genetic 
/// algorithm, you must specify either an individual or a population of individuals. 
/// The new genetic algorithm will clone the individual(s) that you specify to make 
/// its own population. You can change most of the genetic algorithm behaviors after 
/// creation and during the course of the evolution.
///
/// The simple genetic algorithm creates an initial population by cloning the individual 
/// or population you pass when you create it. Each generation the algorithm creates 
/// an entirely new population of individuals by selecting from the previous population 
/// then mating to produce the new offspring for the new population. This process continues 
/// until the stopping criteria are met (determined by the terminator).
///
/// Elitism is optional. By default, elitism is on, meaning that the best individual 
/// from each generation is carried over to the next generation. To turn off elitism, 
/// pass gaFalse to the elitist member function. 
///
pub struct SimpleGeneticAlgorithm<T: GASolution>
{
  current_generation : i32, 
  config : SimpleGeneticAlgorithmCfg,
  population : GAPopulation<T>
}
impl<T: GASolution> SimpleGeneticAlgorithm<T>
{
    // TODO: Document this -new- pattern and others from the
    // pattern GitHub
    pub fn new(cfg: SimpleGeneticAlgorithmCfg,
               factory: Option<&mut GAFactory<T>>,
               population: Option<GAPopulation<T>>) -> SimpleGeneticAlgorithm<T>
    {
        let p : GAPopulation<T>;
        match factory
        {
            Some(f) => {
                p = f.initial_population();
            },
            None => {
                match population
                {
                    Some(p_) =>
                    {
                        p = p_;
                    },
                    None =>
                    {
                        panic!("Simple Genetic Algorithm - either factory or population need to be provided");
                    }
                }
            }
        }

        SimpleGeneticAlgorithm { current_generation: 0, config : cfg, population : p}
    }
}
impl<T: GASolution> GeneticAlgorithm<T> for SimpleGeneticAlgorithm <T>
{
    fn config(&mut self) -> &GAConfig
    {
        &self.config
    }

    fn population(&mut self) -> &mut GAPopulation<T>
    {
        return &mut self.population
    }

    fn initialize_internal(&mut self)
    {
        assert!(self.population().size() > 0)
    }

    fn step_internal(&mut self) -> i32
    {
        let mut new_individuals : Vec<T> = vec![];

        // Evaluate the population
        self.population.evaluate();

        // Create new individuals 
        for _ in 0..self.population.size()
        {
            let ind = self.population.select();
            let mut new_ind = ind.clone();
            if ga_random_float_test(self.config.probability_crossover())
            {
                let ind_2 = self.population.select();
                new_ind = ind.crossover(ind_2);
            }

            new_ind.mutate(self.config.probability_mutation());

            new_individuals.push(new_ind);
        }

        let best_old_individual = self.population.best().clone();
        // population.swap(new_individuals)
        self.population.sort();

        if self.config.elitism()
        {
            if best_old_individual.fitness() > self.population.worst().fitness()
            {
                // population.swap_individual(best_old_individual, ...)
            }
        }

        self.current_generation += 1;
        self.current_generation
    }

    fn done_internal(&mut self) -> bool
    {
        self.current_generation >= self.config().max_generations() 
    }
}
