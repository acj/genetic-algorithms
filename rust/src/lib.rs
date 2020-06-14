extern crate rand;

mod sentence;

pub use crate::sentence::Sentence;

use rand::Rng;
use rayon::prelude::*;
use std::fmt;

pub const ALLOWED_FITNESS_ERROR: f64 = 0.001;

pub trait Individual: Clone + Sync + Send + fmt::Display + Ord {
    fn evaluate(&mut self);

    fn mutate(&self) -> Self;

    fn crossover(&self, other: Self) -> Self;

    fn generate() -> Self;

    fn fitness(&self) -> Option<f64>;
}

pub struct GeneticAlgorithm<T> {
    population: Vec<T>,
}

impl<T: Individual> GeneticAlgorithm<T> {
    pub fn new(population_size: usize) -> GeneticAlgorithm<T> {
        GeneticAlgorithm {
            population: Self::seed(population_size),
        }
    }

    pub fn seed(population_size: usize) -> Vec<T> {
        (0..population_size).map(|_| T::generate()).collect()
    }

    pub fn evaluate(&mut self) {
        self.population.par_iter_mut().for_each(|individual| {
            individual.evaluate();
        });
    }

    pub fn select(population: &[T], top: usize) -> Vec<T> {
        let mut members = population.to_vec();
        members.sort();
        members.iter().rev().take(top).cloned().collect()
    }

    pub fn evolve(&mut self) {
        let mut new_population =
            GeneticAlgorithm::select(&self.population, self.population.len() / 4);
        let random_individuals_needed = self.population.len() / 4;
        let crossover_individuals_needed =
            self.population.len() - new_population.len() - random_individuals_needed;
        for i in &new_population {
            i.mutate();
        }
        for _ in 0..random_individuals_needed {
            new_population.push(GeneticAlgorithm::random_individual(&self.population));
        }
        for _ in 0..crossover_individuals_needed {
            let first_individual = GeneticAlgorithm::random_individual(&self.population);
            let second_individual = GeneticAlgorithm::random_individual(&self.population);
            let crossed_individual = first_individual.crossover(second_individual);
            new_population.push(crossed_individual);
        }
        self.population = new_population;
    }

    pub fn best_individual(&self) -> T {
        self.population.iter().max().unwrap().clone()
    }

    pub fn random_individual(population: &[T]) -> T {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0, population.len());
        population[idx].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sentence::Sentence;

    #[test]
    fn select() {
        let optimal_genotype = Sentence::ideal().genotype;
        let terrible_genotype = String::from("1234");
        let population = vec![
            Sentence::new(optimal_genotype.to_owned()),
            Sentence::new(terrible_genotype.to_owned()),
        ];
        let mut ga = GeneticAlgorithm { population };
        ga.evaluate();

        let selected = GeneticAlgorithm::select(&ga.population, 1);
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].genotype, optimal_genotype);
    }
}
