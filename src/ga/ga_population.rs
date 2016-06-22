// Copyright 2016 Revolution Solid & Contributors.
// author(s): sysnett
// rust-monster is licensed under a MIT License.

//! Genetic Algorithm Population

use super::ga_core::GAIndividual;

use std::cmp::Ordering;
use std::iter::FromIterator;

// Better name than 'Basis'?
#[derive(Clone, Copy)]
pub enum GAPopulationSortBasis
{
    Raw,
    Scaled,
}

// The 'Copy' trait requires the 'Clone' trait.
// 'Copy' removes the 'move' semantics from an assignment or a function return of value.
#[derive(Clone, Copy)]
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

    // 'population' ordered by Raw score.
    population_order_raw: Vec<usize>,
    // Is 'population_order_raw' sorted?
    is_raw_sorted: bool,

    // 'population' ordered by Scaled score.
    population_order_scaled: Vec<usize>,
    // Is 'population_scaled' sorted?
    is_scaled_sorted: bool,

    // We keep 2 lists of indexes to the population vector.
    // One sorted by Raw Score and one by Scaled Score (Fitness).
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
                      population_order_scaled: vec![],
                      is_scaled_sorted: false
                  };

        gap
    }

    pub fn population(&mut self) -> &mut Vec<T>
    {
        return &mut self.population
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
        self.individual(0, GAPopulationSortBasis::Scaled)
    }

    //TODO: This is a temporary implementation 
    pub fn best(&self) -> &T
    {
        // TODO: Call GAPopulation.scale().

        self.individual(0, GAPopulationSortBasis::Scaled)
    }

    //TODO: This is a temporary implementation 
    pub fn worst(&self) -> &T
    {
        self.individual(self.size()-1, GAPopulationSortBasis::Scaled)
    }

    pub fn best_by_raw_score(&self) -> &T
    {
        self.individual(0, GAPopulationSortBasis::Raw)
    }

    pub fn worst_by_raw_score(&self) -> &T
    {
        self.individual(self.size()-1, GAPopulationSortBasis::Scaled)
    }

    pub fn best_by_scaled_score(&self) -> &T
    {
        self.individual(0, GAPopulationSortBasis::Scaled)
    }

    pub fn worst_by_scaled_score(&self) -> &T
    {
        self.individual(self.size()-1, GAPopulationSortBasis::Scaled)
    }

    pub fn individual(&self, i : usize, sort_basis : GAPopulationSortBasis) -> &T
    {
        // TODO: Check that i makes sense
        match sort_basis
        {
            GAPopulationSortBasis::Raw
            => { &self.population[self.population_order_raw[i]] },
            GAPopulationSortBasis::Scaled
            => { &self.population[self.population_order_scaled[i]] },
        }
    }

    pub fn sort(&mut self)
    {
        self.sort_int(false, GAPopulationSortBasis::Scaled);
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
                                            self.population[*s1].score()
                                                .partial_cmp(&self.population[*s2].score()).unwrap_or(Ordering::Equal));

                        },
                        GAPopulationSortOrder::HighIsBest =>
                        {
                            ordered.sort_by(|s1: &usize, s2: &usize|
                                            self.population[*s2].score()
                                                .partial_cmp(&self.population[*s1].score()).unwrap_or(Ordering::Equal));
                                                                  
                        },
                    };
                    self.population_order_raw = ordered;
                    self.is_raw_sorted = true;
                },

            GAPopulationSortBasis::Scaled
            =>  if (!self.is_scaled_sorted) || force_sort
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
                    self.population_order_scaled = ordered;
                    self.is_scaled_sorted = true;
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
            Some(self.population.individual(self.next - 1, GAPopulationSortBasis::Scaled)) 
        }
    }

}

////////////////////////////////////////
// Tests
#[cfg(test)]
mod test
{
    use super::*;
    use super::super::ga_test::*;
    use super::super::ga_core::*;

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
        assert_eq!(population.individual(0, GAPopulationSortBasis::Raw).score(), f);
        assert_eq!(population.individual(1, GAPopulationSortBasis::Raw).score(), f_m);
        assert_eq!(population.individual(0, GAPopulationSortBasis::Scaled).fitness(), i_f_m);
        assert_eq!(population.individual(1, GAPopulationSortBasis::Scaled).fitness(), i_f);
        ga_test_teardown();
    }

    #[test]
    fn test_population_raw_iterator()
    {
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
            let actual_seq: Vec<f32> = it.map(|ind| { ind.score() }).collect();
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
            let actual_seq: Vec<f32> = it.map(|ind| { ind.score() }).collect();
            assert_eq!(expected_seq, actual_seq);
        }
    }

    #[test]
    fn test_population_fitness_iterator()
    {
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

    }
}
