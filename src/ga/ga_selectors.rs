// Copyright 2016 Revolution Solid & Contributors.
// author(s): carlos-lopez-garces, sysnett
// rust-monster is licensed under an MIT License.

//! GA Selectors
//!
//! A selector represents and performs a method of selection.
//!
//! Selection is the action of choosing individuals of the current
//! generation that will create offspring for the next generation.
//!
//! Selectors represent and perform a different method of selection each. The
//! expectation is that the offspring individuals be fitter than their selected
//! parents. For this reason, many of the selectors tend to choose the fitter
//! most of the time. However, many of them acknowledge the need for selecting
//! less fit individuals, too: A genetic operator (crossover, mutation) used on
//! suboptimal individuals may sometimes produce an individual that is fitter 
//! than those that could be produced by optimal ones.
//!
//! Available selectors:
//!
//! `GARankSelector`
//! `GAUniformSelector`
//! `GARouletteWheelSelector`
//! `GATournamentSelector`
//!
//! # Examples
use super::ga_core::GAIndividual;
use super::ga_population::{GAPopulation, GAPopulationSortBasis, GAPopulationSortOrder};
use super::ga_random::{GARandomCtx};
use std::cmp;

/// Selector trait.
///
/// Selector common interface. Each selector implements a different method
/// of selection and keeps and manages its own internal state.
pub trait GASelector<T: GAIndividual>
{
    /// Update internal state. 
    ///
    /// NOOP default implementation for selectors that don't keep internal state.
    fn update<S: GAScoreSelection<T>>(&mut self, _: &mut GAPopulation<T>) {}

    /// Select an individual from the population. 
    ///
    /// Each selector implements a different method of selection. Randomization 
    /// is a key aspect of all methods.
    fn select<'a, S: GAScoreSelection<T>>(&self, pop: &'a GAPopulation<T>, rng_ctx: &mut GARandomCtx) -> &'a T;
}

/// Selection score type basis.
///
/// Selectors are configured, at the time of creation, with the type of score
/// {raw, fitness} they will use to perform selections. The type of score
/// ultimately determines the function that will be invoked on the `GAIndividual`
/// to obtain the score value of the configured type. `GAScoreSelection`
/// objects provide a unified interface to the different score functions of a
/// `GAIndividual`. Selectors use these objects to obtain score values of the
/// configured type, without explicitly choosing between them based on
/// `GAPopulationSortBasis`.
pub trait GAScoreSelection<T: GAIndividual>
{
    fn score(ind: &T) -> f32;

    fn population_sort_basis() -> GAPopulationSortBasis;

    fn max_score(pop: &GAPopulation<T>) -> f32;

    fn min_score(pop: &GAPopulation<T>) -> f32;

    fn iterator<'a>(pop: &'a GAPopulation<T>) -> Box<Iterator<Item=&'a T> + 'a>;
}

/// Selection based on raw score.
pub struct GARawScoreSelection;

impl<T: GAIndividual> GAScoreSelection<T> for GARawScoreSelection
{
    fn score(ind: &T) -> f32
    {
        ind.raw()
    }

    fn population_sort_basis() -> GAPopulationSortBasis
    {
        GAPopulationSortBasis::Raw
    }

    fn max_score(pop: &GAPopulation<T>) -> f32
    {
        Self::score(pop.best_by_raw_score())
    }

    fn min_score(pop: &GAPopulation<T>) -> f32
    {
        Self::score(pop.worst_by_raw_score())
    }

    fn iterator<'a>(pop: &'a GAPopulation<T>) -> Box<Iterator<Item=&'a T> + 'a>
    {
        Box::new(pop.raw_score_iterator())
    }
}

/// Selection based on fitness score.
pub struct GAFitnessScoreSelection;

impl<T: GAIndividual> GAScoreSelection<T> for GAFitnessScoreSelection
{
    fn score(ind: &T) -> f32
    {
        ind.fitness()
    }

    fn population_sort_basis() -> GAPopulationSortBasis
    {
        GAPopulationSortBasis::Fitness
    }

    fn max_score(pop: &GAPopulation<T>) -> f32
    {
        Self::score(pop.best_by_fitness_score())
    }

