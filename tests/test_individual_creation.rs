
#[cfg(test)]
mod individual_creation_tests {
    use darwin::individual::Individual;
    #[test]
    fn test_individual_creation_success() {
        let genes: Vec<f64> = vec![1.0, 2.0, 3.0];
        let individual = Individual::new(genes);
        assert_eq!(individual.genes, vec![1.0, 2.0, 3.0]);
        assert_eq!(individual.fitness, Individual::default_fitness());
    }

    #[test]
    #[should_panic(expected = "Genes cannot be empty")]
    fn test_individual_creation_fail_empty_genes() {
        let genes = vec![];
        let _individual = Individual::new(genes);
    }

    #[test]
    fn test_individual_random_population_success() {
        let gene_pool = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let population = Individual::random_population(10, 3, gene_pool);
        assert_eq!(population.len(), 10);
        for individual in population {
            assert_eq!(individual.genes.len(), 3);
            assert!(individual.genes[0] >= 1.0);
            assert!(individual.genes[0] <= 5.0);
            assert!(individual.genes[1] >= 1.0);
            assert!(individual.genes[1] <= 5.0);
            assert!(individual.genes[2] >= 1.0);
            assert!(individual.genes[2] <= 5.0);
        }
    }
}
