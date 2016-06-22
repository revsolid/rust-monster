// Copyright 2016 Revolution Solid & Contributors.
// author(s): carlos-lopez-garces, sysnett
// rust-monster is licensed under an MIT License.

use std::cmp::Ordering::*;

use ::ga::ga_core::GAIndividual;
use ::ga::ga_population::{GAPopulation, GAPopulationStats, GAPopulationSortOrder};

pub struct GAStatistics<T: GAIndividual>
{
    // All statistics collected after last reset.
    num_selections: usize,              // aka numsel
    num_crossovers: usize,              // aka numcro
    num_mutations: usize,               // aka nummut
    num_replacements: usize,            // aka numrep
    num_ind_evaluations: usize,         // aka numeval
    num_pop_evaluations: usize,         // aka numpeval

    pub cur_generation: u32,            // aka curgen
    record_frequency: u32,              // aka scoreFreq
    record_diversity: bool,             // aka dodiv

    pub alltime_best_pop: Option<GAPopulation<T>>,      // aka boa
    pub alltime_max_score: f32,                         // aka maxever
    pub alltime_min_score: f32,                         // aka minever
    pub on_performance: f32,                            // aka on
    pub off_max_performance: f32,                       // aka offmax
    pub off_min_performance: f32,                       // aka offmin

    // Call generation_statistics(1) instead.
    // init_avg_score: f32,                // aka aveInit
    // init_max_score: f32,                // aka maxInit
    // init_min_score: f32,                // aka minInit
    // init_std_dev: f32,                  // aka devInit
    // init_diversity: f32,                // aka divInit

    // Call generation_statistics(cur_generation) instead.
    // cur_avg_score: f32,                 // aka aveCur
    // cur_max_score: f32,                 // aka maxCur
    // cur_min_score: f32,                 // aka minCur
    // cur_std_dev: f32,                   // aka devCur
    // cur_diversity: f32,                 // aka divCur

    hist_stats: Vec<GAPopulationStats>,
    // num_scores: u32,                    // aka Nscrs
    // generations: Vec<i32>,              // aka gen
    // avg_scores: Vec<f32>,               // aka aveScore
    // max_scores: Vec<f32>,               // aka maxScore
    // min_scores: Vec<f32>,               // aka minScore
    // std_dev_scores: Vec<f32>,           // aka devScore
    // diversities: Vec<f32>,              // aka divScore

}

impl<T: GAIndividual> GAStatistics<T>
{
    fn new() -> GAStatistics<T>
    {
        GAStatistics
        {
            num_selections: 0,
            num_crossovers: 0,
            num_mutations: 0,
            num_replacements: 0,
            num_ind_evaluations: 0,
            num_pop_evaluations: 0,

            cur_generation: 0,
            record_frequency: 1,
            record_diversity: false,

            alltime_best_pop: None,
            alltime_max_score: 0.0,
            alltime_min_score: 0.0,
            on_performance: 0.0,
            off_max_performance: 0.0,
            off_min_performance: 0.0,

            //init_avg_score: 0.0,
            //init_max_score: 0.0,
            //init_min_score: 0.0,
            //init_std_dev: 0.0,
            //init_diversity: -1.0,

            // cur_avg_score: 0.0,
            // cur_max_score: 0.0,
            // cur_min_score: 0.0,
            // cur_std_dev: 0.0,
            // cur_diversity: -1.0,

            hist_stats: Vec::new(),
            // num_scores: 0,
            // generations: Vec::new(),
            // avg_scores: Vec::new(),
            // max_scores: Vec::new(),
            // min_scores: Vec::new(),
            // std_dev_scores: Vec::new(),
            // diversities: Vec::new(),
        }
    }

    fn update(&mut self, pop: &mut GAPopulation<T>) where T: Clone + PartialEq
    {
        match pop.statistics()
        {
            None => 
            { 
                // TODO: Handle. 
            },

            Some(stats) => 
            {
                self.cur_generation += 1;

                // TODO: Flush scores.

                self.alltime_max_score = self.alltime_max_score.max(stats.raw_max);
                self.alltime_min_score = self.alltime_min_score.min(stats.raw_min);
                self.on_performance = (self.on_performance * (self.cur_generation-1) as f32 + stats.raw_avg) / self.cur_generation as f32;
                self.off_max_performance = (self.off_max_performance * (self.cur_generation-1) as f32 + stats.raw_max) / self.cur_generation as f32;
                self.off_min_performance = (self.off_min_performance * (self.cur_generation-1) as f32 + stats.raw_min) / self.cur_generation as f32;

                // Store and compute diversity in GAPopulationStats.
                // self.cur_diversity = if self.record_diversity { pop.diversity() } else { -1.0 };

                // Update the alltime_best_pop with the input population.
                self.update_best(pop);
                
                // Archive this generation's statistics.
                self.hist_stats.push(stats);
            }
        }
    }

