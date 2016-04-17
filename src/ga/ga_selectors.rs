// TODO: COPYRIGHT, USE & AUTHORS
use super::ga_core::GASolution;
use super::ga_population::{GAPopulation, GAPopulationSortBasis, GAPopulationSortOrder};
use super::ga_random;
use std::cmp;

/// Selector Trait
///
/// Interface to Selection Schemes
pub trait GASelector<'a, T: GASolution>
{
    /// Assign the population on which to operate
    fn assign(&mut self, population: &'a mut GAPopulation<T>);
    /// Update internal state. 
    /// Some selectors implement an empty update().
    fn update(&mut self) {}

    fn select(&self) -> &T;
}

pub trait GAScoreTypeBasedSelection<T: GASolution>
{
    fn score(&self, individual: &T) -> f32;

    fn population_sort_basis(&self) -> GAPopulationSortBasis;

    fn max_score(&self, population: &GAPopulation<T>) -> f32
    {
        // TODO: Knowing that best is at 0 is knowing too much.
        self.score(population.individual(0, self.population_sort_basis()))
    }

    fn min_score(&self, population: &GAPopulation<T>) -> f32
    {
        // TODO: Knowing that worst is at size()-1 is knowing too much.
        self.score(population.individual(population.size()-1, self.population_sort_basis()))
    }
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
    population: &'a mut GAPopulation<T>,

    // TODO: Check correct lifetime.
    score_selection: &'a GAScoreTypeBasedSelection<T>
}

impl<'a, T: GASolution> GARankSelector<'a, T>
{
    pub fn new(p: &'a mut GAPopulation<T>, s: &'a GAScoreTypeBasedSelection<T>) -> GARankSelector<'a, T>
    {
        GARankSelector
        {
            population: p,
            score_selection: s
        }
    }
}

// TODO: DOC.
impl<'a, T: GASolution> GASelector<'a, T> for GARankSelector<'a, T>
{
    fn assign(&mut self, population: &'a mut GAPopulation<T>)
    {
        self.population = population;
    }

    fn update(&mut self)
    {
        self.population.sort();
    }

    fn select(&self) -> &T
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
        let best_score: f32 = self.score_selection.score(self.population.individual(0, population_sort_basis));

        // Skip 0th best.
        for i in 1..self.population.size()
        {
            if self.score_selection.score(self.population.individual(i, population_sort_basis)) != best_score
            {
                break;
            }

            best_count = best_count + 1;
        }

        // Select any individual from those that share best score.
        self.population.individual(ga_random::ga_random_range(0, best_count), population_sort_basis)
    }
}

pub struct GAUniformSelector<'a, T: 'a + GASolution>
{
    population: &'a mut GAPopulation<T>
}

impl<'a, T: GASolution> GAUniformSelector<'a, T>
{
    pub fn new(p: &'a mut GAPopulation<T>) -> GAUniformSelector<'a, T>
    {
        GAUniformSelector
        {
            population: p
        }
    }
}

impl<'a, T: GASolution> GASelector<'a, T> for GAUniformSelector<'a, T>
{
    fn assign(&mut self, population: &'a mut GAPopulation<T>)
    {
        self.population = population;
    }

    fn update(&mut self)
    {
        // Need to sort first, because GAPopulation.individual() draws individuals
        // from the sorted lists.
        self.population.sort();
    }

    // Select any individual at random.
    fn select(&self) -> &T
    {
        // Since selection is at random, it doesn't matter where the individual
        // is drawn from, the Raw/score-sorted or the Scaled/fitness-sorted list.
        self.population.individual(
            ga_random::ga_random_range(0, self.population.size()),
            GAPopulationSortBasis::Raw)
    }
}

pub struct GARouletteWheelSelector<'a, T: 'a + GASolution>
{
    population: &'a mut GAPopulation<T>,

    // TODO: Check correct lifetime.
    score_selection: &'a GAScoreTypeBasedSelection<T>,

    wheel_proportions: Vec<f32>,

