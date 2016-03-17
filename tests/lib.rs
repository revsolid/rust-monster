extern crate rust_monster;
#[cfg(test)]
mod tests
{
    use rust_monster::ga::GeneticAlgorithm;
    use rust_monster::ga;
    
    #[allow(dead_code)]
    struct Foo { a : i32}
    impl ga::GASolution for Foo
    {
        fn evaluate(&mut self) -> f32 { 0.0 }
    #[allow(unused_variables)]
        fn crossover(&self, other : &Self) -> &Self { return &self }
        fn mutate(&mut self) {}
    }
    
    #[test]
    fn init_test()
    {
        let mut ga : ga::SimpleGeneticAlgorithm<Foo> =
                     ga::SimpleGeneticAlgorithm::new(ga::SimpleGeneticAlgorithmCfg {
                                                       d_seed : 1,
                                                       flags : ga::DEBUG_FLAG,
                                                       max_generations: 100,
                                                       ..Default::default()
                                                     }
        );
        ga.initialize();
        assert_eq!(ga.step(), 1);
        assert_eq!(ga.done(), false);
    }
}