    fn best(&self) -> Option<GAPopulation<T>> where T: Clone
    {
        self.alltime_best_pop.clone()
    }

    // Set generation #1. Or reset to new generation #1.
    fn set_best(&mut self, mut pop: GAPopulation<T>)
    {
        match pop.statistics()
        {
            None =>
            {
                // TODO: Handle.
            },
            Some(stats) =>
            {
                self.cur_generation = 1;
                self.alltime_max_score = self.alltime_max_score.max(stats.raw_max);
                self.alltime_min_score = self.alltime_min_score.min(stats.raw_min);
                self.on_performance = (self.on_performance * (self.cur_generation-1) as f32 + stats.raw_avg) / self.cur_generation as f32;
                self.off_max_performance = (self.off_max_performance * (self.cur_generation-1) as f32 + stats.raw_max) / self.cur_generation as f32;
                self.off_min_performance = (self.off_min_performance * (self.cur_generation-1) as f32 + stats.raw_min) / self.cur_generation as f32;

                self.alltime_best_pop = Some(pop);
                self.hist_stats.push(stats);
            }
        }
    }

    fn update_best(&mut self, pop: &GAPopulation<T>) where T: Clone + PartialEq
    {
        match self.alltime_best_pop
        {
            Some(ref mut best_pop) if best_pop.size() > 0 => 
            {
                let best_pop_size = best_pop.size();

                if pop.order() != best_pop.order()
                {
                    // This is what galib does.
                    // Why would the order change from one generation to another?
                    best_pop.set_order_and_sort(pop.order());
                }

                let order = best_pop.order();

                if best_pop_size == 1
                {
                    let mut best_pop_best_ind = best_pop.best_by_raw_score_mut();
                    let pop_best_ind = pop.best_by_raw_score();

                    if (order == GAPopulationSortOrder::LowIsBest
                        && pop_best_ind.raw() < best_pop_best_ind.raw())
                       ||
                       (order == GAPopulationSortOrder::HighIsBest
                        && pop_best_ind.raw() > best_pop_best_ind.raw())
                    {
                       (*best_pop_best_ind).clone_from(pop_best_ind); 
                    }
                }
		else
		{
                    let mut i = 0;
                    let pop_size = pop.size();

                    // This closure compares the raw scores of 2 individuals
                    // and determines whether the left-hand-side is a better
                    // score than the right-hand-side according to the 
                    // populations' order.
                    let cmp = |l_raw: f32, r_raw: f32| 
                    {
                        match order
                        {
                            GAPopulationSortOrder::HighIsBest => l_raw.partial_cmp(&r_raw).unwrap(),
                            GAPopulationSortOrder::LowIsBest => 
                            {
                                match l_raw.partial_cmp(&r_raw).unwrap()
                                {
                                    Equal => Equal,
                                    Greater => Less,
                                    Less => Greater
                                }
                            }
                        }
                    };

                    // Read Greater as Better.
                    while i < pop_size
                          && cmp(pop.kth_best_by_raw_score(i).raw(), 
                                 best_pop.worst_by_raw_score().raw()) == Greater
                    {
                        let mut k = 0;
                        let pop_ith_best = pop.kth_best_by_raw_score(i);
                        let pop_ith_best_raw = pop_ith_best.raw();

                        // Read Less as Worse.
                        while cmp(pop_ith_best_raw, best_pop.kth_best_by_raw_score(k).raw()) == Less
                              && k < best_pop_size
                        {
                            k = k+1;
                        }

                        for j in k..best_pop_size
                        {
                            let best_pop_jth_best_raw;

                            {
                                // Introduce a new scope. Otherwise, the later mutable
                                // borrow from worst_by_raw_score_mut() would not be
                                // allowed to co-exist with the immutable borrow from
                                // kth_best_by_raw_score().

                                let best_pop_jth_best = best_pop.kth_best_by_raw_score(j);
                                best_pop_jth_best_raw = best_pop_jth_best.raw();

                                if pop_ith_best == best_pop_jth_best
                                {
                                    break;
                                }
                            }

                            // Read Greater as Better.
                            if cmp(pop_ith_best_raw, best_pop_jth_best_raw) == Greater
                            {
                                (*best_pop.worst_by_raw_score_mut()).clone_from(pop_ith_best); 
                                best_pop.force_sort();

                                break;
                            }
                        }

                        i = i+1;
                    }
		}

                best_pop.reset_statistics();
                best_pop.statistics();
            },

            _ => 
            {
                // Usage error. Client should call set_best() first, with a non-empty population.
                // NOTE: This is what galib does in updateBestIndividual().
            }
        }
    }