    // TODO: Remove if not useful.
    wheel_is_dirty: bool
}

impl<'a, T: GASolution> GARouletteWheelSelector<'a, T>
{
    // TODO: Check s's lifetime.
    pub fn new(p: &'a mut GAPopulation<T>, 
               s: &'a GAScoreTypeBasedSelection<T>) -> GARouletteWheelSelector<'a, T>
    {
        // TODO: Comment doesn't look correct.
        // vec![] borrows references (invocation of size() is through *p, or so the
        // compiler says); since p has already been borrowed as a mutable reference
        // (no data races allowed), p.size() can't be passed to vec![].
        let wheel_size = p.size();

        GARouletteWheelSelector
        {
            population: p,
            score_selection: s,
            wheel_proportions: vec![0.0; wheel_size],
            wheel_is_dirty: false
        }
    }
}

impl<'a, T: GASolution> GASelector<'a, T> for GARouletteWheelSelector<'a, T>
{
    fn assign(&mut self, population: &'a mut GAPopulation<T>)
    {
        self.population = population;

        if self.population.size() != self.wheel_proportions.len()
        {
            self.wheel_proportions.resize(self.population.size(), 0.0);
        }

        self.wheel_is_dirty = true;
    }

    fn update(&mut self)
    {
        // TODO: Can a population grow? If it can, need to resize the wheel.

        let wheel_slots = self.wheel_proportions.len();
        let max_score = self.score_selection.max_score(self.population);
        let min_score = self.score_selection.min_score(self.population);

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

            self.population.sort();

            match self.population.order()
            {
                GAPopulationSortOrder::HighIsBest 
                =>  {
                        self.wheel_proportions[0] 
                          = self.score_selection.score(
                              self.population.individual(0, population_sort_basis));

                        for i in 1 .. wheel_slots
                        {
                            self.wheel_proportions[i]
                              = self.score_selection.score(
                                  self.population.individual(i, population_sort_basis))
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
                               self.population.individual(0, population_sort_basis)) 
                            + max_score + min_score;

                        for i in 1 .. wheel_slots
                        {
                            self.wheel_proportions[i] 
                              = -self.score_selection.score(
                                   self.population.individual(i, population_sort_basis))
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

    fn select(&self) -> &T
    {
        // TODO: Cache this value? Or Vec already caches it?
        let wheel_slots = self.wheel_proportions.len();
        let cutoff = ga_random::ga_random_float();
        let mut lower = 0;
        let mut upper = wheel_slots-1;
        let mut i;

        while upper >= lower
        {
            i = lower + (upper-lower)/2;

            assert!(i >= 0 && i < wheel_slots);

            if self.wheel_proportions[i] > cutoff
            {
                upper = i-1;
            }
            else
            {
                lower = i+1;
            }
        }

        lower = cmp::min(wheel_slots-1, lower);
        lower = cmp::max(0, lower);

        self.population.individual(lower, self.score_selection.population_sort_basis())
    }
}

pub struct GATournamentSelector<'a, T: 'a + GASolution>
{
    // TODO: Check correct lifetime.
    score_selection: &'a GAScoreTypeBasedSelection<T>,

    roulette_wheel_selector: GARouletteWheelSelector<'a, T>,

    population_sort_order: GAPopulationSortOrder
}

impl<'a, T: GASolution> GATournamentSelector<'a, T>
{
    // TODO: Check s's lifetime.
    pub fn new(p: &'a mut GAPopulation<T>, 
               s: &'a GAScoreTypeBasedSelection<T>) -> GATournamentSelector<'a, T>
    {
        GATournamentSelector
        {
            score_selection: s,
            population_sort_order: p.order(),
            roulette_wheel_selector: GARouletteWheelSelector::new(p, s)
        }
    }
}

impl<'a, T: GASolution> GASelector<'a, T> for GATournamentSelector<'a, T>
{
    fn assign(&mut self, population: &'a mut GAPopulation<T>)
    {
        self.population_sort_order = population.order();

        self.roulette_wheel_selector.assign(population);
    }

    fn update(&mut self)
    {
        self.roulette_wheel_selector.update();
    }

    fn select(&self) -> &T
    {
        let low_score_individual;
        let high_score_individual;
        let individual1;
        let individual2;

        individual1 = self.roulette_wheel_selector.select();

        individual2 = self.roulette_wheel_selector.select();

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

        match self.population_sort_order
        {
            GAPopulationSortOrder::HighIsBest => high_score_individual,
            GAPopulationSortOrder::LowIsBest  => low_score_individual
        } 
    }
}
