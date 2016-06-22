// Copyright 2016 Revolution Solid & Contributors.
// author(s): sysnett
// rust-monster is licensed under a MIT License.

//! Genetic Algorithm Population

use ::ga::ga_core::GAIndividual;
use ::ga::ga_random::GARandomCtx;

use std::cmp::{Ordering};
use std::iter::FromIterator;
use std::any::Any;
use std::option::Option;
use std::f32;

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

impl Default for GAPopulationSortOrder
{
    fn default() -> GAPopulationSortOrder { GAPopulationSortOrder::HighIsBest }
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

    // `None` if statistics haven't been computed.
    statistics: Option<GAPopulationStats>,
}
impl<T: GAIndividual> GAPopulation<T>
{
    // TODO: New should use some parameters, maybe a Config
    pub fn new(p: Vec<T>, order: GAPopulationSortOrder) -> GAPopulation<T>
    {
        GAPopulation
        {
            population: p,
            sort_order: order,
            population_order_raw: vec![],
            is_raw_sorted: false,
            population_order_fitness: vec![],
            is_fitness_sorted: false,
            statistics: None
        }
    }

    pub fn population(&mut self) -> &mut Vec<T>
    {
        return &mut self.population
    }

    pub fn evaluate(&mut self, evaluation_ctx: &mut Any)
    {
        for ref mut ind in &mut self.population
        {
            ind.evaluate(evaluation_ctx);
        }
    }

    pub fn size(&self) -> usize
    {
        self.population.len()
    }

    fn size_mut(&mut self) -> usize
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

    pub fn set_order_and_sort(&mut self, order: GAPopulationSortOrder)
    {
        // TODO: Test that changing order after the population has been used doesn't
        // cause problems.
        if self.sort_order != order
        {
            self.sort_order = order;
            self.is_raw_sorted = false;
            self.is_fitness_sorted = false;
            self.sort();
        }
    }

    //TODO: this is a temporary implementation
    pub fn select(&self) -> &T
    {
        self.individual(0, GAPopulationSortBasis::Fitness)
    }

