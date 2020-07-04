extern crate rand;

mod sentence;

pub use crate::sentence::Sentence;

use rand::rngs::StdRng;
use rand::Rng;
use rayon::prelude::*;
use std::fmt;

pub const ALLOWED_FITNESS_ERROR: f64 = 0.001;

pub trait Individual: Clone + Sync + Send + fmt::Display + Ord {
    fn evaluate(&mut self);

    fn mutate(&self, rng: &mut StdRng) -> Self;

    fn crossover(&self, other: Self, rng: &mut StdRng) -> Self;

    fn generate(rng: &mut StdRng) -> Self;

    fn fitness(&self) -> Option<f64>;
}

pub struct GeneticAlgorithm<T> {
    population: Vec<T>,
    rng: StdRng,
}

impl<T: Individual> GeneticAlgorithm<T> {
    pub fn new(population_size: usize, rng: StdRng) -> GeneticAlgorithm<T> {
        let mut rng = rng;
        GeneticAlgorithm {
            population: Self::seed(population_size, &mut rng),
            rng: rng,
        }
    }

    pub fn seed(population_size: usize, rng: &mut StdRng) -> Vec<T> {
        (0..population_size).map(|_| T::generate(rng)).collect()
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
            i.mutate(&mut self.rng);
        }
        for _ in 0..random_individuals_needed {
            new_population.push(GeneticAlgorithm::random_individual(
                &self.population,
                &mut self.rng,
            ));
        }
        for _ in 0..crossover_individuals_needed {
            let first_individual =
                GeneticAlgorithm::random_individual(&self.population, &mut self.rng);
            let second_individual =
                GeneticAlgorithm::random_individual(&self.population, &mut self.rng);
            let crossed_individual = first_individual.crossover(second_individual, &mut self.rng);
            new_population.push(crossed_individual);
        }
        self.population = new_population;
    }

    pub fn best_individual(&self) -> T {
        self.population.iter().max().unwrap().clone()
    }

    pub fn random_individual(population: &[T], rng: &mut rand::rngs::StdRng) -> T {
        let idx = rng.gen_range(0, population.len());
        population[idx].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sentence::Sentence;
    use rand::{rngs::StdRng, SeedableRng};

    #[test]
    fn select() {
        let optimal_genotype = Sentence::ideal().genotype;
        let terrible_genotype = String::from("1234");
        let population = vec![
            Sentence::new(optimal_genotype.to_owned()),
            Sentence::new(terrible_genotype.to_owned()),
        ];
        let rng = StdRng::seed_from_u64(1234);
        let mut ga = GeneticAlgorithm { population, rng };
        ga.evaluate();

        let selected = GeneticAlgorithm::select(&ga.population, 1);
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].genotype, optimal_genotype);
    }
}