    fn min_score(pop: &GAPopulation<T>) -> f32
    {
        Self::score(pop.worst_by_fitness_score())
    }

    fn iterator<'a>(pop: &'a GAPopulation<T>) -> Box<Iterator<Item=&'a T> + 'a>
    {
        Box::new(pop.fitness_score_iterator())
    }
}

/// Rank selector.
///
/// Select the best individual of the population. If more than 1 share the
/// best score, choose 1 among them at random.
pub struct GARankSelector;

impl GARankSelector
{
    pub fn new() -> GARankSelector
    {
        GARankSelector
    }
}

impl<T: GAIndividual> GASelector<T> for GARankSelector
{
    fn update<S: GAScoreSelection<T>>(&mut self, pop: &mut GAPopulation<T>)
    {
        pop.sort();
    }

    fn select<'a, S: GAScoreSelection<T>>(&self, pop: &'a GAPopulation<T>, rng_ctx: &mut GARandomCtx) -> &'a T
    {
        // All individuals that share the best score will be considered for selection.
        let best_score: f32 = S::max_score(pop);

        // Collect all individuals that share the best score.
        let best_inds: Vec<&T> = S::iterator(pop).take_while(|ind| S::score(ind) == best_score).collect();

        // Select 1 from them at random.
        best_inds[rng_ctx.gen_range(0, best_inds.len())]
    }
}

/// Uniform selector.
///
/// Select an individual at random, with equal probability.
pub struct GAUniformSelector;

impl GAUniformSelector
{
    pub fn new() -> GAUniformSelector
    {
        GAUniformSelector
    }
}

impl<T: GAIndividual> GASelector<T> for GAUniformSelector
{
    fn update<S: GAScoreSelection<T>>(&mut self, pop: &mut GAPopulation<T>)
    {
        // Need to sort first, because GAPopulation.individual() draws individuals
        // from the sorted lists.
        pop.sort();
    }

    // Select any individual at random.
    fn select<'a, S: GAScoreSelection<T>>(&self, pop: &'a GAPopulation<T>, rng_ctx: &mut GARandomCtx) -> &'a T
    {
        // Since selection is at random, it doesn't matter where the individual
        // is drawn from, the raw-score-sorted or the fitness-score-sorted list.
        pop.individual(
            rng_ctx.gen_range(0, pop.size()),
            GAPopulationSortBasis::Raw)
    }
}

/// Roulette Wheel selector.
///
/// Select an individual at random, each one having a probability of selection
/// that is proportional to its score according to ranking (LowIsBest or 
/// HighIsBest). 
pub struct GARouletteWheelSelector
{
    wheel_proportions: Vec<f32>,
}

impl GARouletteWheelSelector
{
    pub fn new(p_size: usize) -> GARouletteWheelSelector
    {
        let wheel_size = p_size;

        GARouletteWheelSelector
        {
            wheel_proportions: vec![0.0; wheel_size],
        }
    }
}

impl<T: GAIndividual> GASelector<T> for GARouletteWheelSelector
{
    fn update<S: GAScoreSelection<T>>(&mut self, pop: &mut GAPopulation<T>)
    {
        if pop.size() != self.wheel_proportions.len()
        {
            self.wheel_proportions.resize(pop.size(), 0.0);
        }

        pop.sort();

        let wheel_slots = self.wheel_proportions.len();
        let max_score = S::max_score(pop);
        let min_score = S::min_score(pop);

        if max_score == min_score
        {
            // Upper bound is excluded.
            for i in 0 .. wheel_slots
            {
                self.wheel_proportions[i] = ((i+1) as f32)/(wheel_slots as f32);
            }
        }
        else if (max_score > 0.0 && min_score >= 0.0) 
                 || (max_score <= 0.0 && min_score < 0.0)
        {
            // This is not a move, but a copy.
            let population_sort_basis = S::population_sort_basis();

            match pop.order()
            {
                GAPopulationSortOrder::HighIsBest 
                =>  {
                        self.wheel_proportions[0] 
                          = S::score(
                              pop.individual(0, population_sort_basis));

                        for i in 1 .. wheel_slots
                        {
                            self.wheel_proportions[i]
                              = S::score(
                                  pop.individual(i, population_sort_basis))
                                + self.wheel_proportions[i-1]; 
                        }

                        for i in 0 .. wheel_slots
                        {
                            self.wheel_proportions[i] 
                              /= self.wheel_proportions[wheel_slots-1];
                        }
                    },
                GAPopulationSortOrder::LowIsBest
                =>  {
                        self.wheel_proportions[0] 
                          = -S::score(
                               pop.individual(0, population_sort_basis)) 
                            + max_score + min_score;

                        for i in 1 .. wheel_slots
                        {
                            self.wheel_proportions[i] 
                              = -S::score(
                                   pop.individual(i, population_sort_basis))
                                + max_score + min_score 
                                + self.wheel_proportions[i-1]; 
                        }

                        for i in 0 .. wheel_slots
                        {
                            self.wheel_proportions[i]
                              /= self.wheel_proportions[wheel_slots-1];
                        }
                    }
            }
        }
        else
        {
            // TODO: Raise error.
        }
    }

