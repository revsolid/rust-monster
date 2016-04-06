// TODO: COPYRIGHT, USE & AUTHORS
// TODO: RUST DOCS!

extern crate rust_monster;
#[macro_use]
extern crate log;
extern crate env_logger;
#[cfg(test)]
mod tests
{
    use rust_monster::ga::{GeneticAlgorithm, GASolution, GAPopulation, GAPopulationSortOrder};
    use rust_monster::ga;

    use env_logger;
    use std::sync::{Once, ONCE_INIT};

    static INIT: Once = ONCE_INIT;
    const VAL: f32 = 3.14159;
    
    struct TestSolution
    {
        fitness: f32
    }
    impl ga::GASolution for TestSolution 
    {
        fn new() -> TestSolution
        {
            TestSolution{ fitness: VAL }
        }

        fn clone(&self) -> Self { TestSolution::new() }
        fn evaluate(&mut self) -> f32 { self.fitness }
    #[allow(unused_variables)]
        fn crossover(&self, other : &Self) -> Self { TestSolution::new() }
    #[allow(unused_variables)]
        fn mutate(&mut self, pm : f32) {}
        fn fitness(&self) -> f32 { self.fitness }
        fn score(&self) -> f32 { self.fitness }
    }

    #[allow(dead_code)]
    struct TestFactory
    {
        starting_fitness: f32
    }
    impl ga::GAFactory<TestSolution> for TestFactory
    {
        fn initial_population(&mut self) -> GAPopulation<TestSolution> {
            GAPopulation::new(vec![TestSolution { fitness: self.starting_fitness }], GAPopulationSortOrder::HighIsBest)
        }
    }

////////////////////////////////////////
// Utility functions

    fn ga_test_setup()
    {
        INIT.call_once(||{
            env_logger::init().unwrap();
        });
    }


    fn simple_ga_validation(sga:&mut ga::SimpleGeneticAlgorithm<TestSolution>)
    {
        sga.initialize();
        assert_eq!(sga.step(), 1);
        assert_eq!(sga.done(), false);
        assert_eq!(sga.population().size(), 1);
        assert_eq!(sga.population().best().fitness(), VAL);
    }

////////////////////////////////////////
// Tests
    
    #[test]
    fn init_test_with_initial_population()
    {
        ga_test_setup();
        let initial_population = GAPopulation::new(vec![TestSolution { fitness: VAL}], GAPopulationSortOrder::HighIsBest);
        let mut ga : ga::SimpleGeneticAlgorithm<TestSolution> =
                     ga::SimpleGeneticAlgorithm::new(ga::SimpleGeneticAlgorithmCfg {
                                                       d_seed : 1,
                                                       flags : ga::DEBUG_FLAG,
                                                       max_generations: 100,
                                                       ..Default::default()
                                                     },
                                                     None,
                                                     Some(initial_population) 
                                                     );
        simple_ga_validation(&mut ga);
    }

    #[test]
    fn init_test_with_factory()
    {
        ga_test_setup();
        let mut factory = TestFactory { starting_fitness: VAL };
        let mut ga : ga::SimpleGeneticAlgorithm<TestSolution> =
                     ga::SimpleGeneticAlgorithm::new(ga::SimpleGeneticAlgorithmCfg {
                                                       d_seed : 1,
                                                       flags : ga::DEBUG_FLAG,
                                                       max_generations: 100,
                                                       ..Default::default()
                                                     },
                                                     Some(&mut factory as &mut ga::GAFactory<TestSolution>),
                                                     None
                                                     );
        simple_ga_validation(&mut ga);
    }

    #[test]
    #[should_panic]
    #[allow(unused_variables)]
    fn init_test_missing_args()
    {
        ga_test_setup();
        let ga : ga::SimpleGeneticAlgorithm<TestSolution> =
                 ga::SimpleGeneticAlgorithm::new(ga::SimpleGeneticAlgorithmCfg {
                                                   d_seed : 1,
                                                   flags : ga::DEBUG_FLAG,
                                                   max_generations: 100,
                                                   ..Default::default()
                                                 },
                                                 None,
                                                 None 
                                                 );
        // Not reached
    }

    #[test]
    #[should_panic]
    fn init_test_empty_initial_pop()
    {
        ga_test_setup();
        let empty_initial_population : GAPopulation<TestSolution> = GAPopulation::new(vec![], GAPopulationSortOrder::HighIsBest);
        let mut ga : ga::SimpleGeneticAlgorithm<TestSolution> =
                     ga::SimpleGeneticAlgorithm::new(ga::SimpleGeneticAlgorithmCfg {
                                                       d_seed : 1,
                                                       flags : ga::DEBUG_FLAG,
                                                       max_generations: 100,
                                                       ..Default::default()
                                                     },
                                                     None,
                                                     Some(empty_initial_population) 
                                                     );
        ga.initialize()
        //Not reached 
    }
}
