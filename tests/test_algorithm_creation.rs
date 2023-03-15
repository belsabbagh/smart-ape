
#[cfg(test)]
mod genetic_algorithm_creation_tests {
    use darwin::individual::crossover::single_point_crossover;
    use darwin::genetic_algorithm::GeneticAlgorithm;
    use darwin::individual::Individual;
    use darwin::individual::mutation::inversion_mutation;
    use darwin::individual::selection::tournament_selection;
    struct DefaultFns {
        pub crossover: fn(
            &Individual,
            &Individual,
        ) -> Vec<Individual>,
        pub mutation: fn(&Individual) -> Individual,
        pub selection:
            fn(&Vec<Individual>) -> Vec<Individual>,
        pub fitness: fn(&Individual) -> f64,
    }

    fn setup() -> DefaultFns {
        let fitness = |individual: &Individual| -> f64 { individual.genes[0] };
        let crossover = single_point_crossover;
        let mutation = inversion_mutation;
        let selection = tournament_selection;
        return DefaultFns {
            fitness,
            crossover,
            mutation,
            selection,
        };
    }

    #[test]
    fn test_ga_creation_success() {
        let fns = setup();
        let ga = GeneticAlgorithm::new(
            fns.fitness,
            fns.crossover,
            fns.mutation,
            fns.selection,
            10,
            0.1,
            true,
            2,
        );
        assert_eq!(ga.population_size, 10);
        assert_eq!(ga.mutation_rate, 0.1);
        assert_eq!(ga.elitism, true);
        assert_eq!(ga.elitism_count, 2);
    }

    #[test]
    #[should_panic(expected = "Elitism count cannot be greater than population size")]
    fn test_ga_creation_fail_elitism_count() {
        let fns = setup();
        let _ga = GeneticAlgorithm::new(
            fns.fitness,
            fns.crossover,
            fns.mutation,
            fns.selection,
            10,
            0.1,
            true,
            11,
        );
    }

    #[test]
    #[should_panic(expected = "Mutation rate must be between 0.0 and 1.0")]
    fn test_ga_creation_fail_mutation_rate() {
        let fns = setup();
        let _ga = GeneticAlgorithm::new(
            fns.fitness,
            fns.crossover,
            fns.mutation,
            fns.selection,
            10,
            1.1,
            true,
            2,
        );
    }

    #[test]
    #[should_panic(expected = "Population size must be greater than 1")]
    fn test_ga_creation_fail_population_size() {
        let fns = setup();
        let _ga = GeneticAlgorithm::new(
            fns.fitness,
            fns.crossover,
            fns.mutation,
            fns.selection,
            1,
            0.1,
            false,
            0,
        );
    }

    #[test]
    fn test_ga_evaluate() {
        let fns = setup();
        let mut ga = GeneticAlgorithm::new(
            fns.fitness,
            fns.crossover,
            fns.mutation,
            fns.selection,
            10,
            0.1,
            true,
            2,
        );
        let mut population = vec![
            Individual::new(vec![1.0]),
            Individual::new(vec![2.0]),
            Individual::new(vec![3.0]),
        ];
        ga.evaluate(&mut population);
        assert_eq!(population[2].fitness, 3.0);
        assert_eq!(population[1].fitness, 2.0);
        assert_eq!(population[0].fitness, 1.0);
    }
}