    fn select<'a, S: GAScoreSelection<T>>(&self, pop: &'a GAPopulation<T>, rng_ctx: &mut GARandomCtx) -> &'a T
    {
        let wheel_slots = self.wheel_proportions.len();
        let cutoff = rng_ctx.gen::<f32>();
        let mut lower = 0;
        let mut upper = wheel_slots-1;
        let mut i;

        while upper > lower
        {
            i = lower + (upper-lower)/2;

            assert!(i < wheel_slots);

            if self.wheel_proportions[i] > cutoff
            {
                if i > 0
                {
                    upper = i-1;
                }
                else
                {
                    upper = 0;
                }
            }
            else
            {
                lower = i+1;
            }
        }

        lower = cmp::min(wheel_slots-1, lower);

        pop.individual(lower, S::population_sort_basis())
    }
}

/// Tournament selector.
///
/// Select 2 individuals using Roulette Wheel selection and select the best of the 2.
pub struct GATournamentSelector
{
    roulette_wheel_selector: GARouletteWheelSelector,
}

impl GATournamentSelector
{
    pub fn new(p_size: usize) -> GATournamentSelector
    {
        GATournamentSelector
        {
            roulette_wheel_selector: GARouletteWheelSelector::new(p_size)
        }
    }
}

impl<T: GAIndividual> GASelector<T> for GATournamentSelector
{
    fn update<S: GAScoreSelection<T>>(&mut self, pop: &mut GAPopulation<T>)
    {
        self.roulette_wheel_selector.update::<S>(pop);
    }

    fn select<'a, S: GAScoreSelection<T>>(&self, pop: &'a GAPopulation<T>, rng_ctx: &mut GARandomCtx) -> &'a T
    {
        let low_score_individual;
        let high_score_individual;
        let individual1;
        let individual2;

        // Select 2 individuals using Roulette Wheel selection.
        individual1 = self.roulette_wheel_selector.select::<S>(pop, rng_ctx);
        individual2 = self.roulette_wheel_selector.select::<S>(pop, rng_ctx);

        if S::score(individual1) 
           >= S::score(individual2)
        {
            low_score_individual = individual2;
            high_score_individual = individual1;
        }
        else
        {
            low_score_individual = individual1;
            high_score_individual = individual2;
        }

        // Return the individual that is best according to population rank.
        match pop.order()
        {
            GAPopulationSortOrder::HighIsBest => high_score_individual,
            GAPopulationSortOrder::LowIsBest  => low_score_individual
        } 
    }
}


////////////////////////////////////////
// Tests
#[cfg(test)]
mod test
{
    use super::super::ga_core::*;
    use super::super::ga_population::*;
    use super::super::ga_random::*;
    use super::super::ga_test::*;
    use super::*;

