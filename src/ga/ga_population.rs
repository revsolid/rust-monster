// Copyright 2016 Revolution Solid & Contributors.
// author(s): sysnett
// rust-monster is licensed under a MIT License.

//! Genetic Algorithm Population

use ::ga::ga_core::GAIndividual;

use std::cmp::Ordering;
use std::iter::FromIterator;

// Better name than 'Basis'?
#[derive(Clone, Copy)]
pub enum GAPopulationSortBasis
{
    Raw,
    Fitness,
}

// The 'Copy' trait requires the 'Clone' trait.
// 'Copy' removes the 'move' semantics from an assignment or a function return of value.
#[derive(Clone, Copy, PartialEq)]
pub enum GAPopulationSortOrder
{
    LowIsBest,
    HighIsBest,
}

/// Genetic Algorithm Population
pub struct GAPopulation<T: GAIndividual>
{
    population: Vec<T>,

    sort_order: GAPopulationSortOrder,

    // 'population' ordered by raw score.
    population_order_raw: Vec<usize>,
    // Is 'population_order_raw' sorted?
    is_raw_sorted: bool,

    // 'population' ordered by fitness score.
    population_order_fitness: Vec<usize>,
    // Is 'population_order_fitness' sorted?
    is_fitness_sorted: bool,

    // We keep 2 lists of indexes to the population vector.
    // One sorted by raw score and one by fitness score.

    evaluation_function: fn(&mut Vec<T>),
}
impl<T: GAIndividual> GAPopulation<T>
{
    // TODO: New should use some parameters, maybe a Config
    pub fn new(p: Vec<T>, order: GAPopulationSortOrder) -> GAPopulation<T>
    {
        let gap = GAPopulation 
                  {
                      population: p,
                      sort_order: order,
                      population_order_raw: vec![],
                      is_raw_sorted: false,
                      population_order_fitness: vec![],
                      is_fitness_sorted: false,
                      evaluation_function: {
                          fn noop_evaluation<T>(_: &mut Vec<T>)
                          {
                              return
                          };
                          noop_evaluation
                      }
                  };

        gap
    }

    pub fn new_with_eval_function(p: Vec<T>, order: GAPopulationSortOrder,
                                  eval_function: fn(&mut Vec<T>)) -> GAPopulation<T>
    {
        let gap = GAPopulation 
                  {
                      population: p,
                      sort_order: order,
                      population_order_raw: vec![],
                      is_raw_sorted: false,
                      population_order_fitness: vec![],
                      is_fitness_sorted: false,
                      evaluation_function: eval_function
                  };

        gap
    }

    pub fn population(&mut self) -> &mut Vec<T>
    {
        return &mut self.population
    }

    pub fn evaluate(&mut self)
    {
        let f = self.evaluation_function;
        f(&mut self.population);
    }

    pub fn size(&self) -> usize
    {
        self.population.len()
    }

    pub fn order(&self) -> GAPopulationSortOrder
    {
        // This is not a 'move', but a copy (GAPopulationSortOrder implements the
        // 'Copy' trait). A move from a borrowed reference (such as 'self') would 
        // not be permitted.
        self.sort_order
    }

    //TODO: this is a temporary implementation
    pub fn select(&self) -> &T
    {
        self.individual(0, GAPopulationSortBasis::Fitness)
    }

    //TODO: This is a temporary implementation 
    pub fn best(&self) -> &T
    {
        // TODO: Call GAPopulation.scale().

        self.individual(0, GAPopulationSortBasis::Fitness)
    }

    //TODO: This is a temporary implementation 
    pub fn worst(&self) -> &T
    {
        self.individual(self.size()-1, GAPopulationSortBasis::Fitness)
    }

    pub fn best_by_raw_score(&self) -> &T
    {
        self.individual(0, GAPopulationSortBasis::Raw)
    }

    pub fn worst_by_raw_score(&self) -> &T
    {
        self.individual(self.size()-1, GAPopulationSortBasis::Raw)
    }

