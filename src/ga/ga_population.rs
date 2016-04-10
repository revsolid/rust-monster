// TODO: COPYRIGHT, USE & AUTHORS

use super::ga_core::GASolution;

use std::cmp::Ordering;
use std::iter::FromIterator;

// Better name than 'Basis'?
pub enum GAPopulationSortBasis
{
    Raw,
    Scaled,
}

// Isn't a boolean enough?
pub enum GAPopulationSortOrder
{
    LowIsBest,
    HighIsBest,
}

/// Genetic Algorithm Population
// TODO: RUST DOCS!
pub struct GAPopulation<T: GASolution>
{
    population: Vec<T>,

    sort_order: GAPopulationSortOrder,

    // 'population' ordered by Raw score.
    population_order_raw: Vec<usize>,
    // Is 'population_order_raw' sorted?
    is_raw_sorted: bool,

    // 'population' ordered by Scaled score.
    population_order_scaled: Vec<usize>,
    // Is 'population_scaled' sorted?
    is_scaled_sorted: bool,

    // We keep 2 lists of indexes to the population vector.
    // One sorted by Raw Score and one by Scaled Score (Fitness).
}
impl<T: GASolution> GAPopulation<T>
{
    // Need best() and sort() for Selectors.

    // TODO: New should use some parameters, maybe a Config
    pub fn new(p: Vec<T>, order: GAPopulationSortOrder) -> GAPopulation<T>
    {
        let gap = GAPopulation 
                  {
                      population: p,
                      sort_order: order,
                      population_order_raw: vec![],
                      is_raw_sorted: false,
                      population_order_scaled: vec![],
                      is_scaled_sorted: false
                  };

        gap
    }

    pub fn population(&mut self) -> &mut Vec<T>
    {
        return &mut self.population
    }

    pub fn evaluate(&mut self)
    {
        for ref mut ind in &mut self.population
        {
            ind.evaluate();
        }
    }

    pub fn size(&self) -> usize
    {
        self.population.len()
    }

    //TODO: this is a temporary implementation
    pub fn select(&self) -> &T
    {
        self.individual(0, GAPopulationSortBasis::Scaled)
    }

    //TODO: This is a temporary implementation 
    pub fn best(&self) -> &T
    {
        // TODO: Call GAPopulation.scale().

        self.individual(0, GAPopulationSortBasis::Scaled)
    }

    //TODO: This is a temporary implementation 
    pub fn worst(&self) -> &T
    {
        self.individual(self.size()-1, GAPopulationSortBasis::Scaled)
    }

    pub fn individual(&self, i : usize, sort_basis : GAPopulationSortBasis) -> &T
    {
        // TODO: Check that i makes sense
        match sort_basis
        {
            GAPopulationSortBasis::Raw
            => { &self.population[self.population_order_raw[i]] },
            GAPopulationSortBasis::Scaled
            => { &self.population[self.population_order_scaled[i]] },
        }
    }

    pub fn sort(&mut self)
    {
        self.sort_int(false, GAPopulationSortBasis::Scaled);
        self.sort_int(false, GAPopulationSortBasis::Raw);
    }

    //TODO: I hate this name
    pub fn sort_int(&mut self, force_sort: bool, sort_basis: GAPopulationSortBasis)
    {
        let mut ordered : Vec<usize> = Vec::from_iter(0..self.size());
        match sort_basis
        {
            GAPopulationSortBasis::Raw
            =>  if (!self.is_raw_sorted) || force_sort
                {
                    match self.sort_order
                    {
                        GAPopulationSortOrder::LowIsBest =>
                        {
                            ordered.sort_by(|s1: &usize, s2: &usize|
                                            self.population[*s1].score()
                                                .partial_cmp(&self.population[*s2].score()).unwrap_or(Ordering::Equal));

                        },
                        GAPopulationSortOrder::HighIsBest =>
                        {
                            ordered.sort_by(|s1: &usize, s2: &usize|
                                            self.population[*s2].score()
                                                .partial_cmp(&self.population[*s1].score()).unwrap_or(Ordering::Equal));
                                                                  
                        },
                    };
                    self.population_order_raw = ordered;
                    self.is_raw_sorted = true;
                },

            GAPopulationSortBasis::Scaled
            =>  if (!self.is_scaled_sorted) || force_sort
                {
                    match self.sort_order
                    {
                        GAPopulationSortOrder::LowIsBest =>
                        { 
                            ordered.sort_by(|s1: &usize, s2: &usize|
                                            self.population[*s1].fitness()
                                                .partial_cmp(&self.population[*s2].fitness()).unwrap_or(Ordering::Equal));
                        },

                        GAPopulationSortOrder::HighIsBest =>
                        {
                            ordered.sort_by(|s1: &usize, s2: &usize|
                                            self.population[*s2].fitness()
                                                .partial_cmp(&self.population[*s1].fitness()).unwrap_or(Ordering::Equal));
                        }
                    };
                    self.population_order_scaled = ordered;
                    self.is_scaled_sorted = true;
                },
        };
    }
}


#[cfg(test)]
mod test
{
}
