// Copyright 2016 Revolution Solid & Contributors.
// author(s): sysnett
// rust-monster is licensed under a MIT License.
use ::ga::ga_core::{GAFactory, GAFlags, GeneticAlgorithm, GAIndividual};
use ::ga::ga_population::{GAPopulation, GAPopulationSortBasis, GAPopulationSortOrder};
use ::ga::ga_random::{GARandomCtx, GASeed};
use ::ga::ga_selectors::*;

use std::any::Any;

/// Simple Evaluation Context
/// Empty Evaluation Context 
struct SimpleEvaluationCtx;

/// Simple Genetic Algorithm Config
/// Genetic Algorithm Config Trait Implementation for the Simple Genetic Algorithm
#[derive(Copy, Clone, Default)]
pub struct SimpleGeneticAlgorithmCfg
{
    pub d_seed : GASeed,

    pub max_generations         : i32, 
    pub population_size         : usize,

    pub probability_crossover   : f32,
    pub probability_mutation    : f32,

    pub population_sort_order : GAPopulationSortOrder,

    pub elitism : bool,

    pub flags                   : GAFlags, 
}

/// Simple Genetic Algorithm 
///
/// A basic implementation of a Genetic Algorithm.
///
/// This genetic algorithm is the 'simple' genetic algorithm that Goldberg describes 
/// in his book. It uses non-overlapping populations. When you create a simple genetic 
/// algorithm, you must specify either an individual or a population of individuals. 
pub struct SimpleGeneticAlgorithm<'a, T: GAIndividual>
{
  current_generation : i32, 
  config : SimpleGeneticAlgorithmCfg,
  population : GAPopulation<T>,
  rng_ctx : GARandomCtx,
  eval_ctx: Option<&'a mut Any>,
}
impl<'a, T: GAIndividual> SimpleGeneticAlgorithm<'a, T>
{
    pub fn new(cfg: SimpleGeneticAlgorithmCfg,
               factory: Option<&mut GAFactory<T>>,
               population: Option<GAPopulation<T>>) -> SimpleGeneticAlgorithm<'a, T>
    {
        SimpleGeneticAlgorithm::new_with_eval_ctx(cfg, factory, population, None)
    }

    pub fn new_with_eval_ctx(cfg: SimpleGeneticAlgorithmCfg,
                             factory: Option<&mut GAFactory<T>>,
                             population: Option<GAPopulation<T>>,
                             eval_ctx: Option<&'a mut Any>) -> SimpleGeneticAlgorithm<'a, T>

    {
        //TODO: Some sort of generator for the name of the rng would be good
        let mut rng = GARandomCtx::from_seed(cfg.d_seed, String::from("")) ;
        let p : GAPopulation<T>;
        match factory
        {
            Some(f) => {
                p = f.random_population(cfg.population_size, cfg.population_sort_order, &mut rng);
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

        SimpleGeneticAlgorithm { current_generation: 0, config: cfg, population: p, rng_ctx: rng, eval_ctx: eval_ctx }
    }
}
impl<'a, T: GAIndividual + Clone> GeneticAlgorithm<T> for SimpleGeneticAlgorithm <'a, T>
{
    fn population(&mut self) -> &mut GAPopulation<T>
    {
        &mut self.population
    }

    fn initialize_internal(&mut self)
    {
        assert!(self.population().size() > 0);
        match self.eval_ctx
        {
            Some(ref mut eval_ctx) =>
            {
                self.population.evaluate(*eval_ctx);
            },
            None =>
            {
                let mut v = SimpleEvaluationCtx{};
                self.population.evaluate(&mut v as &mut Any);
            }
        }
        self.population.sort();
    }

    fn step_internal(&mut self) -> i32
    {
        let mut new_individuals : Vec<T> = vec![];

        let mut roulette_selector = GARouletteWheelSelector::new(self.population.size());
        roulette_selector.update::<GARawScoreSelection>(&mut self.population);


        // Create new individuals 
        for _ in 0..self.population.size()
        {
            let ind = roulette_selector.select::<GARawScoreSelection>(&self.population, &mut self.rng_ctx);
            let mut new_ind = ind.clone();
            if self.rng_ctx.test_value(self.config.probability_crossover)
            {
                let ind_2 = roulette_selector.select::<GARawScoreSelection>(&self.population, &mut self.rng_ctx);
                new_ind = *ind.crossover(ind_2, &mut self.rng_ctx);
            }

            new_ind.mutate(self.config.probability_mutation, &mut self.rng_ctx);

            new_individuals.push(new_ind);
        }

        let best_old_individual = self.population.best(0, GAPopulationSortBasis::Fitness).clone();

        // Evaluate the new population
        // TODO: Archive the old population
        let order = self.population.order();
        self.population = GAPopulation::new(new_individuals, order);

        match self.eval_ctx
        {
            Some(ref mut eval_ctx) =>
            {
                self.population.evaluate(*eval_ctx);
            },
            None =>
            {
                let mut v = SimpleEvaluationCtx{};
                self.population.evaluate(&mut v as &mut Any);
            }
        }
        self.population.sort();

        if self.config.elitism
        {
            self.population.swap_individual(best_old_individual);
            self.population.sort(); // I don't love the double sorting :(
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
                                                   population_size: 1, 
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
