

struct SimpleGeneticAlgorithmCfg
{
  d_seed : i32,
  pconv  : f32,
  pcross : f32,
  pmut   : f32,
  minmax : i32
}

trait GeneticAlgorithm
{
    fn initialize(&self);
    fn step(&self);
    fn done(&self);
}

struct SimpleGeneticAlgorithm
{
  cfg : SimpleGeneticAlgorithmCfg
}

impl GeneticAlgorithm for SimpleGeneticAlgorithm 
{
    fn initialize(&self) {}
    fn step(&self) {}
    fn done(&self) {}
}


