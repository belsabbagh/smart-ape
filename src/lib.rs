pub mod individual;

pub mod genetic_algorithm {
    use crate::individual::Individual;

    pub struct GeneticAlgorithm {
        pub fitness: fn(&Individual) -> f64,
        pub crossover: fn(&Individual, &Individual) -> Vec<Individual>,
        pub mutation: fn(&Individual) -> Individual,
        pub selection: fn(&Vec<Individual>) -> Vec<Individual>,
        pub population_size: usize,
        pub mutation_rate: f64,
        pub elitism: bool,
        pub elitism_count: usize,
    }

    impl GeneticAlgorithm {
        pub fn new(
            fitness: fn(&Individual) -> f64,
            crossover: fn(&Individual, &Individual) -> Vec<Individual>,
            mutation: fn(&Individual) -> Individual,
            selection: fn(&Vec<Individual>) -> Vec<Individual>,
            population_size: usize,
            mutation_rate: f64,
            elitism: bool,
            elitism_count: usize,
        ) -> GeneticAlgorithm {
            if elitism && elitism_count > population_size {
                panic!("Elitism count cannot be greater than population size");
            }
            if mutation_rate < 0.0 || mutation_rate > 1.0 {
                panic!("Mutation rate must be between 0.0 and 1.0");
            }
            if population_size < 2 {
                panic!("Population size must be greater than 1");
            }
            GeneticAlgorithm {
                fitness,
                crossover,
                mutation,
                selection,
                population_size,
                mutation_rate,
                elitism,
                elitism_count,
            }
        }

        pub fn evaluate(&mut self, population: &mut Vec<Individual>) {
            for individual in population {
                individual.fitness = (self.fitness)(individual);
            }
        }

        pub fn evolve(&mut self, population: &mut Vec<Individual>) {
            self.evaluate(population);
            population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
            let mut new_population = Vec::new();
            if self.elitism {
                for i in 0..self.elitism_count {
                    new_population.push(population[i].clone());
                }
            }
            while new_population.len() < self.population_size {
                let parents = (self.selection)(&population);
                let children = (self.crossover)(&parents[0], &parents[1]);
                for child in children {
                    if rand::random::<f64>() < self.mutation_rate {
                        new_population.push((self.mutation)(&child));
                        continue;
                    }
                    new_population.push(child);
                }
            }
            population.clear();
            population.append(&mut new_population);
        }

        pub fn run(&mut self, population: &mut Vec<Individual>, generations: usize, verbose: bool) {
            for i in 1..generations + 1 {
                self.evolve(population);
                let best = population[0].clone();
                if verbose {
                    Self::run_msg(i, &best);
                }
                if best.fitness == 0.0 {
                    break;
                }
            }
        }

        fn run_msg(gen: usize, best: &Individual) {
            println!(
                "Gen: {}\tBest fitness: {}\tBest genes: {:?}",
                gen, best.fitness, best.genes
            );
        }
    }
}