    #[test]
    #[allow(unused_variables)]
    fn test_rank_selector()
    {
        ga_test_setup("ga_selectors::test_rank_selector");
        let f = GA_TEST_FITNESS_VAL;
        let f_m = GA_TEST_FITNESS_VAL - 1.0;
        let i_f = 1.0 / f;
        let i_f_m = 1.0 / f_m;

        let mut population
          = GAPopulation::new(vec![GATestIndividual::new(f),
                                   GATestIndividual::new(f_m)],
                              GAPopulationSortOrder::HighIsBest);

        {
            let mut raw_rank_selector = GARankSelector::new();

            raw_rank_selector.update::<GARawScoreSelection>(&mut population);

            // Best Raw score is that of 1st individual.
            assert_eq!(raw_rank_selector.select::<GARawScoreSelection>(&population, &mut GARandomCtx::new_unseeded(String::from("test_rank_selector_rng"))).raw(), f);
        }

        {
            let mut fitness_rank_selector = GARankSelector::new();

            fitness_rank_selector.update::<GAFitnessScoreSelection>(&mut population);

            assert_eq!(fitness_rank_selector.select::<GAFitnessScoreSelection>(&population, &mut GARandomCtx::new_unseeded(String::from("test_rank_selector_rng"))).fitness(), i_f_m);
        }
        ga_test_teardown();
    }

    #[test]
    fn test_uniform_selector()
    {
        ga_test_setup("ga_selectors::test_uniform_selector");
        let f = GA_TEST_FITNESS_VAL;
        let f_m = GA_TEST_FITNESS_VAL - 1.0;

        let mut population
          = GAPopulation::new(vec![GATestIndividual::new(f),
                                   GATestIndividual::new(f_m)],
                              GAPopulationSortOrder::HighIsBest);

        let mut uniform_selector = GAUniformSelector::new();

        uniform_selector.update::<GARawScoreSelection>(&mut population);

        let selected_individual = uniform_selector.select::<GARawScoreSelection>(&population, &mut GARandomCtx::new_unseeded(String::from("test_rank_selector_rng")));
        assert!(selected_individual.raw() == f || selected_individual.raw() == f_m);  
        ga_test_teardown();
    }

    #[test]
    #[allow(unused_variables)]
    fn test_roulette_wheel_selector()
    {
        ga_test_setup("ga_selectors::test_roulette_wheel_selector");
        // Just exercise the code.
        // TODO: How to test when there is randomness?

        let mut individuals = vec![];
        let mut rng_ctx = GARandomCtx::new_unseeded(String::from("test_roulette_wheel_selector_rng"));


        for i in 1 .. 20
        {
            individuals.push(GATestIndividual::new(rng_ctx.gen::<f32>()));
        }

        let mut population
          = GAPopulation::new(individuals, GAPopulationSortOrder::LowIsBest);

        {
            let mut raw_roulette_wheel_selector 
              = GARouletteWheelSelector::new(population.size());

            raw_roulette_wheel_selector.update::<GARawScoreSelection>(&mut population);

            raw_roulette_wheel_selector.select::<GARawScoreSelection>(&population, &mut rng_ctx);
        }
        
        {
            let mut fitness_roulette_wheel_selector 
              = GARouletteWheelSelector::new(population.size());

            fitness_roulette_wheel_selector.update::<GAFitnessScoreSelection>(&mut population);

            fitness_roulette_wheel_selector.select::<GAFitnessScoreSelection>(&population, &mut rng_ctx);
        }
        ga_test_teardown();
    }

    #[test]
    #[allow(unused_variables)]
    fn test_tournament_selector()
    {
        // Just exercise the code.
        // TODO: How to test when there is randomness?

        ga_test_setup("ga_selectors::test_tournament_selector");
        let mut individuals = vec![];
        let mut rng_ctx = GARandomCtx::new_unseeded(String::from("test_tournament_selector_rng"));

        for i in 1 .. 20
        {
            individuals.push(GATestIndividual::new(rng_ctx.gen::<f32>()));
        }

        let mut population
          = GAPopulation::new(individuals, GAPopulationSortOrder::LowIsBest);

        {
            let mut raw_tournament_selector 
              = GARouletteWheelSelector::new(population.size());

            raw_tournament_selector.update::<GARawScoreSelection>(&mut population);

            raw_tournament_selector.select::<GARawScoreSelection>(&population, &mut rng_ctx);
        }

        {
            let mut fitness_tournament_selector 
              = GARouletteWheelSelector::new(population.size());

            fitness_tournament_selector.update::<GAFitnessScoreSelection>(&mut population);

            fitness_tournament_selector.select::<GAFitnessScoreSelection>(&population, &mut rng_ctx);
        }
        ga_test_teardown();
    }
}
