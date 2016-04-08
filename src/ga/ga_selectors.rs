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

    fn select(&mut self, pop_sort_basis: GAPopulationSortBasis) -> &T;
}

// Do we need {RAW, SCALED} enum? Why not just boolean?

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
    // TODO: Implement new() function and drop 'pub' from 'population'.
    pub population: &'a mut GAPopulation<T>
}

// TODO: DOC.
impl<'a, T: GASolution> GASelector<'a, T> for GARankSelector<'a, T>
{
    fn assign(&mut self, population: &'a mut GAPopulation<T>)
    {
        self.population = population;
    }

    fn select(&mut self, pop_sort_basis: GAPopulationSortBasis) -> &T
    {
        // Number of individuals that share best score/fitness.
        let mut best_count;

        // Q: sort_int() is pub, but is it meant to be called outside of 
        // ga_populations? Ideally, pop_sort_basis would be passed to sort().
        self.population.sort();

        match pop_sort_basis
        {
            GAPopulationSortBasis::Raw
            =>  {
                    // Currently, individual(i, sort_basis) returns the ith-best
                    // individual, sort_basis-ordered.
                    // Q: Should individual() (ith_best()) return an optional or
                    //    does it guarantee that it will always return something valid?
                    //    What if the population is still empty?
                    let best_score: f32 = self.population.individual(0, GAPopulationSortBasis::Raw).score();

                    best_count = 1;

                    // Skip 0th best.
                    for i in 1..self.population.size()
                    {
                        if self.population.individual(i, GAPopulationSortBasis::Raw).score()  != best_score
                        {
                            break;
                        }

                        best_count = best_count + 1;
                    }
                } 

            GAPopulationSortBasis::Scaled
            =>  {
                    let best_fitness: f32 = self.population.individual(0, GAPopulationSortBasis::Scaled).fitness();

                    best_count = 1;

                    for i in 1..self.population.size()
                    {
                        if self.population.individual(i, GAPopulationSortBasis::Scaled).fitness() != best_fitness
                        {
                            break;
                        }

                        best_count = best_count + 1;
                    }
                }
        };

        // Select any individual from those that share best score/fitness.
        self.population.individual(ga_random::ga_random_range(0, best_count), pop_sort_basis)
    }
}
