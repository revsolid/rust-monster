// Copyright 2016 Revolution Solid & Contributors.
// author(s): carlos-lopez-garces, sysnett
// rust-monster is licensed under an MIT License.

//! GA Selectors
//!
//! A selector represents and performs a method of selection.
//!
//! Selection is the action of choosing solutions (individuals) of the current
//! generation that will create offspring for the next generation.
//!
//! Selectors represent and perform a different method of selection each. The
//! expectation is that the offspring solutions be fitter than their selected
//! parents. For this reason, many of the selectors tend to choose the fitter
//! most of the time. However, many of them acknowledge the need for selecting
//! less fit solutions, too: A genetic operator (crossover, mutation) used on
//! suboptimal solutions may sometimes produce a solution that is fitter than
//! those that could be produced by optimal ones.
//!
//! Available selectors:
//!
//! `GARankSelector`
//! `GAUniformSelector`
//! `GARouletteWheelSelector`
//! `GATournamentSelector`
//!
//! # Examples
use super::ga_core::GASolution;
use super::ga_population::{GAPopulation, GAPopulationSortBasis, GAPopulationSortOrder};
use super::ga_random::{GARandomCtx};
use std::cmp;

/// Selector Trait
///
/// Interface to Selection Schemes
pub trait GASelector<'a, T: GASolution>
{
    /// Update internal state. 
    /// Some selectors implement an empty update().
    fn update(&mut self, population: &mut GAPopulation<T>) {}

    fn select(&self, population: &'a GAPopulation<T>, rng_ctx: &mut GARandomCtx) -> &'a T;
}

///
///
/// Selectors are configured, at the time of creation, with the type of score
/// {RAW, SCALED} they will use to perform selections. The type of score
/// ultimately determines the function that will be invoked on the `GASolution`
/// to obtain the score value of the configured type. `GAScoreTypeBasedSelection`
/// objects provide a unified interface to the different score functions of a
/// `GASolution`. Selectors use these objects to obtain score values of the
/// configured type, without choosin
pub trait GAScoreTypeBasedSelection<T: GASolution>
{
    fn score(&self, individual: &T) -> f32;

    fn population_sort_basis(&self) -> GAPopulationSortBasis;

    fn max_score(&self, population: &GAPopulation<T>) -> f32;

    fn min_score(&self, population: &GAPopulation<T>) -> f32;
}

pub struct GARawScoreBasedSelection;

impl<T: GASolution> GAScoreTypeBasedSelection<T> for GARawScoreBasedSelection
{
    fn score(&self, individual: &T) -> f32
    {
        individual.score()
    }

    fn population_sort_basis(&self) -> GAPopulationSortBasis
    {
        GAPopulationSortBasis::Raw
    }

    fn max_score(&self, population: &GAPopulation<T>) -> f32
    {
        self.score(population.best_by_raw_score())
    }

    fn min_score(&self, population: &GAPopulation<T>) -> f32
    {
        self.score(population.worst_by_raw_score())
    }
}

pub struct GAScaledScoreBasedSelection;

impl<T: GASolution> GAScoreTypeBasedSelection<T> for GAScaledScoreBasedSelection
{
    fn score(&self, individual: &T) -> f32
    {
        individual.fitness()
    }

    fn population_sort_basis(&self) -> GAPopulationSortBasis
    {
        GAPopulationSortBasis::Scaled
    }

    fn max_score(&self, population: &GAPopulation<T>) -> f32
    {
        self.score(population.best_by_scaled_score())
    }

    fn min_score(&self, population: &GAPopulation<T>) -> f32
    {
        self.score(population.worst_by_scaled_score())
    }
}

// GASolution-s live as long as the population. Lifetime 'a is bound to the
// population borrowed by the 'population' member, as well as to the enclosed
// GASolution-s, because they share the same memory.
pub struct GARankSelector<'a, T: 'a + GASolution>
{
    // Q: How to make this member common to all GASelectors?
    // Q: Will 'mut' make things more complex? When the rank selector is in
    //    scope and borrowing the population, no other objects will be able
    //    to borrow it, not even in non-mut mode according to the rules.
    // Selectors modify populations (they sort them, for instance), so the
    // reference must be 'mut'.
    // population: &'a mut GAPopulation<T>,

    // TODO: Check correct lifetime.
    score_selection: &'a GAScoreTypeBasedSelection<T>
}

impl<'a, T: GASolution> GARankSelector<'a, T>
{
    pub fn new(s: &'a GAScoreTypeBasedSelection<T>) -> GARankSelector<'a, T>
    {
        GARankSelector
        {
            score_selection: s
        }
    }
}

// TODO: DOC.
impl<'a, T: GASolution> GASelector<'a, T> for GARankSelector<'a, T>
{
    fn update(&mut self, population: &mut GAPopulation<T>)
    {
        population.sort();
    }