    // Get the statistics of the nth generation (#1 is the first one).
    fn generation_statistics(&mut self, nth_generation: usize) -> Option<GAPopulationStats>
    {
        if nth_generation > 0 && nth_generation <= self.hist_stats.len()
        {
            Some(self.hist_stats[nth_generation-1].clone())
        }
        else
        {
            None
        }
    }

    // Get the statistics of the alltime-best individuals.
    fn alltime_best_statistics(&mut self) -> Option<GAPopulationStats>
    {
        match self.alltime_best_pop
        {
            Some(ref mut best_pop) => best_pop.statistics(),
            None => None
        }
    }
}

////////////////////////////////////////
// Tests
#[cfg(test)]
mod test
{
    use std::f32;

    use super::*;
    use ::ga::ga_test::*;
    use ::ga::ga_core::*;
    use ::ga::ga_population::*;
    use ::ga::ga_random::GARandomCtx;

    #[test]
    fn test_update_statistics()
    {
        ga_test_setup("ga_statistics::test_update_statistics");

        // Generation 1.

        let raw_scores_1: Vec<f32> = vec![-10.0, -8.0, -6.0, -4.0, -2.0, 0.1, 2.0, 4.0, 6.0, 8.0, 10.0];
        let expected_sum_1 = raw_scores_1.iter().fold(0.0, |sum, rs| sum + rs);
        let expected_avg_1 = expected_sum_1 / raw_scores_1.len() as f32;
        let expected_max_1 = raw_scores_1.iter().cloned().fold(f32::NEG_INFINITY, |max, rs| max.max(rs));
        let expected_min_1 = raw_scores_1.iter().cloned().fold(f32::INFINITY, |min, rs| min.min(rs));
        let expected_var_1 = raw_scores_1.iter().fold(0.0, |var, rs| var + (rs - expected_avg_1).powi(2)) / (raw_scores_1.len()-1) as f32;
        let expected_std_dev_1 = expected_var_1.sqrt();

        let mut inds_1: Vec<GATestIndividual> = Vec::new();
        for rs in raw_scores_1.iter().cloned()
        {
            inds_1.push(GATestIndividual::new(rs));
        }
        let mut pop_1 = GAPopulation::new(inds_1, GAPopulationSortOrder::LowIsBest);
        pop_1.sort();
        pop_1.statistics();

        // Generation 2.

        let raw_scores_2: Vec<f32> = vec![-9.0, -7.0, -5.0, -3.0, -1.0, 1.0, 3.0, 5.0, 7.0, 9.0, 11.0];
        let expected_sum_2 = raw_scores_2.iter().fold(0.0, |sum, rs| sum + rs);
        let expected_avg_2 = expected_sum_2 / raw_scores_2.len() as f32;
        let expected_max_2 = raw_scores_2.iter().cloned().fold(f32::NEG_INFINITY, |max, rs| max.max(rs));
        let expected_min_2 = raw_scores_2.iter().cloned().fold(f32::INFINITY, |min, rs| min.min(rs));
        let expected_var_2 = raw_scores_2.iter().fold(0.0, |var, rs| var + (rs - expected_avg_2).powi(2)) / (raw_scores_2.len()-1) as f32;
        let expected_std_dev_2 = expected_var_2.sqrt();

        let mut inds_2: Vec<GATestIndividual> = Vec::new();
        for rs in raw_scores_2.iter().cloned()
        {
            inds_2.push(GATestIndividual::new(rs));
        }
        let mut pop_2 = GAPopulation::new(inds_2, GAPopulationSortOrder::LowIsBest);
        pop_2.sort();
        pop_2.statistics();

        // Statistics after generation 1.

        let mut stats = GAStatistics::<GATestIndividual>::new();
        stats.set_best(pop_1.clone());

        let pop1_stats = pop_1.statistics().unwrap();
        let gen1_stats = stats.generation_statistics(1).unwrap();
        assert_eq!(stats.alltime_max_score == expected_max_1, true);
        assert_eq!(stats.alltime_min_score == expected_min_1, true);
        assert_eq!(gen1_stats == pop1_stats, true);

        // Statistics after generation 2.

        stats.update(&mut pop_2);

        let pop2_stats = pop_2.statistics().unwrap();
        let gen2_stats = stats.generation_statistics(2).unwrap();
        assert_eq!(stats.alltime_max_score == expected_max_2, true);
        assert_eq!(stats.alltime_min_score == expected_min_1, true);
        assert_eq!(gen2_stats == pop2_stats, true);

        ga_test_teardown();
    }

