// TODO: COPYRIGHT, USE & AUTHORS
// TODO: RUST DOCS!

extern crate rust_monster;
#[cfg(test)]
mod tests
{
    use rust_monster::ga::{GeneticAlgorithm, GASolution};
    use rust_monster::ga;
    const VAL : f32 = 3.14159;
    
    struct TestSolution
    {
        fitness: f32
    }
    impl ga::GASolution for TestSolution 
    {
        fn evaluate(&mut self) -> f32 { self.fitness }
    #[allow(unused_variables)]
        fn crossover(&self, other : &Self) -> &Self { &self }
        fn mutate(&mut self) {}
        fn fitness(&self) -> f32 { self.fitness }
    }

    #[allow(dead_code)]
    struct TestFactory
    {
        starting_fitness: f32
    }
    impl ga::GAFactory<TestSolution> for TestFactory
    {
        fn initial_population(&mut self) -> Vec<TestSolution> {
            vec![TestSolution { fitness: self.starting_fitness }]
        }
    }

    fn simple_ga_validation(sga:&mut ga::SimpleGeneticAlgorithm<TestSolution>)
    {
        sga.initialize();
        assert_eq!(sga.step(), 1);
        assert_eq!(sga.done(), false);
        assert_eq!(sga.population().len(), 1);
        assert_eq!(sga.population()[0].fitness(), VAL);
    }
    
    #[test]
    fn init_test_with_initial_population()
    {
        let initial_population = vec![TestSolution { fitness: VAL}];

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
        let empty_initial_population : Vec<TestSolution> = vec![];
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
