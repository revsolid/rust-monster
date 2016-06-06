// Copyright 2016 Revolution Solid & Contributors.
// author(s): sysnett, carlos-lopez-garces
// rust-monster is licensed under an MIT License.

//! GA Scaling Schemes
//!
//! Scales the raw score of a Population's individuals.

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
pub struct GANoScaling;

impl<T: GASolution> GAScaling<T> for GANoScaling
{
    fn evaluate(&self, pop: &mut GAPopulation<T>)
    {
        // TODO: This is why we need iterators :(
        let pop_vec = pop.population();
        for ind in pop_vec
        {
            let rs = ind.score();
            ind.set_fitness(rs); 
        }
    }
}

/// Linear Scaling
/// Uses a simple ```a*fitness + b``` scaling.
/// ```a``` and ```b``` are the intersect of the linear function and are calculated
/// based on Goldberg's book implementation
pub struct GALinearScaling
{
    multiplier: f32
}

#[allow(unused_variables)]
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

        if min > ((m*avg - max) / (m - 1.0))
        {
            delta = max - avg;
            a = (m - 1.0) * avg / delta;
            b = avg * (max - m * avg) / delta;
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
        let max = pop.best_by_raw_score().score();
        let min = pop.worst_by_raw_score().score();

        // TODO: avg should be part of GAPopulation
        let avg = (max - min) / 2.0;

        let (a, b) = self.prescale(max, min, avg);

        let pop_vec = pop.population();
        for ind in pop_vec
        {
            let rs = ind.score();
            ind.set_fitness(a*rs+b); 
        }
    }
}


////////////////////////////////////////
// Tests
#[cfg(test)]
mod test
{
    use super::*;
    use super::super::ga_core::*;
    use super::super::ga_population::*;
    use super::super::ga_test::*;
    
    #[test]
    fn no_scaling()
    {
        ga_test_setup("ga_scaling::no_scaling");
        let f = GA_TEST_FITNESS_VAL;
        let mut population = GAPopulation::new(vec![GATestSolution::new(f)], GAPopulationSortOrder::HighIsBest);
        population.sort();

        let scaler = GANoScaling{};

        scaler.evaluate(&mut population);

        assert_eq!(population.individual(0, GAPopulationSortBasis::Raw).fitness(),
                   population.individual(0, GAPopulationSortBasis::Raw).score());

        ga_test_teardown();
    }

    #[test]
    fn linear_scaling()
    {
        ga_test_setup("ga_scaling::no_scaling");
        let f = GA_TEST_FITNESS_VAL;
        let mut population = GAPopulation::new(vec![GATestSolution::new(f)], GAPopulationSortOrder::HighIsBest);
        population.sort();

        let scaler = GALinearScaling{ multiplier: super::GA_LINEAR_SCALING_MULTIPLIER };

        scaler.evaluate(&mut population);

        // TODO: Real test
        assert!(population.individual(0, GAPopulationSortBasis::Raw).fitness() !=
                population.individual(0, GAPopulationSortBasis::Raw).score());

        ga_test_teardown();
    }

}
