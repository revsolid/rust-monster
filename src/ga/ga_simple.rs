// Copyright 2016 Revolution Solid & Contributors.
// author(s): sysnett
// rust-monster is licensed under a MIT License.
use ::ga::ga_core::{GAFactory, GAFlags, GeneticAlgorithm, GAIndividual};
use ::ga::ga_population::GAPopulation;
use ::ga::ga_random::{GARandomCtx, GASeed};

/// Simple Genetic Algorithm Config
/// Genetic Algorithm Config Trait Implementation for the Simple Genetic Algorithm
#[derive(Copy, Clone, Default, Debug)]
pub struct SimpleGeneticAlgorithmCfg
{
    pub d_seed : GASeed,
    pub pconv  : f32,
    pub is_min : bool,
    pub max_generations         : i32, 
    pub flags                   : GAFlags, 
    pub probability_crossover   : f32,
    pub probability_mutation    : f32,
    pub elitism : bool,
}

/// Simple Genetic Algorithm 
///
/// A basic implementation of a Genetic Algorithm.
///
/// This genetic algorithm is the 'simple' genetic algorithm that Goldberg describes 
/// in his book. It uses non-overlapping populations. When you create a simple genetic 
/// algorithm, you must specify either an individual or a population of individuals. 
pub struct SimpleGeneticAlgorithm<T: GAIndividual>
{
  current_generation : i32, 
  config : SimpleGeneticAlgorithmCfg,
  population : GAPopulation<T>,
  rng_ctx : GARandomCtx,
}
impl<T: GAIndividual> SimpleGeneticAlgorithm<T>
{
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

        //TODO: Some sort of generator for the name of the rng would be good
        SimpleGeneticAlgorithm { current_generation: 0, config : cfg, population : p, rng_ctx : GARandomCtx::from_seed(cfg.d_seed, String::from("")) }
    }
}
impl<T: GAIndividual + Clone> GeneticAlgorithm<T> for SimpleGeneticAlgorithm <T>
{
    fn population(&mut self) -> &mut GAPopulation<T>
    {
        &mut self.population
    }

    fn initialize_internal(&mut self)
    {
        assert!(self.population().size() > 0);
        self.population.sort();
    }

    fn step_internal(&mut self) -> i32
    {
        let mut new_individuals : Vec<T> = vec![];

        // Create new individuals 
        for _ in 0..self.population.size()
        {
            let ind = self.population.select();
            let mut new_ind = ind.clone();
            if self.rng_ctx.test_value(self.config.probability_crossover)
            {
                let ind_2 = self.population.select();
                new_ind = *ind.crossover(ind_2);
            }

            new_ind.mutate(self.config.probability_mutation);

            new_individuals.push(new_ind);
        }

        // Evaluate the new population
        // self.population.swap(new_individuals);
        self.population.evaluate();
        self.population.sort();

        let best_old_individual = self.population.best().clone();

        if self.config.elitism
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
        self.current_generation >= self.config.max_generations 
    }
}

////////////////////////////////////////
// Tests
#[cfg(test)]
mod tests
{
    use ::ga::ga_test::*;
    use ::ga::ga_population::*;
    use ::ga::ga_core::*;
    use super::*;

    fn simple_ga_validation(sga:&mut SimpleGeneticAlgorithm<GATestIndividual>)
    {
        sga.initialize();
        assert_eq!(sga.step(), 1);
        assert_eq!(sga.done(), false);
        assert_eq!(sga.population().size(), 1);
        assert_eq!(sga.population().best().raw(), GA_TEST_FITNESS_VAL);
    }

    #[test]
    fn init_test_with_initial_population()
    {
        ga_test_setup("ga_simple::init_test_with_initial_population");
        let initial_population = GAPopulation::new(vec![GATestIndividual::new(GA_TEST_FITNESS_VAL)],
                                 GAPopulationSortOrder::HighIsBest);
        let mut ga : SimpleGeneticAlgorithm<GATestIndividual> =
                     SimpleGeneticAlgorithm::new(SimpleGeneticAlgorithmCfg {
                                                   d_seed : [1; 4],
                                                   flags : DEBUG_FLAG,
                                                   max_generations: 100,
                                                   ..Default::default()
                                                 },
                                                 None,
                                                 Some(initial_population) 
                                                 );
        simple_ga_validation(&mut ga);
    }

    #[test]
    fn init_test_with_factory()
    {
        ga_test_setup("ga_simple::init_test_with_factory");
        let mut factory = GATestFactory::new(GA_TEST_FITNESS_VAL);
        let mut ga : SimpleGeneticAlgorithm<GATestIndividual> =
                     SimpleGeneticAlgorithm::new(SimpleGeneticAlgorithmCfg {
                                                   d_seed : [1; 4],
                                                   flags : DEBUG_FLAG,
                                                   max_generations: 100,
                                                   ..Default::default()
                                                 },
                                                 Some(&mut factory as &mut GAFactory<GATestIndividual>),
                                                 None
                                                 );
        simple_ga_validation(&mut ga);
        ga_test_teardown();
    }

    #[test]
    #[should_panic]
    #[allow(unused_variables)]
    fn init_test_missing_args()
    {
        ga_test_setup("ga_simple::init_test_missing_args");
        let ga : SimpleGeneticAlgorithm<GATestIndividual> =
                 SimpleGeneticAlgorithm::new(SimpleGeneticAlgorithmCfg {
                                               d_seed : [1; 4],
                                               flags : DEBUG_FLAG,
                                               max_generations: 100,
                                               ..Default::default()
                                             },
                                             None,
                                             None 
                                             );
        // Not reached
        ga_test_teardown();
    }

    #[test]
    #[should_panic]
    fn init_test_empty_initial_pop()
    {
        ga_test_setup("ga_simple::init_test_empty_initial_pop");
        let empty_initial_population : GAPopulation<GATestIndividual> = GAPopulation::new(vec![], GAPopulationSortOrder::HighIsBest);
        let mut ga : SimpleGeneticAlgorithm<GATestIndividual> =
                     SimpleGeneticAlgorithm::new(SimpleGeneticAlgorithmCfg {
                                                   d_seed : [1; 4],
                                                   flags : DEBUG_FLAG,
                                                   max_generations: 100,
                                                   ..Default::default()
                                                 },
                                                 None,
                                                 Some(empty_initial_population) 
                                                 );
        ga.initialize();
        //Not reached 
        ga_test_teardown();
    }
}
