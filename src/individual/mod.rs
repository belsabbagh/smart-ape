use rand::seq::SliceRandom;

pub struct Individual {
    pub genes: Vec<f64>,
    pub fitness: f64,
}

impl Individual {
    pub fn new(genes: Vec<f64>) -> Individual {
        if genes.is_empty() {
            panic!("Genes cannot be empty");
        }
        Individual {
            genes,
            fitness: Self::default_fitness(),
        }
    }
    pub fn default_fitness() -> f64 {
        5000000.0
    }
    pub fn random_population(
        population_size: usize,
        gene_size: usize,
        gene_pool: Vec<f64>,
    ) -> Vec<Individual> {
        let mut population = Vec::new();
        for _ in 0..population_size {
            let mut genes = Vec::<f64>::new();
            for _ in 0..gene_size {
                genes.push(*gene_pool.choose(&mut rand::thread_rng()).unwrap());
            }
            population.push(Individual::new(genes));
        }
        population
    }
}
impl Clone for Individual {
    fn clone(&self) -> Self {
        Individual {
            genes: self.genes.clone(),
            fitness: self.fitness,
        }
    }
}

pub mod crossover {
    use super::Individual;

    pub fn single_point_crossover(parent1: &Individual, parent2: &Individual) -> Vec<Individual> {
        let mut child1 = parent1.clone();
        let mut child2 = parent2.clone();
        let point = rand::random::<usize>() % parent1.genes.len();
        for i in point..parent1.genes.len() {
            child1.genes[i] = parent2.genes[i];
            child2.genes[i] = parent1.genes[i];
        }
        vec![child1, child2]
    }

    pub fn two_point_crossover(parent1: &Individual, parent2: &Individual) -> Vec<Individual> {
        let mut child1 = parent1.clone();
        let mut child2 = parent2.clone();
        let point1 = rand::random::<usize>() % parent1.genes.len();
        let point2 = rand::random::<usize>() % parent1.genes.len();
        let start = point1.min(point2);
        let end = point1.max(point2);
        for i in start..end {
            child1.genes[i] = parent2.genes[i];
            child2.genes[i] = parent1.genes[i];
        }
        vec![child1, child2]
    }

    pub fn uniform_crossover(parent1: &Individual, parent2: &Individual) -> Vec<Individual> {
        let mut child1 = parent1.clone();
        let mut child2 = parent2.clone();
        for i in 0..parent1.genes.len() {
            if rand::random::<bool>() {
                child1.genes[i] = parent2.genes[i];
                child2.genes[i] = parent1.genes[i];
            }
        }
        vec![child1, child2]
    }

    pub fn arithmetic_crossover(parent1: &Individual, parent2: &Individual) -> Vec<Individual> {
        let mut child1 = parent1.clone();
        let mut child2 = parent2.clone();
        let alpha = rand::random::<f64>();
        for i in 0..parent1.genes.len() {
            child1.genes[i] = alpha * parent1.genes[i] + (1.0 - alpha) * parent2.genes[i];
            child2.genes[i] = alpha * parent2.genes[i] + (1.0 - alpha) * parent1.genes[i];
        }
        vec![child1, child2]
    }
}

pub mod mutation {
    use super::Individual;
    use rand::seq::SliceRandom;

    pub fn swap_mutation(individual: &Individual) -> Individual {
        let mut child = individual.clone();
        let point1 = rand::random::<usize>() % individual.genes.len();
        let point2 = rand::random::<usize>() % individual.genes.len();
        child.genes.swap(point1, point2);
        child
    }

    pub fn scramble_mutation(individual: &Individual) -> Individual {
        let mut child = individual.clone();
        let point1 = rand::random::<usize>() % individual.genes.len();
        let point2 = rand::random::<usize>() % individual.genes.len();
        let start = point1.min(point2);
        let end = point1.max(point2);
        child.genes[start..end].shuffle(&mut rand::thread_rng());
        child
    }

    pub fn inversion_mutation(individual: &Individual) -> Individual {
        let mut child = individual.clone();
        let point1 = rand::random::<usize>() % individual.genes.len();
        let point2 = rand::random::<usize>() % individual.genes.len();
        let start = point1.min(point2);
        let end = point1.max(point2);
        child.genes[start..end].reverse();
        child
    }
}

pub mod selection {
    use super::Individual;

    pub fn roulette_wheel_selection(population: &Vec<Individual>) -> Vec<Individual> {
        let mut parents = Vec::new();
        let mut fitness_sum = 0.0;
        for individual in population {
            fitness_sum += individual.fitness;
        }
        for _ in 0..2 {
            let mut fitness = rand::random::<f64>() * fitness_sum;
            for individual in population {
                fitness -= individual.fitness;
                if fitness <= 0.0 {
                    parents.push(individual.clone());
                    break;
                }
            }
        }
        parents
    }

    pub fn tournament_selection(population: &Vec<Individual>) -> Vec<Individual> {
        let mut parents = Vec::new();
        for _ in 0..2 {
            let mut best = population[0].clone();
            for _ in 0..2 {
                let individual = population[rand::random::<usize>() % population.len()].clone();
                if individual.fitness < best.fitness {
                    best = individual;
                }
            }
            parents.push(best);
        }
        parents
    }
}
