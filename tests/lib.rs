extern crate rust_monster;
#[cfg(test)]
mod tests
{
    use rust_monster::ga::GeneticAlgorithm;
    use rust_monster::ga;
    
    #[allow(dead_code)]
    struct TestSolution
    {
        fitness: f32
    }
    impl ga::GASolution for TestSolution 
    {
        fn evaluate(&mut self) -> f32 { 0.0 }
    #[allow(unused_variables)]
        fn crossover(&self, other : &Self) -> &Self { &self }
        fn mutate(&mut self) {}
        fn fitness(&self) -> f32 { self.fitness }
    }
    
    #[test]
    fn init_test_with_initial_population()
    {
        let initial_population = vec![TestSolution { fitness: 0.0 }];

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
        ga.initialize();
        assert_eq!(ga.step(), 1);
        assert_eq!(ga.done(), false);
    }

    #[test]
    #[should_panic]
    fn init_test_missing_args()
    {
        let mut ga : ga::SimpleGeneticAlgorithm<TestSolution> =
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
        ga.initialize();
    }
}
