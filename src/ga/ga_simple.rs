use super::ga_core::{GAConfig, GAFactory, GAFlags, GeneticAlgorithm, GASolution};

/// Simple Genetic Algorithm 
///
/// A basic implementation of a Genetic Algorithm
/// TODO: Based on ?, GALib Ref
///

// Simple Genetic Algorithm Config
#[derive(Copy, Clone, Default, Debug)]
pub struct SimpleGeneticAlgorithmCfg
{
    pub d_seed : i32,
    pub pconv  : f32,
    pub is_min : bool,

    // GAConfig Trait
    pub max_generations        : i32, 
    pub flags                  : GAFlags, 
    pub percentage_crossover   : f32,
    pub probability_mutation   : f32,
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
    fn percentage_crossover(&self) -> f32 { 0.0 }
    fn probability_mutation(&self) -> f32 { 0.0 }
}

// Simple Genetic Algorithm
pub struct SimpleGeneticAlgorithm<T: GASolution>
{
  current_generation : i32, 
  config : SimpleGeneticAlgorithmCfg,
  population : Vec<T>
}
impl<T: GASolution> SimpleGeneticAlgorithm<T>
{
    pub fn new(cfg: SimpleGeneticAlgorithmCfg,
               factory: Option<&mut GAFactory<T>>,
               population: Option<Vec<T>>) -> SimpleGeneticAlgorithm<T>
    {
        let p : Vec<T>;
        match factory {
            Some(f) => {
                p = f.initial_population();
            },
            None => {
                match population {
                    Some(p_) => {
                        p = p_;
                    },
                    None => {
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

    fn population(&mut self) -> &Vec<T>
    {
        return &self.population
    }

    fn step_internal(&mut self) -> i32
    {
        self.current_generation += 1;
        self.current_generation
    }

    fn done_internal(&mut self) -> bool
    {
        self.current_generation >= self.config().max_generations() 
    }
}