    pub fn best_by_fitness_score(&self) -> &T
    {
        self.individual(0, GAPopulationSortBasis::Fitness)
    }

    pub fn worst_by_fitness_score(&self) -> &T
    {
        self.individual(self.size()-1, GAPopulationSortBasis::Fitness)
    }

    pub fn individual(&self, i : usize, sort_basis : GAPopulationSortBasis) -> &T
    {
        // TODO: Check that i makes sense
        match sort_basis
        {
            GAPopulationSortBasis::Raw
            => { &self.population[self.population_order_raw[i]] },
            GAPopulationSortBasis::Fitness
            => { &self.population[self.population_order_fitness[i]] },
        }
    }

    pub fn sort(&mut self)
    {
        self.sort_int(false, GAPopulationSortBasis::Fitness);
        self.sort_int(false, GAPopulationSortBasis::Raw);
    }

    //TODO: I hate this name
    pub fn sort_int(&mut self, force_sort: bool, sort_basis: GAPopulationSortBasis)
    {
        let mut ordered : Vec<usize> = Vec::from_iter(0..self.size());
        match sort_basis
        {
            GAPopulationSortBasis::Raw
            =>  if (!self.is_raw_sorted) || force_sort
                {
                    match self.sort_order
                    {
                        GAPopulationSortOrder::LowIsBest =>
                        {
                            ordered.sort_by(|s1: &usize, s2: &usize|
                                            self.population[*s1].raw()
                                                .partial_cmp(&self.population[*s2].raw()).unwrap_or(Ordering::Equal));

                        },
                        GAPopulationSortOrder::HighIsBest =>
                        {
                            ordered.sort_by(|s1: &usize, s2: &usize|
                                            self.population[*s2].raw()
                                                .partial_cmp(&self.population[*s1].raw()).unwrap_or(Ordering::Equal));
                                                                  
                        },
                    };
                    self.population_order_raw = ordered;
                    self.is_raw_sorted = true;
                },

            GAPopulationSortBasis::Fitness
            =>  if (!self.is_fitness_sorted) || force_sort
                {
                    match self.sort_order
                    {
                        GAPopulationSortOrder::LowIsBest =>
                        { 
                            ordered.sort_by(|s1: &usize, s2: &usize|
                                            self.population[*s1].fitness()
                                                .partial_cmp(&self.population[*s2].fitness()).unwrap_or(Ordering::Equal));
                        },

                        GAPopulationSortOrder::HighIsBest =>
                        {
                            ordered.sort_by(|s1: &usize, s2: &usize|
                                            self.population[*s2].fitness()
                                                .partial_cmp(&self.population[*s1].fitness()).unwrap_or(Ordering::Equal));
                        }
                    };
                    self.population_order_fitness = ordered;
                    self.is_fitness_sorted = true;
                },
        };
    }

    pub fn raw_score_iterator<'a>(&'a self) -> GAPopulationRawIterator<'a, T>
    {
        GAPopulationRawIterator { population: &self, next: 0 }
    }

    pub fn fitness_score_iterator<'a>(&'a self) -> GAPopulationFitnessIterator<'a, T>
    {
        GAPopulationFitnessIterator { population: &self, next: 0 }
    }

    // TODO: This needs a better name or more parameters
    // Currently it only swaps out the worst-fitness individual
    // and puts the new individual into the population
    pub fn swap_individual(&mut self, new_individual: T)
    {
        let l = self.population.len();
        self.population[self.population_order_fitness[l-1]] = new_individual;
        self.is_raw_sorted = false;
        self.is_fitness_sorted = false;
    }
}

impl<T: GAIndividual + Clone> Clone for GAPopulation<T>
{
    fn clone(&self) -> Self
    {
        GAPopulation
        {
            population: self.population.clone(),
            sort_order: self.sort_order,
            population_order_raw: self.population_order_raw.clone(),
            is_raw_sorted: self.is_raw_sorted,
            population_order_fitness: self.population_order_fitness.clone(),
            is_fitness_sorted: self.is_fitness_sorted
        }
    }
}

