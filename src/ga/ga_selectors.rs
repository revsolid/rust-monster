// TODO: COPYRIGHT, USE & AUTHORS
use super::ga_core::GASolution;

/// Selector Trait
///
/// Interface to Selection Schemes
pub trait GASelector<T: GASolution>
{
    fn assign(&mut self, population: GAPopulation<T>);

    fn update(&mut self);

    fn select(&mut self) -> &T;
}

// Do we need {RAW, SCALED} enum? Why not just boolean?

struct GARankSelector<T: GASolution>
{
    // Q: How to make this member common to all GASelectors?
    population: GAPopulation<T>,
}

impl GASelector<T: GASolution> for GARankSelector<T>
{
    // GASelect gets ownership of population.
    fn assign(&mut self, population: GAPopulation<T>)
    {
        self.population = population;
    }

    // GARankSelector implements an empty update().
    fn update(&mut self)
    {

    }

    fn select(&mut self) -> &T
    {

    }
}
