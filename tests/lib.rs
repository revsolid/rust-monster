// TODO: COPYRIGHT, USE & AUTHORS
// TODO: RUST DOCS!

extern crate rust_monster;
#[macro_use]
extern crate log;
#[cfg(test)]
mod tests
{

    extern crate env_logger;
    // Test with the classic TSP problem
    use rust_monster::ga::ga_test::*;
    use rust_monster::ga::ga_random::*;
    use rust_monster::ga::ga_simple::*;
    use rust_monster::ga::ga_population::*;
    use rust_monster::ga::ga_core::*;

    use std::cmp::min;
    use std::any::Any;
    use std::f64;

    struct TSPEvaluationCtx
    {
        pub cities: Vec<(f64, f64)>,
    }
    impl TSPEvaluationCtx
    {
        pub fn new(num_cities: usize) -> TSPEvaluationCtx
        {
            let mut rng = GARandomCtx::from_seed([1,2,3,4], "TSP Cities Placement RNG".to_string());
            let mut cities = vec![];
            for _ in 0..num_cities
            {
                // 360 degrees, in radians
                let th: f64 = rng.gen_range(0.0, f64::consts::PI*2.0);
                let x = th.cos(); 
                let y = th.sin(); 
                cities.push((x,y))
            }
            TSPEvaluationCtx { cities: cities }
        }
    }

    #[derive(Clone)]
    struct TSPIndividual
    {
        raw: f32,
        fitness: f32,
        inxes: Vec<usize>
    }
    impl TSPIndividual
    {
        pub fn new(rnd_ctx: &mut GARandomCtx,
                   tsp_size: usize) -> TSPIndividual
        {
            let mut inxes = vec![];
            for i in 0..tsp_size
            {
                inxes.push(i);
            }

            rnd_ctx.shuffle(&mut inxes[..]);

            TSPIndividual{raw: 0.0, fitness: 0.0, inxes: inxes}
        }

        pub fn new_from_inxes(inxes: Vec<usize>) -> TSPIndividual
        {
            TSPIndividual{raw: 0.0, fitness: 0.0, inxes: inxes}
        }
    }
    impl GAIndividual for TSPIndividual
    {
        // Crossing over a permutation isn't as simple as one might think
        // algorithm inspired in: http://www.permutationcity.co.uk/projects/mutants/tsp.html
        fn crossover(&self, other: &TSPIndividual, ctx: &mut Any) -> Box<TSPIndividual>
        {
            match ctx.downcast_mut::<GARandomCtx>()
            {
                Some(rng_ctx) =>
                {
                    let to_pick = min(self.inxes.len(), 3);
                    let mut new_inxes = vec![];
        
                    // Copy the first parent
                    for i in 0..self.inxes.len()
                    {
                        new_inxes.push(self.inxes[i]);
                    }
        
                    let mut picked = vec![]; //This are indexes
                    for _ in 0..to_pick
                    {
                        picked.push(rng_ctx.gen_range(0, self.inxes.len()));
                    }
        
                    let mut places: [usize; 3] = [0, 0, 0];
                    for pi in 0..picked.len()
                    {
                        for oi in 0..other.inxes.len()
                        {
                            if self.inxes[picked[pi]] == other.inxes[oi]
                            {
                                places[pi] = oi;
                                break;
                            }
                        }
                    }
        
                    for pi in 0..picked.len()
                    {
                        let temp = new_inxes[picked[pi]];
                        new_inxes[picked[pi]] = new_inxes[places[pi]];
                        new_inxes[places[pi]] = temp;
                    }
        
                    Box::new(TSPIndividual::new_from_inxes(new_inxes))
                },
                None =>
                {
                    panic!("Incorrect type passed for context");
                }
            }
        }

