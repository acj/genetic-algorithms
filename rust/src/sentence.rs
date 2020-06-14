use crate::Individual;

use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::cmp::Ordering;
use std::fmt;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz ";

#[derive(Debug, Clone)]
pub struct Sentence {
    pub genotype: String,
    fitness: Option<f64>,
}

impl Sentence {
    pub fn new(genotype: String) -> Self {
        Self {
            genotype,
            fitness: None,
        }
    }

    pub fn ideal() -> Self {
        Self {
            genotype: String::from("The quick brown fox jumped over the lazy dog"),
            fitness: Some(1.0),
        }
    }

    fn crossover_with_pivot(&self, other: Self, pivot: usize) -> Self {
        let first_half = self.genotype[0..pivot].to_owned();
        let second_half = &other.genotype[pivot..];

        Self::new(first_half + second_half)
    }
}

impl Individual for Sentence {
    fn evaluate(&mut self) {
        if self.fitness == None {
            // Assumption: only ascii characters in the genotype
            let ideal_sentence = Sentence::ideal();
            let optimal_genes = ideal_sentence.genotype.chars();
            let my_genes = self.genotype.chars();

            let num_matches = my_genes
                .zip(optimal_genes)
                .filter(|(my_gene, optimal_gene)| my_gene == optimal_gene)
                .count();

            self.fitness = Some(num_matches as f64 / (ideal_sentence.genotype.len() as f64));
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

    fn fitness(&self) -> Option<f64> {
        self.fitness
    }
}

impl PartialOrd for Sentence {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if let (Some(my_fitness), Some(their_fitness)) = (self.fitness(), other.fitness()) {
            return my_fitness.partial_cmp(&their_fitness);
        }

        return None;
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
        if let Some(fitness) = self.fitness {
            write!(f, "\"{}\", fitness {}", self.genotype, fitness)
        } else {
            write!(f, "\"{}\", fitness not yet evaluated", self.genotype)
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

        assert_eq!(excellent.fitness.unwrap(), 1.0);
        assert_eq!(terrible.fitness.unwrap(), 0.0);
    }
}