impl<T: GAIndividual + PartialEq> PartialEq for GAPopulation<T>
{
    // Only meant for testing clone(). We may consider relaxing the
    // condition for general use (i.e. compare only population and
    // sort_order, and not the rest, which are the result of invoking
    // a method with deterministic outcome).
    fn eq(&self, other: &GAPopulation<T>) -> bool
    {
        // Fail fast, by doing lightweight comparisons first.
        //
        // Vector comparisons actually compare element by element.
        // That's why T has to implement PartialEq.
        self.sort_order == other.sort_order 
        && self.is_raw_sorted == other.is_raw_sorted
        && self.is_fitness_sorted == other.is_fitness_sorted
        && self.population == other.population 
        && self.population_order_raw == other.population_order_raw
        && self.population_order_fitness == other.population_order_fitness
    }
}

pub struct GAPopulationRawIterator<'a, T: 'a + GAIndividual>
{
    population: &'a GAPopulation<T>,
    next: usize
}

impl<'a, T: GAIndividual> Iterator for GAPopulationRawIterator<'a, T>
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.next == self.population.size()
        {
            None
        }
        else
        {
            self.next = self.next + 1;
            Some(self.population.individual(self.next - 1, GAPopulationSortBasis::Raw)) 
        }
    }
}

pub struct GAPopulationFitnessIterator<'a, T: 'a + GAIndividual>
{
    population: &'a GAPopulation<T>,
    next: usize
}

impl<'a, T: GAIndividual> Iterator for GAPopulationFitnessIterator<'a, T>
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.next == self.population.size()
        {
            None
        }
        else
        {
            self.next = self.next + 1;
            Some(self.population.individual(self.next - 1, GAPopulationSortBasis::Fitness)) 
        }
    }

}

////////////////////////////////////////
// Tests
#[cfg(test)]
mod test
{
    use super::*;
    use ::ga::ga_test::*;
    use ::ga::ga_core::*;

    #[test]
    fn test_sort_population()
    {
        ga_test_setup("ga_population::test_sort_population");
        let f = GA_TEST_FITNESS_VAL;
        let f_m = GA_TEST_FITNESS_VAL - 1.0;
        let i_f = 1.0 / f;
        let i_f_m = 1.0 / f_m;

        let mut population = GAPopulation::new(vec![GATestIndividual::new(f), GATestIndividual::new(f_m)], GAPopulationSortOrder::HighIsBest);
        population.sort();

        //GATestIndividual's Fitness is the inverse of the Score (F = 1/S)
        assert_eq!(population.individual(0, GAPopulationSortBasis::Raw).raw(), f);
        assert_eq!(population.individual(1, GAPopulationSortBasis::Raw).raw(), f_m);
        assert_eq!(population.individual(0, GAPopulationSortBasis::Fitness).fitness(), i_f_m);
        assert_eq!(population.individual(1, GAPopulationSortBasis::Fitness).fitness(), i_f);
        ga_test_teardown();
    }

    #[test]
    fn test_clone_population()
    {
        ga_test_setup("ga_population::test_clone_population");

        // The clone of a population should be equal to the original.

        let mut fact = GATestFactory::new(0.0);

        {
            let mut pop = fact.random_population(10, GAPopulationSortOrder::HighIsBest);

            // Upon creation.
            assert_eq!(pop == pop.clone(), true);

            pop.sort();

            // After updating its state.
            assert_eq!(pop == pop.clone(), true);
        }

        {
            let mut pop = fact.random_population(10, GAPopulationSortOrder::LowIsBest);

            // Upon creation.
            assert_eq!(pop == pop.clone(), true);

            pop.sort();

            // After updating its state.
            assert_eq!(pop == pop.clone(), true);
        }

        ga_test_teardown();
    }

