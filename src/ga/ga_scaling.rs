// Copyright 2016 Revolution Solid & Contributors.
// author(s): sysnett, carlos-lopez-garces
// rust-monster is licensed under an MIT License.

//! GA Scaling Schemes
//!
//! Scheme to

use super::ga_core::GASolution;
use super::ga_population::GAPopulation;

/// Scaling Scheme Trait
/// 
/// Embeeded in the population, scales the values of raw score in a
/// GASolution to set their fitness score
pub trait GAScaling<T: GASolution>
{
    fn evaluate(&self, pop: &mut GAPopulation<T>);
}

/// No Scaling - Raw and Scaled are the same
struct GANoScaling
{
}

impl<T: GASolution> GAScaling<T> for GANoScaling
{
    fn evaluate(&self, pop: &mut GAPopulation<T>)
    {
        // TODO: This is why we need iterators :(
        let mut pop_vec = pop.population();
        for ref mut ind in pop_vec
        {
           ind.set_fitness(ind.score()); 
        }
    }
}

/// Linear Scaling
/// Uses a simple a*fitness + b scaling.
/// a and b are the intersect of the linear function and are calculated
/// based on Goldberg's implementation from his book
struct GALinearScaling
{
    multiplier: f32
}

const GA_LINEAR_SCALING_MULTIPLIER : f32 = 2.0;
impl GALinearScaling
{
    fn new(scaling: f32) -> GALinearScaling
    {
        GALinearScaling{ multiplier: scaling }
    }

    fn prescale(&self, max: f32, min: f32, avg: f32) -> (f32, f32)
    {
        let m = self.multiplier;
        let a;
        let b;
        let delta;

        if min > ( (m*avg - max) / (m - 1.0) )
        {
            delta = max - avg;
            a = avg / delta;
            b = avg * (max - m*avg) / delta;
        }
        else
        {
            delta = avg - min;
            a = avg / delta;
            b = (-1.0*min*avg) / delta;
        }

        (a, b)
    }
}

impl<T: GASolution> GAScaling<T> for GALinearScaling
{
    fn evaluate(&self, pop : &mut GAPopulation<T>)
    {
        let mut pop_vec = pop.population();

        // TODO FIX THIS MESS 
        // max(), min() and avg() should exist in GAPopulation
        let mut max = pop_vec[0].score();
        let mut min = pop_vec[pop_vec.len()-1].score();
        if min > max 
        {
            let mut t = min;
            min = max;
            max = t;
        }

        let avg = (max - min) / 2.0;
        // ~TODO

        let (a, b) = self.prescale(max, min, avg);

        for ref mut ind in pop_vec
        {
           ind.set_fitness(a*ind.score()+b); 
        }
    }
}


#[cfg(test)]
mod test
{}