    //TODO: This is a temporary implementation 
    pub fn best(&self, i: usize, sort_basis: GAPopulationSortBasis) -> &T
    {
        // TODO: Call GAPopulation.scale().

        self.individual(i, sort_basis)
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

    pub fn best_by_raw_score_mut(&mut self) -> &mut T
    {
        self.individual_mut(0, GAPopulationSortBasis::Raw)
    }

    pub fn kth_best_by_raw_score(&self, k: usize) -> &T
    {
        self.individual(k, GAPopulationSortBasis::Raw)
    }

    pub fn worst_by_raw_score(&self) -> &T
    {
        self.individual(self.size()-1, GAPopulationSortBasis::Raw)
    }

    pub fn worst_by_raw_score_mut(&mut self) -> &mut T
    {
        // Cannot call self.size() in line with individual_mut(),
        // because then there would be simultaneous immutable
        // (from size()) and mutable (from individual_mut()) borrows
        // of self.
        let size = self.size();
        self.individual_mut(size-1, GAPopulationSortBasis::Raw)
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

    pub fn individual_mut(&mut self, i : usize, sort_basis : GAPopulationSortBasis) -> &mut T
    {
        match sort_basis
        {
            GAPopulationSortBasis::Raw
            => { &mut self.population[self.population_order_raw[i]] },
            GAPopulationSortBasis::Fitness
            => { &mut self.population[self.population_order_fitness[i]] },
        }

    }

    pub fn sort(&mut self)
    {
        self.sort_int(false, GAPopulationSortBasis::Fitness);
        self.sort_int(false, GAPopulationSortBasis::Raw);
    }
    
    pub fn force_sort(&mut self)
    {
        self.sort_int(true, GAPopulationSortBasis::Fitness);
        self.sort_int(true, GAPopulationSortBasis::Raw);
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

    pub fn swap_individual(&mut self, new_individual: T)
    {
        let mut should_swap = false;

        {
            let worst = self.worst();
            match self.sort_order
            {
                GAPopulationSortOrder::LowIsBest =>
                {
                    should_swap = new_individual.fitness() < worst.fitness();
                },
                GAPopulationSortOrder::HighIsBest =>
                {
                    should_swap = new_individual.fitness() > worst.fitness();
                }
            }
        }
        let l = self.population.len();
        if should_swap
        {
            self.population[self.population_order_fitness[l-1]] = new_individual;
            self.is_raw_sorted = false;
            self.is_fitness_sorted = false;
        }
    }

    // Compute statistics of a population.
    //
    // Statistics are computed only if they haven't been computed before.
    // Subsequent calls will return the statistics computed previously.
    //
    // `None` is returned if the population is empty. Otherwise,
    // a clone of the statistics owned by the population is returned,
    // wrapped in `Option`.
    pub fn statistics(&mut self) -> Option<GAPopulationStats>
    {
        match self.statistics
        {
            // Statistics have been computed already. Return a clone.
            Some(_) => self.statistics.clone(),

            None    => 
            {
                if self.size() == 0
                {
                    // No individuals over which to compute statistics.
                    None
                }
                else
                {
                    // Populated with appropriate default values.
                    let mut stats = GAPopulationStats::new();

                    for ind in &self.population
                    {
                        let raw = ind.raw();
                        stats.raw_sum += raw;
                        stats.raw_max = stats.raw_max.max(raw);
                        stats.raw_min = stats.raw_min.min(raw);

                        let fitness = ind.fitness();
                        stats.fitness_sum += fitness;
                        stats.fitness_max = stats.fitness_max.max(fitness);
                        stats.fitness_min = stats.fitness_min.min(fitness);
                    }

                    let size = self.size();
                    stats.raw_avg = stats.raw_sum / size as f32;
                    stats.fitness_avg = stats.fitness_sum / size as f32;

                    // When there is only 1 individual, the default value of the
                    // variance is appropriate.
                    if size > 1
                    {
                        for ind in &self.population
                        {
                            stats.raw_var += (ind.raw() - stats.raw_avg).powi(2);
                            stats.fitness_var += (ind.fitness() - stats.fitness_avg).powi(2);
                        }
                        stats.raw_var /= (size-1) as f32;
                        stats.fitness_var /= (size-1) as f32;
                    }

                    stats.raw_std_dev = stats.raw_var.sqrt();
                    stats.fitness_std_dev = stats.fitness_var.sqrt();

                    // A clone will be owned by the population, to reuse in future calls.
                    self.statistics = Some(stats.clone());

                    // Move the working object to the caller (`GAPopulationStats` doesn't
                    // implement the `Copy` trait). 2 allocations must have been made only:
                    // 1) The working object being returned and moved here, and 2) the clone
                    // owned by the population.
                    Some(stats)
                }
            }
        }
    }

    pub fn reset_statistics(&mut self)
    {
        self.statistics = None;
    }

    pub fn diversity(&mut self) -> f32
    {
        // Dummy implementation.
        // -1.0 is the recorded diversity value when diversity is not recorded.
        -1.0
    }

    pub fn print_statistics(&self)
    {
        match self.statistics 
        {
            Some(ref statistics) =>
            {
                debug!("RAW");
                debug!("sum {}\n", statistics.raw_sum);
                debug!("avg {}\n", statistics.raw_avg);
                debug!("max {}\n", statistics.raw_max);
                debug!("min {}\n", statistics.raw_min);
                debug!("var {}\n", statistics.raw_var);
                debug!("dev {}\n", statistics.raw_std_dev);
                debug!("FIT");
                debug!("sum {}\n", statistics.fitness_sum);
                debug!("avg {}\n", statistics.fitness_avg);
                debug!("max {}\n", statistics.fitness_max);
                debug!("min {}\n", statistics.fitness_min);
                debug!("var {}\n", statistics.fitness_var);
                debug!("dev {}\n", statistics.fitness_std_dev);
            },
                None => {}
        }
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
            is_fitness_sorted: self.is_fitness_sorted,
            statistics: self.statistics.clone()
        }
    }
}

impl<T: GAIndividual + PartialEq> PartialEq for GAPopulation<T>
{
    fn eq(&self, other: &GAPopulation<T>) -> bool
    {
        self.size() == other.size()
        && self.sort_order == other.sort_order 
        && self.is_raw_sorted == other.is_raw_sorted
        && self.is_fitness_sorted == other.is_fitness_sorted
        // FIXME: INFs are not equal to each other; NANs either.
        // If statistics contain INFs or NANs, this check will
        // fail. This happens when raw=0 and fitness=1/raw.
        && self.statistics == other.statistics
        // FIXME: sort() must have been called to avoid panic.
        && self.raw_score_iterator().eq(other.raw_score_iterator())
        && self.fitness_score_iterator().eq(other.fitness_score_iterator())
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

/// Population statistics.
///
/// Statistics of `GAIndividual`s' scores (both Raw and Fitness) of a `GAPopulation`:
///
/// Sum
/// Average
/// Maximum
/// Minimum
/// Variance
/// Standard deviation
#[derive(Clone)]
pub struct GAPopulationStats
{
    pub raw_sum: f32,
    pub raw_avg: f32,
    pub raw_max: f32,
    pub raw_min: f32,
    pub raw_var: f32,
    pub raw_std_dev: f32,

    pub fitness_sum: f32,
    pub fitness_avg: f32,
    pub fitness_max: f32,
    pub fitness_min: f32,
    pub fitness_var: f32,
    pub fitness_std_dev: f32,
}

impl GAPopulationStats
{
    fn new() -> GAPopulationStats
    {
        GAPopulationStats
        {
            raw_sum: 0.0,
            raw_avg: 0.0,
            raw_max: f32::NEG_INFINITY,
            raw_min: f32::INFINITY,
            raw_var: 0.0,
            raw_std_dev: 0.0,

            fitness_sum: 0.0,
            fitness_avg: 0.0,
            fitness_max: f32::NEG_INFINITY,
            fitness_min: f32::INFINITY,
            fitness_var: 0.0,
            fitness_std_dev: 0.0,
        }
    }
}

impl PartialEq for GAPopulationStats
{
    fn eq(&self, other: &GAPopulationStats) -> bool
    {
        let error = 0.00001;
        (self.raw_sum-other.raw_sum).abs() < error
        && (self.raw_avg-other.raw_avg).abs() < error
        && (self.raw_max-other.raw_max).abs() < error
        && (self.raw_min-other.raw_min).abs() < error
        && (self.raw_var-other.raw_var).abs() < error
        && (self.raw_std_dev-other.raw_std_dev).abs() < error
        && (self.fitness_sum-other.fitness_sum).abs() < error
        && (self.fitness_avg-other.fitness_avg).abs() < error
        && (self.fitness_max-other.fitness_max).abs() < error
        && (self.fitness_min-other.fitness_min).abs() < error
        && (self.fitness_var-other.fitness_var).abs() < error
        && (self.fitness_std_dev-other.fitness_std_dev).abs() < error
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
    use ::ga::ga_random::*;

    use std::f32;

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
            let mut pop = fact.random_population(10, GAPopulationSortOrder::HighIsBest, &mut GARandomCtx::new_unseeded("ga_population::test_clone_population".to_string()));

            // Upon creation.
            // FIXME: Panics because eq() iterates over non-init'ed sorted arrays.
            //assert_eq!(pop == pop.clone(), true);

            pop.sort();
            pop.statistics();

            // After updating its state.
            assert_eq!(pop == pop.clone(), true);
        }

        {
            let mut pop = fact.random_population(10, GAPopulationSortOrder::LowIsBest, &mut GARandomCtx::new_unseeded("ga_population::test_clone_population".to_string()));

            // Upon creation.
            // FIXME: Panics because eq() iterates over non-init'ed sorted arrays.
            //assert_eq!(pop == pop.clone(), true);

            pop.sort();
            pop.statistics();

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
    fn test_population_raw_statistics()
    {
        let raw_scores: Vec<f32> = vec![9.0, 2.0, 5.0, 4.0, 12.0, 7.0, 8.0, 11.0, 9.0, 3.0,
                                        7.0, 4.0, 12.0, 5.0, 4.0, -10.0, 9.0, 6.0, 9.0, 4.0];
        let expected_sum = raw_scores.iter().fold(0.0, |sum, rs| sum + rs);
        let expected_avg = expected_sum / raw_scores.len() as f32;
        let expected_max = raw_scores.iter().cloned().fold(f32::NEG_INFINITY, |max, rs| max.max(rs));
        let expected_min = raw_scores.iter().cloned().fold(f32::INFINITY, |min, rs| min.min(rs));
        let expected_var = raw_scores.iter().fold(0.0, |var, rs| var + (rs - expected_avg).powi(2)) / (raw_scores.len()-1) as f32;
        let expected_std_dev = expected_var.sqrt();

        // Statistics are `None` for an empty population.

        {
            let mut pop = GAPopulation::new(Vec::<GATestIndividual>::new(), GAPopulationSortOrder::HighIsBest);

            assert_eq!(pop.statistics().is_none(), true)
        }

        // 1-individual population.

        {
            let mut pop = GAPopulation::new(vec![GATestIndividual::new(5.0)], GAPopulationSortOrder::HighIsBest);

            let stats = pop.statistics().unwrap();

            assert_eq!(stats.raw_sum, 5.0);
            assert_eq!(stats.raw_avg, 5.0);
            assert_eq!(stats.raw_max, 5.0);
            assert_eq!(stats.raw_min, 5.0);
            assert_eq!(stats.raw_var, 0.0);
            assert_eq!(stats.raw_std_dev, 0.0);
        }

        // Multi-individual population with HighIsBest ranking.

        {
            let mut inds: Vec<GATestIndividual> = Vec::new();
            for rs in raw_scores.iter().cloned()
            {
                inds.push(GATestIndividual::new(rs)); 
            }
            let mut pop = GAPopulation::new(inds, GAPopulationSortOrder::HighIsBest);

            // Statistics should not change across invocations.

            let mut stats;
            for _ in 0..2
            {
                stats = pop.statistics().unwrap();
                assert_eq!(stats.raw_sum, expected_sum);
                assert_eq!(stats.raw_avg, expected_avg);
                assert_eq!(stats.raw_max, expected_max);
                assert_eq!(stats.raw_min, expected_min);
                assert_eq!(stats.raw_var, expected_var);
                assert_eq!(stats.raw_std_dev, expected_std_dev);
            }

            // Statistics should not change after sorting the individuals.

            pop.sort();
            stats = pop.statistics().unwrap();
            assert_eq!(stats.raw_sum, expected_sum);
            assert_eq!(stats.raw_avg, expected_avg);
            assert_eq!(stats.raw_max, expected_max);
            assert_eq!(stats.raw_min, expected_min);
            assert_eq!(stats.raw_var, expected_var);
            assert_eq!(stats.raw_std_dev, expected_std_dev);
        }

        // Multi-individual population with LowIsBest ranking.

        {
            let mut inds: Vec<GATestIndividual> = Vec::new();
            for rs in raw_scores.iter().cloned()
            {
                inds.push(GATestIndividual::new(rs)); 
            }
            let mut pop = GAPopulation::new(inds, GAPopulationSortOrder::LowIsBest);

            // Statistics should not change across invocations.

            let mut stats;
            for _ in 0..2
            {
                stats = pop.statistics().unwrap();
                assert_eq!(stats.raw_sum, expected_sum);
                assert_eq!(stats.raw_avg, expected_avg);
                assert_eq!(stats.raw_max, expected_max);
                assert_eq!(stats.raw_min, expected_min);
                assert_eq!(stats.raw_var, expected_var);
                assert_eq!(stats.raw_std_dev, expected_std_dev);
            }

            // Statistics should not change after sorting the individuals.

            pop.sort();
            stats = pop.statistics().unwrap();
            assert_eq!(stats.raw_sum, expected_sum);
            assert_eq!(stats.raw_avg, expected_avg);
            assert_eq!(stats.raw_max, expected_max);
            assert_eq!(stats.raw_min, expected_min);
            assert_eq!(stats.raw_var, expected_var);
            assert_eq!(stats.raw_std_dev, expected_std_dev);
        }

    }
}