    #[test]
    fn test_population_raw_iterator()
    {
        ga_test_setup("ga_population::test_population_raw_iterator");

        // Iteration of a LowIsBest population yields a sequence of non-decreasing raw scores.

        let mut expected_seq: Vec<f32> = (1..10).map(|rs| rs as f32).collect();
        {
            let mut inds: Vec<GATestIndividual> = Vec::new();
            for rs in 1..10
            {
                inds.push(GATestIndividual::new(rs as f32)); 
            }
            let mut pop = GAPopulation::new(inds, GAPopulationSortOrder::LowIsBest);
            pop.sort();

            let it = pop.raw_score_iterator();
            let actual_seq: Vec<f32> = it.map(|ind| { ind.raw() }).collect();
            assert_eq!(expected_seq, actual_seq);
        }

        // Iteration of a HighIsBest population yields a sequence of non-increasing raw scores.

        expected_seq.reverse();
        {
            let mut inds: Vec<GATestIndividual> = Vec::new();
            for rs in 1..10
            {
                inds.push(GATestIndividual::new(rs as f32)); 
            }
            let mut pop = GAPopulation::new(inds, GAPopulationSortOrder::HighIsBest);
            pop.sort();

            let it = pop.raw_score_iterator();
            let actual_seq: Vec<f32> = it.map(|ind| { ind.raw() }).collect();
            assert_eq!(expected_seq, actual_seq);
        }
        ga_test_teardown()
    }

    #[test]
    fn test_population_fitness_iterator()
    {
        ga_test_setup("ga_population::test_population_fitness_iterator");

        // Iteration of a HighIsBest population yields a sequence of non-decreasing fitness scores (when fitness = 1/raw).

        let mut expected_seq: Vec<f32> = (1..10).map(|rs| 1.0/(rs as f32)).collect();
        {
            let mut inds: Vec<GATestIndividual> = Vec::new();
            for rs in 1..10
            {
                inds.push(GATestIndividual::new(rs as f32)); 
            }
            let mut pop = GAPopulation::new(inds, GAPopulationSortOrder::HighIsBest);
            pop.sort();

            let it = pop.fitness_score_iterator();
            let actual_seq: Vec<f32> = it.map(|ind| { ind.fitness() }).collect();
            assert_eq!(expected_seq, actual_seq);
        }

        // Iteration of a LowIsBest population yields a sequence of non-increasing fitness scores (when fitness = 1/raw).

        expected_seq.reverse();
        {
            let mut inds: Vec<GATestIndividual> = Vec::new();
            for rs in 1..10
            {
                inds.push(GATestIndividual::new(rs as f32)); 
            }
            let mut pop = GAPopulation::new(inds, GAPopulationSortOrder::LowIsBest);
            pop.sort();

            let it = pop.fitness_score_iterator();
            let actual_seq: Vec<f32> = it.map(|ind| { ind.fitness() }).collect();
            assert_eq!(expected_seq, actual_seq);
        }
        ga_test_teardown();
    }

    #[test]
    fn test_population_with_custom_eval_func()
    {
        ga_test_setup("test_population_with_custom_eval_func");
        let expected_seq: Vec<f32> = (1..10).map(|rs| (rs*10) as f32).collect();
        {
            let mut inds: Vec<GATestIndividual> = Vec::new();
            for rs in 1..10
            {
                inds.push(GATestIndividual::new(rs as f32)); 
            }
            let mut pop = GAPopulation::new_with_eval_function(inds, GAPopulationSortOrder::LowIsBest,
                                                               {
                                                                   fn f(p: &mut Vec<GATestIndividual>)
                                                                   {
                                                                       for ref mut ind in p
                                                                        {
                                                                            let r = ind.raw();
                                                                            ind.set_raw(10.0*r);
                                                                        }
                                                                   }
                                                                   f
                                                               });
            pop.evaluate();
            pop.sort();

            let it = pop.raw_score_iterator();
            let actual_seq: Vec<f32> = it.map(|ind| { ind.raw() }).collect();
            assert_eq!(expected_seq, actual_seq);
        }

        ga_test_teardown();
    }
}