        fn mutate(&mut self, probability: f32, ctx: &mut Any)
        {
            match ctx.downcast_mut::<GARandomCtx>()
            {
                Some(rng) =>
                {
                    if rng.test_value(probability)
                    {
                        let p1 = rng.gen_range(0, self.inxes.len());
                        let mut p2 = p1;
        
                        while p1 == p2
                        {
                            p2 = rng.gen_range(0, self.inxes.len());
                        }
        
                        let tmp = self.inxes[p1];
                        self.inxes[p1] = self.inxes[p2];
                        self.inxes[p2] = tmp;
                    }
                },
                None =>
                {
                    panic!("Incorrect context");
                }
            }
        }

        fn evaluate(&mut self, evaluation_ctx: &mut Any)
        {

            match evaluation_ctx.downcast_mut::<TSPEvaluationCtx>()
            {
               Some(as_tsp_eval_ctx) =>
               {
                   let mut cost = 0.0;
                   for i in 0..self.inxes.len()-1
                   {
                      let j = i + 1;
                      let c1 = as_tsp_eval_ctx.cities[self.inxes[i]];
                      let c2 = as_tsp_eval_ctx.cities[self.inxes[j]];

                      cost += ((c1.0 - c2.0) * (c1.0 - c2.0) + (c1.1 - c2.1) * (c1.1 - c2.1)).sqrt();
                   }
                   let c1 = as_tsp_eval_ctx.cities[self.inxes[0]];
                   let c2 = as_tsp_eval_ctx.cities[self.inxes[self.inxes.len() - 1]];
                   cost += ((c1.0 - c2.0) * (c1.0 - c2.0) + (c1.1 - c2.1) * (c1.1 - c2.1)).sqrt();
                   self.set_raw((cost as f32));
                   self.set_fitness((cost as f32));
               }
               None =>
               {
                   // TODO: Better error messaging, print types
                   panic!("Incorrect type of context passed"); 
               }
            }
        }

        fn fitness(&self) -> f32 { self.fitness }
        fn set_fitness(&mut self, fitness: f32) { self.fitness = fitness; }
        fn raw(&self) -> f32 { self.raw }
        fn set_raw(&mut self, raw: f32) { self.raw = raw; }
    }

    struct TSPIndividualFactory
    {
        tsp_size: usize,
    }
    impl TSPIndividualFactory
    {
        fn new(tsp_size: usize) -> TSPIndividualFactory
        {
            TSPIndividualFactory { tsp_size: tsp_size}
        }
    }
    impl GAFactory<TSPIndividual> for TSPIndividualFactory
    {
        fn random_population(&mut self, n: usize,
                             sort_order: GAPopulationSortOrder, rng_ctx: &mut GARandomCtx) -> GAPopulation<TSPIndividual>
        {
            let mut new_individuals = vec![];

            for _ in 0..n
            {
                new_individuals.push(TSPIndividual::new(rng_ctx, self.tsp_size));
            }

            GAPopulation::new(new_individuals, sort_order)
        }
    }


    #[test]
    fn tsp_integration_test()
    {
        let _ = env_logger::init();
        let tsp_size = 30;
        let mut ind_factory = TSPIndividualFactory::new(tsp_size);
        let mut evaluation_ctx = TSPEvaluationCtx::new(tsp_size);


        debug!("{:?}", evaluation_ctx.cities);
        let mut sga = SimpleGeneticAlgorithm::new_with_eval_ctx(SimpleGeneticAlgorithmCfg {
                                                                d_seed : [1,0,1,0],
                                                                flags : DEBUG_FLAG,
                                                                max_generations: 200,
                                                                population_size: 100,
                                                                probability_crossover: 0.9,
                                                                probability_mutation: 0.15,
                                                                population_sort_order: GAPopulationSortOrder::LowIsBest,
                                                                elitism: true,
                                                                ..Default::default()
                                                              },
                                                              Some(&mut ind_factory),
                                                              None,
                                                              Some(&mut evaluation_ctx as &mut Any),
                                                 );
        sga.initialize();

        while !sga.done()
        {
            let gen = sga.step();
            debug!("Generation #{} {:?} {:?}", gen,
                   sga.population().individual(0, GAPopulationSortBasis::Raw).raw(),
                   sga.population().individual(0, GAPopulationSortBasis::Raw).inxes);
        }
    }
}