    fn select(&self, population: &'a GAPopulation<T>, rng_ctx: &mut GARandomCtx) -> &'a T
    {
        // TODO: Confirm assumption that population has 1 individual at least.
        // Number of individuals that share best score.
        let mut best_count = 1;

        // This is not a move, but a copy.
        let population_sort_basis = self.score_selection.population_sort_basis();

        // TODO: Use best() when implemented.
        // Q: Should individual() (ith_best()) return an optional or
        //    does it guarantee that it will always return something valid?
        //    What if the population is still empty?
        let best_score: f32 = self.score_selection.score(population.individual(0, population_sort_basis));

        // Skip 0th best.
        for i in 1..population.size()
        {
            if self.score_selection.score(population.individual(i, population_sort_basis)) != best_score
            {
                break;
            }

            best_count = best_count + 1;
        }

        // Select any individual from those that share best score.
        population.individual(rng_ctx.gen_range(0, best_count), population_sort_basis)
    }
}

pub struct GAUniformSelector;

impl GAUniformSelector
{
    pub fn new() -> GAUniformSelector
    {
        GAUniformSelector
    }
}

impl<'a, T: GASolution> GASelector<'a, T> for GAUniformSelector
{
    fn update(&mut self, population: &mut GAPopulation<T>)
    {
        // Need to sort first, because GAPopulation.individual() draws individuals
        // from the sorted lists.
        population.sort();
    }

    // Select any individual at random.
    fn select(&self, population: &'a GAPopulation<T>, rng_ctx: &mut GARandomCtx) -> &'a T
    {
        // Since selection is at random, it doesn't matter where the individual
        // is drawn from, the Raw/score-sorted or the Scaled/fitness-sorted list.
        population.individual(
            rng_ctx.gen_range(0, population.size()),
            GAPopulationSortBasis::Raw)
    }
}

pub struct GARouletteWheelSelector<'a, T: 'a + GASolution>
{
    // TODO: Check correct lifetime.
    score_selection: &'a GAScoreTypeBasedSelection<T>,

    wheel_proportions: Vec<f32>,

    // TODO: Remove if not useful.
    wheel_is_dirty: bool
}

impl<'a, T: GASolution> GARouletteWheelSelector<'a, T>
{
    // TODO: Check s's lifetime.
    pub fn new(s: &'a GAScoreTypeBasedSelection<T>, p_size: usize) -> GARouletteWheelSelector<'a, T>
    {
        // TODO: Comment doesn't look correct.
        // vec![] borrows references (invocation of size() is through *p, or so the
        // compiler says); since p has already been borrowed as a mutable reference
        // (no data races allowed), p.size() can't be passed to vec![].
        let wheel_size = p_size;

        GARouletteWheelSelector
        {
            score_selection: s,
            wheel_proportions: vec![0.0; wheel_size],
            wheel_is_dirty: false
        }
    }
}

impl<'a, T: GASolution> GASelector<'a, T> for GARouletteWheelSelector<'a, T>
{
    fn update(&mut self, population: &mut GAPopulation<T>)
    {
        // TODO: Can a population grow? If it can, need to resize the wheel.
        if population.size() != self.wheel_proportions.len()
        {
            self.wheel_proportions.resize(population.size(), 0.0);
        }


        population.sort();

        let wheel_slots = self.wheel_proportions.len();
        let max_score = self.score_selection.max_score(population);
        let min_score = self.score_selection.min_score(population);

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
            let population_sort_basis = self.score_selection.population_sort_basis();

            match population.order()
            {
                GAPopulationSortOrder::HighIsBest 
                =>  {
                        self.wheel_proportions[0] 
                          = self.score_selection.score(
                              population.individual(0, population_sort_basis));

                        for i in 1 .. wheel_slots
                        {
                            self.wheel_proportions[i]
                              = self.score_selection.score(
                                  population.individual(i, population_sort_basis))
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
                          = -self.score_selection.score(
                               population.individual(0, population_sort_basis)) 
                            + max_score + min_score;

                        for i in 1 .. wheel_slots
                        {
                            self.wheel_proportions[i] 
                              = -self.score_selection.score(
                                   population.individual(i, population_sort_basis))
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

    fn select(&self, population: &'a GAPopulation<T>, rng_ctx: &mut GARandomCtx) -> &'a T
    {
        // TODO: Cache this value? Or Vec already caches it?
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

        population.individual(lower, self.score_selection.population_sort_basis())
    }
}

pub struct GATournamentSelector<'a, T: 'a + GASolution>
{
    // TODO: Check correct lifetime.
    score_selection: &'a GAScoreTypeBasedSelection<T>,
    roulette_wheel_selector: GARouletteWheelSelector<'a, T>,
}

impl<'a, T: GASolution> GATournamentSelector<'a, T>
{
    // TODO: Check s's lifetime.
    pub fn new(s: &'a GAScoreTypeBasedSelection<T>, p_size: usize) -> GATournamentSelector<'a, T>
    {
        GATournamentSelector
        {
            score_selection: s,
            roulette_wheel_selector: GARouletteWheelSelector::new(s, p_size)
        }
    }
}

impl<'a, T: GASolution> GASelector<'a, T> for GATournamentSelector<'a, T>
{
    fn update(&mut self, population: &mut GAPopulation<T>)
    {
        self.roulette_wheel_selector.update(population);
    }

    fn select(&self, population: &'a GAPopulation<T>, rng_ctx: &mut GARandomCtx) -> &'a T
    {
        let low_score_individual;
        let high_score_individual;
        let individual1;
        let individual2;

        individual1 = self.roulette_wheel_selector.select(population, rng_ctx);
        individual2 = self.roulette_wheel_selector.select(population, rng_ctx);

        if self.score_selection.score(individual1) 
           >= self.score_selection.score(individual2)
        {
            low_score_individual = individual2;
            high_score_individual = individual1;
        }
        else
        {
            low_score_individual = individual1;
            high_score_individual = individual2;
        }

        match population.order()
        {
            GAPopulationSortOrder::HighIsBest => high_score_individual,
            GAPopulationSortOrder::LowIsBest  => low_score_individual
        } 
    }
}
