extern crate rand;

use rayon::prelude::*;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::cmp::Ordering;
use std::fmt;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz ";
const NOT_YET_EVALUATED: f64 = -1.0;
const ALLOWED_FITNESS_ERROR: f64 = 0.001;

pub trait Individual: Clone + Sync + Send + fmt::Display + Ord {
    fn evaluate(&mut self);

    fn mutate(&self) -> Self;

    fn crossover(&self, other: Self) -> Self;

    fn generate() -> Self;

    fn fitness(&self) -> f64;
}

struct GeneticAlgorithm<T> {
    population: Vec<T>,
}

impl<T: Individual> GeneticAlgorithm<T> {
    fn new(population_size: usize) -> GeneticAlgorithm<T> {
        GeneticAlgorithm {
            population: Self::seed(population_size),
        }
    }

    fn seed(population_size: usize) -> Vec<T> {
        (0..population_size).map(|_| T::generate()).collect()
    }

    fn evaluate(&mut self) {
        self.population.par_iter_mut().for_each(|individual| {
            individual.evaluate();
        });
    }

    fn select(population: &[T], top: usize) -> Vec<T> {
        let mut members = population.to_vec();
        members.sort_unstable_by(|individual1, individual2| {
            individual1
                .fitness()
                .partial_cmp(&individual2.fitness())
                .unwrap()
        });
        members.iter().rev().take(top).cloned().collect()
    }

    fn evolve(&mut self) {
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

    fn best_individual(&self) -> T {
        self.population
            .iter()
            .max_by(|individual1, individual2| {
                individual1
                    .fitness()
                    .partial_cmp(&individual2.fitness())
                    .unwrap()
            })
            .unwrap()
            .clone()
    }

    fn random_individual(population: &[T]) -> T {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0, population.len());
        population[idx].clone()
    }
}

#[derive(Debug, Clone)]
struct Sentence {
    genotype: String,
    fitness: f64,
}

impl Sentence {
    fn new(genotype: String) -> Self {
        Self {
            genotype,
            fitness: NOT_YET_EVALUATED,
        }
    }

    fn ideal() -> Self {
        Self {
            genotype: String::from("The quick brown fox jumped over the lazy dog"),
            fitness: 1.0,
        }
    }

    fn crossover_with_pivot(&self, other: Self, pivot: usize) -> Self {
        let first_half = self.genotype[0..pivot].to_owned();
        let second_half = &other.genotype[pivot..];

        Self::new(first_half + second_half)
    }
}

impl PartialOrd for Sentence {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Sentence {
    fn cmp(&self, other: &Self) -> Ordering {
        // Assumption: we never use NaN as a fitness value
        self.fitness.partial_cmp(&other.fitness).unwrap()
    }
}

impl PartialEq for Sentence {
    fn eq(&self, other: &Self) -> bool {
        self.genotype == other.genotype
    }
}
impl Eq for Sentence {}

impl fmt::Display for Sentence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\", fitness {}", self.genotype, self.fitness)
    }
}

impl Individual for Sentence {
    fn evaluate(&mut self) {
        if (self.fitness - NOT_YET_EVALUATED).abs() < ALLOWED_FITNESS_ERROR {
            // Assumption: only ascii characters in the genotype
            let ideal_sentence = Sentence::ideal();
            let optimal_genes = ideal_sentence.genotype.chars();
            let my_genes = self.genotype.chars();

            let num_matches = my_genes
                .zip(optimal_genes)
                .filter(|(my_gene, optimal_gene)| my_gene == optimal_gene)
                .count();

            self.fitness = num_matches as f64 / (ideal_sentence.genotype.len() as f64);
        }
    }

    fn mutate(&self) -> Sentence {
        let per_site_mut_rate = 1.0 / Sentence::ideal().genotype.len() as f64;
        // TODO: Use random seed
        let mut rng = rand::thread_rng();
        let mut genotype = vec![0; self.genotype.len()];
        let genotype_bytes = self.genotype.as_bytes();
        let charset_between = Uniform::from(0..CHARSET.len());
        let probability_between = Uniform::from(0.0..1.0);

        for i in 0..self.genotype.len() {
            genotype[i] = if probability_between.sample(&mut rng) <= per_site_mut_rate {
                CHARSET[charset_between.sample(&mut rng)]
            } else {
                genotype_bytes[i]
            };
        }

        Sentence::new(String::from_utf8(genotype).unwrap())
    }

    fn crossover(&self, other: Sentence) -> Sentence {
        let mut rng = rand::thread_rng();
        let pivot = rng.gen_range(0, self.genotype.len());

        self.crossover_with_pivot(other, pivot)
    }

    fn generate() -> Sentence {
        let genotype_size = Sentence::ideal().genotype.len();
        let mut rng = rand::thread_rng();

        let genotype: String = (0..genotype_size)
            .map(|_| {
                let idx = rng.gen_range(0, CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        Sentence::new(genotype)
    }

    fn fitness(&self) -> f64 {
        self.fitness
    }
}

fn main() {
    let generations = 1000;
    let population_size = 10000;

    let mut ga = GeneticAlgorithm::<Sentence>::new(population_size);

    for i in 0..generations {
        ga.evaluate();
        ga.evolve();

        let best = ga.best_individual();

        println!("[{}]: {}", i, best);
        if (best.fitness - 1.0).abs() < ALLOWED_FITNESS_ERROR {
            std::process::exit(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crossover_with_pivot() {
        let i1 = Sentence::new(String::from("abcd"));
        let i2 = Sentence::new(String::from("efgh"));
        let crossed_individual = i1.crossover_with_pivot(i2, 1);
        assert_eq!(crossed_individual.genotype, "afgh");
    }

    #[test]
    fn evaluate() {
        let mut excellent = Sentence::new(Sentence::ideal().genotype);
        let mut terrible = Sentence::new(String::from("1234"));

        excellent.evaluate();
        terrible.evaluate();

        assert_eq!(excellent.fitness, 1.0);
        assert_eq!(terrible.fitness, 0.0);
    }

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
