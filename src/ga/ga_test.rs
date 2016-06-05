// Copyright 2016 Revolution Solid & Contributors.
// author(s): sysnett
// rust-monster is licensed under a MIT License.

//! GA Test Utilities
//! Reusable classes for testing

use super::ga_core::*;
use super::ga_population::*;

#[cfg(test)]
extern crate env_logger;
pub const GA_TEST_FITNESS_VAL: f32 = 3.14159;

/// GA Test Setup
/// Utility function to setup useful test systems (like logging)
pub fn ga_test_setup(test_name: &str)
{
    ga_test_setup_internal(test_name);
}

#[cfg(not(test))]
fn ga_test_setup_internal(_: &str)
{
    //This only exists to avoid bringing env_logger in non_test builds
}

#[cfg(test)]
fn ga_test_setup_internal(test_name: &str)
{
    let _ = env_logger::init();
    debug!("{:?}", test_name);
}


/// GA Test Teardown
/// Utlity function to teardown used test systems
pub fn ga_test_teardown(){}


/// GATestSolution
/// Implements the GASolution Trait with a only no-ops
pub struct GATestSolution
{
    fitness: f32
}
impl GASolution for GATestSolution 
{
    fn new(f:f32) -> GATestSolution
    {
        GATestSolution{ fitness: f}
    }

    fn clone(&self) -> Self { GATestSolution::new(self.fitness) }
    fn evaluate(&mut self) -> f32 { self.fitness }
#[allow(unused_variables)]
    fn crossover(&self, other : &Self) -> Self { GATestSolution::new(self.fitness) }
#[allow(unused_variables)]
    fn mutate(&mut self, pm : f32) {}
    fn fitness(&self) -> f32 { (1.0 / self.fitness) }
    fn score(&self) -> f32 { self.fitness }
}

#[allow(dead_code)]
pub struct GATestFactory
{
    starting_fitness: f32
}
impl GATestFactory
{
    pub fn new(starting_fitness: f32) -> GATestFactory
    {
        GATestFactory {starting_fitness: starting_fitness}
    }
}
impl GAFactory<GATestSolution> for GATestFactory
{
    fn initial_population(&mut self) -> GAPopulation<GATestSolution>
    {
        GAPopulation::new(vec![GATestSolution { fitness: self.starting_fitness }], GAPopulationSortOrder::HighIsBest)
    }
}
