// TODO: COPYRIGHT, USE & AUTHORS
use super::ga_core::GASolution;

/// Selector Trait
///
/// Interface to Selection Schemes
pub trait GASelector<T: GASolution>
{
    /// Assign the population on which to operate
    fn assign(&mut self, population :Vec<&T>);
    /// Update internal state
    fn update(&mut self);
    /// Return an individual from the population
    fn select(&mut self) -> &T;
}

// TODO: Selectors