    #[test]
    fn test_update_best_population()
    {
        let test_name = "ga_statistics::test_update_best_population";
        ga_test_setup(test_name);

        let mut fact = GATestFactory::new(0.0);
        let rng_ctx = &mut GARandomCtx::new_unseeded(test_name.to_string());

        {
            /* Create 3 populations, each one better than the previous one. */
            /* Each one should replace the previous as the all-time best. */
            /* 1-individual populations, HighIsBest ranking. */

            let mut pop = fact.random_population(1, GAPopulationSortOrder::HighIsBest, rng_ctx);
            pop.sort();
            pop.statistics();

            let mut better_pop = fact.better_random_population_than(&pop);
            better_pop.sort();
            better_pop.statistics();

            let mut even_better_pop = fact.better_random_population_than(&better_pop);
            even_better_pop.sort();
            even_better_pop.statistics();

            let mut stats = GAStatistics::<GATestIndividual>::new();
            stats.set_best(pop.clone());
            stats.update_best(&better_pop);

            let mut best_pop = stats.best().unwrap();
            assert_eq!(best_pop == better_pop, true);

            stats.update_best(&even_better_pop);
            best_pop = stats.best().unwrap();
            assert_eq!(best_pop == even_better_pop, true);
        }

        {
            /* Create 3 populations, each one better than the previous one. */
            /* Each one should replace the previous as the all-time best. */
            /* 1-individual populations, LowIsBest ranking. */

            let mut pop = fact.random_population(1, GAPopulationSortOrder::LowIsBest, rng_ctx);
            pop.sort();
            pop.statistics();

            let mut better_pop = fact.better_random_population_than(&pop);
            better_pop.sort();
            better_pop.statistics();

            let mut even_better_pop = fact.better_random_population_than(&better_pop);
            even_better_pop.sort();
            even_better_pop.statistics();

            let mut stats = GAStatistics::<GATestIndividual>::new();
            stats.set_best(pop.clone());
            stats.update_best(&better_pop);

            let mut best_pop = stats.best().unwrap();
            assert_eq!(best_pop == better_pop, true);

            stats.update_best(&even_better_pop);
            best_pop = stats.best().unwrap();
            assert_eq!(best_pop == even_better_pop, true);
        }

        {
            /* Create 3 populations, each one better than the previous one. */
            /* Each one should replace the previous as the all-time best. */
            /* N-individual populations, N > 1, HighIsBest ranking. */

            let mut pop = fact.random_population(5, GAPopulationSortOrder::HighIsBest, rng_ctx);
            pop.sort();
            pop.statistics();

            let mut better_pop = fact.better_random_population_than(&pop);
            better_pop.sort();
            better_pop.statistics();

            let mut even_better_pop = fact.better_random_population_than(&better_pop);
            even_better_pop.sort();
            even_better_pop.statistics();

            let mut stats = GAStatistics::<GATestIndividual>::new();
            stats.set_best(pop.clone());
            stats.update_best(&better_pop);

            let mut best_pop = stats.best().unwrap();
            assert_eq!(best_pop == better_pop, true);

            stats.update_best(&even_better_pop);
            best_pop = stats.best().unwrap();
            assert_eq!(best_pop == even_better_pop, true);
        }

        {
            /* Create 3 populations, each one better than the previous one. */
            /* Each one should replace the previous as the all-time best. */
            /* N-individual populations, N > 1, LowIsBest ranking. */

            let mut pop = fact.random_population(5, GAPopulationSortOrder::LowIsBest, rng_ctx);
            pop.sort();
            pop.statistics();

            let mut better_pop = fact.better_random_population_than(&pop);
            better_pop.sort();
            better_pop.statistics();

            let mut even_better_pop = fact.better_random_population_than(&better_pop);
            even_better_pop.sort();
            even_better_pop.statistics();

            let mut stats = GAStatistics::<GATestIndividual>::new();
            stats.set_best(pop.clone());
            stats.update_best(&better_pop);

            let mut best_pop = stats.best().unwrap();
            assert_eq!(best_pop == better_pop, true);

            stats.update_best(&even_better_pop);
            best_pop = stats.best().unwrap();
            assert_eq!(best_pop == even_better_pop, true);
        }

        {
            // Scores were chosen so that the best population contain individuals
            // from 2 different populations.
            // HighIsBest ranking.

            // Even scores.
            let raw_scores_1: Vec<f32> = vec![-10.0, -8.0, -6.0, -4.0, -2.0, 0.1, 2.0, 4.0, 6.0, 8.0, 10.0];
            // Odd scores.
            let raw_scores_2: Vec<f32> = vec![-9.0, -7.0, -5.0, -3.0, -1.0, 1.0, 3.0, 5.0, 7.0, 9.0, 11.0];

            let mut inds_1: Vec<GATestIndividual> = Vec::new();
            for rs in raw_scores_1.iter().cloned()
            {
                inds_1.push(GATestIndividual::new(rs));
            }
            let mut pop_1 = GAPopulation::new(inds_1, GAPopulationSortOrder::HighIsBest);
            pop_1.sort();
            pop_1.statistics();

            let mut inds_2: Vec<GATestIndividual> = Vec::new();
            for rs in raw_scores_2.iter().cloned()
            {
                inds_2.push(GATestIndividual::new(rs));
            }
            let mut pop_2 = GAPopulation::new(inds_2, GAPopulationSortOrder::HighIsBest);
            pop_2.sort();
            pop_1.statistics();

            let mut stats = GAStatistics::<GATestIndividual>::new();
            stats.set_best(pop_1.clone());
            stats.update_best(&pop_2);

            let mut best_pop = stats.best().unwrap();

            let best_raw_scores: Vec<f32> = vec![11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];

            let mut best_inds: Vec<GATestIndividual> = Vec::new();
            for rs in best_raw_scores.iter().cloned()
            {
                best_inds.push(GATestIndividual::new(rs));
            }
            let mut expected_best_pop = GAPopulation::new(best_inds, GAPopulationSortOrder::HighIsBest);
            expected_best_pop.sort();
            expected_best_pop.statistics();

            assert_eq!(best_pop == expected_best_pop, true);
        }

        {
            // Scores were chosen so that the best population contain individuals
            // from 2 different populations.
            // LowIsBest ranking.

            // Even scores.
            let raw_scores_1: Vec<f32> = vec![-10.0, -8.0, -6.0, -4.0, -2.0, 0.1, 2.0, 4.0, 6.0, 8.0, 10.0];
            // Raw scores.
            let raw_scores_2: Vec<f32> = vec![-9.0, -7.0, -5.0, -3.0, -1.0, 1.0, 3.0, 5.0, 7.0, 9.0, 11.0];

            let mut inds_1: Vec<GATestIndividual> = Vec::new();
            for rs in raw_scores_1.iter().cloned()
            {
                inds_1.push(GATestIndividual::new(rs));
            }
            let mut pop_1 = GAPopulation::new(inds_1, GAPopulationSortOrder::LowIsBest);
            pop_1.sort();
            pop_1.statistics();

            let mut inds_2: Vec<GATestIndividual> = Vec::new();
            for rs in raw_scores_2.iter().cloned()
            {
                inds_2.push(GATestIndividual::new(rs));
            }
            let mut pop_2 = GAPopulation::new(inds_2, GAPopulationSortOrder::LowIsBest);
            pop_2.sort();
            pop_1.statistics();

            let mut stats = GAStatistics::<GATestIndividual>::new();
            stats.set_best(pop_1.clone());
            stats.update_best(&pop_2);

            let mut best_pop = stats.best().unwrap();

            let best_raw_scores: Vec<f32> = vec![-10.0, -9.0, -8.0, -7.0, -6.0, -5.0, -4.0, -3.0, -2.0, -1.0, 0.1];

            let mut best_inds: Vec<GATestIndividual> = Vec::new();
            for rs in best_raw_scores.iter().cloned()
            {
                best_inds.push(GATestIndividual::new(rs));
            }
            let mut expected_best_pop = GAPopulation::new(best_inds, GAPopulationSortOrder::LowIsBest);
            expected_best_pop.sort();
            expected_best_pop.statistics();

            assert_eq!(best_pop == expected_best_pop, true);
        }

        ga_test_teardown();
    }
}
