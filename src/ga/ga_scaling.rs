// TODO: COPYRIGHT, USE & AUTHORS
use super::ga_core::GASolution;

/// Scaling Scheme Trait
///
/// Interface to a Scaling Scheme. Goes from 'raw' objective values
/// to fitness values.
pub trait GAScaling<T: GASolution>
{
    fn evaluate(pop : &mut GAPopluation<T>);
}
