// TODO: COPYRIGHT, USE & AUTHORS
use super::ga_core::GASolution;
use super::ga_population::{GAPopulation, GAPopulationSortBasis};
use super::ga_random;

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

    fn select(&mut self) -> &T;
}

pub trait GAScoreTypeBasedSelection<T: GASolution>
{
    fn score(&self, individual: &T) -> f32;

    fn population_sort_basis(&self) -> GAPopulationSortBasis;
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
    score_type_selection: &'a GAScoreTypeBasedSelection<T>
}

impl<'a, T: GASolution> GARankSelector<'a, T>
{
    pub fn new(p: &'a mut GAPopulation<T>, s: &'a GAScoreTypeBasedSelection<T>) -> GARankSelector<'a, T>
    {
        GARankSelector
        {
            population: p,
            score_type_selection: s
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

    fn select(&mut self) -> &T
    {
        // TODO: Confirm assumption that population has 1 individual at least.
        // Number of individuals that share best score.
        let mut best_count = 1;

        self.population.sort();

        // TODO: Use best() when implemented.
        // Q: Should individual() (ith_best()) return an optional or
        //    does it guarantee that it will always return something valid?
        //    What if the population is still empty?
        let best_score: f32 = self.score_type_selection.score(self.population.individual(0, self.score_type_selection.population_sort_basis()));

        // Skip 0th best.
        for i in 1..self.population.size()
        {
            if self.score_type_selection.score(self.population.individual(i, self.score_type_selection.population_sort_basis())) != best_score
            {
                break;
            }

            best_count = best_count + 1;
        }

        // Select any individual from those that share best score.
        self.population.individual(ga_random::ga_random_range(0, best_count), self.score_type_selection.population_sort_basis())
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

    // Select any individual at random.
    fn select(&mut self) -> &T
    {
        // Need to sort first, because GAPopulation.individual() draws individuals
        // from the sorted lists.
        // TODO: GAPopulation should implement a method for selecting individuals
        // from the unsorted vector; GAUniformSelector would use it, because it
        // doesn't care about the order.
        self.population.sort();

        // Since selection is at random, it doesn't matter where the individual
        // is drawn from, the Raw/score-sorted or the Scaled/fitness-sorted list.
        self.population.individual(
            ga_random::ga_random_range(0, self.population.size()),
            GAPopulationSortBasis::Raw)
    }
}

//pub struct GARouletteWheelSelector<'a, T: 'a + GASolution>
//{
//    population: &'a mut GAPopulation<T>,
//
//    pop_sort_basis: GAPopulationSortBasis,
//
//    wheel_proportions: Vec<f32>,
//
//    wheel_is_dirty: bool
//}
//
//impl<'a, T: GASolution> GASelector<'a, T> for GARouletteWheelSelector<'a, T>
//{
//    fn assign(&mut self, population: &'a mut GAPopulation<T>)
//    {
//        self.population = population;
//
//        if self.population.size() != self.wheel_proportions.len()
//        {
//            self.wheel_proportions.resize(self.population.size(), 0.0);
//        }
//
//        self.wheel_is_dirty = true;
//    }
//
//    fn select(&mut self) -> &T
//    {
//        // Dummy.
//        self.population.individual(0, GAPopulationSortBasis::Raw)
//    }
//
//    fn update(&mut self)
//    {
//        // TODO: Replace with cached stats when implemented.
//        // TODO: Replace with best() and worst() once there is one for Raw and one for Scaled.
//        let max =  match self.pop_sort_basis
//                   { 
//                        GAPopulationSortBasis::Raw    => self.population.individual(0, GAPopulationSortBasis::Raw).score(),
//                        GAPopulationSortBasis::Scaled => self.population.individual(0, GAPopulationSortBasis::Scaled).fitness()
//                   };
//
//        let min =  match self.pop_sort_basis 
//                   {
//                        GAPopulationSortBasis::Raw    => self.population.individual(self.population.size()-1, GAPopulationSortBasis::Raw).score(),
//                        GAPopulationSortBasis::Scaled => self.population.individual(self.population.size()-1, GAPopulationSortBasis::Scaled).fitness()
//                   };
//    }
//}
