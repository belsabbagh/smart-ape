#[cfg(test)]
mod individual_functions_tests{
    use darwin::individual::crossover::single_point_crossover;
    use darwin::individual::Individual;
    #[test]
    fn test_single_point_crossover() {
        let parent1 = Individual::new(vec![1.0]);
        let parent2 = Individual::new(vec![3.0]);
        let children = single_point_crossover(&parent1, &parent2);
        assert_eq!(children[0].genes, vec![3.0], "Child 1 genes are incorrect");
        assert_eq!(children[1].genes, vec![1.0], "Child 2 genes are incorrect");
    }
}