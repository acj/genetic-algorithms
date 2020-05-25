extern crate rand;

use rand::distributions::{Distribution, Uniform};
use rand::Rng;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz ";

struct GeneticAlgorithm;

impl GeneticAlgorithm {
    fn seed(population_size: usize, genotype_size: usize) -> Vec<Individual> {
        let mut individuals: Vec<Individual> = vec![];

        for _ in 0..population_size {
            individuals.push(GeneticAlgorithm::generate_individual(genotype_size));
        }

        return individuals;
    }

    fn evaluate(population: &mut Vec<Individual>, ideal_genotype: &String) {
        // TODO: parallelize
        for individual in population {
            // Assumption: only ascii characters in the genotype
            let optimal_genes = ideal_genotype.chars();
            let num_matches = individual
                .genotype
                .chars()
                .zip(optimal_genes)
                .filter(|(my_gene, optimal_gene)| my_gene == optimal_gene)
                .count();

            individual.fitness = num_matches as f64 / (ideal_genotype.len() as f64);
        }
    }

    fn select(population: &Vec<Individual>, top: usize) -> Vec<Individual> {
        let mut members = population.to_vec();
        members.sort_unstable_by(|individual1, individual2| {
            individual1
                .fitness
                .partial_cmp(&individual2.fitness)
                .unwrap()
        });
        return members.iter().rev().take(top).cloned().collect();
    }

    fn next(population: Vec<Individual>, mutation_probability: f64) -> Vec<Individual> {
        let mut new_population = GeneticAlgorithm::select(&population, population.len() / 4);
        let random_individuals_needed = population.len() / 4;
        let crossover_individuals_needed =
            population.len() - new_population.len() - random_individuals_needed;
        for i in &new_population {
            i.mutate(mutation_probability);
        }
        for _ in 0..random_individuals_needed {
            new_population.push(GeneticAlgorithm::random_individual(&population));
        }
        for _ in 0..crossover_individuals_needed {
            let first_individual = GeneticAlgorithm::random_individual(&population);
            let second_individual = GeneticAlgorithm::random_individual(&population);
            let crossed_individual = first_individual.crossover(second_individual);
            new_population.push(crossed_individual);
        }
        return new_population;
    }

    fn best_individual(population: &Vec<Individual>) -> Individual {
        let mut best = population.first().unwrap().to_owned();
        for individual in population {
            if individual.fitness > best.fitness {
                best = individual.to_owned();
            }
        }
        return best;
    }

    fn generate_individual(genotype_size: usize) -> Individual {
        let mut rng = rand::thread_rng();

        let genotype: String = (0..genotype_size)
            .map(|_| {
                let idx = rng.gen_range(0, CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        Individual {
            genotype: genotype,
            fitness: -1.0,
        }
    }

    fn random_individual(population: &Vec<Individual>) -> Individual {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0, population.len());
        return population[idx].clone();
    }
}

#[derive(Debug, Clone)]
struct Individual {
    genotype: String,
    fitness: f64,
}

impl Individual {
    fn mutate(&self, per_site_mut_rate: f64) -> String {
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

        String::from_utf8(genotype).unwrap()
    }

    fn crossover_with_pivot(&self, other: Individual, pivot: usize) -> Individual {
        let first_half = self.genotype[0..pivot].to_owned();
        let second_half = &other.genotype[pivot..];

        return Individual {
            genotype: first_half + second_half,
            fitness: 0.0,
        };
    }

    fn crossover(&self, other: Individual) -> Individual {
        let mut rng = rand::thread_rng();
        let pivot = rng.gen_range(0, self.genotype.len());

        return self.crossover_with_pivot(other, pivot);
    }
}

fn main() {
    let ideal_genotype = String::from("The quick brown fox jumped over the lazy dog");
    let population_size = 10000;
    let generations = 100;
    let mutation_prob = 1.0 / ideal_genotype.len() as f64;
    let mut pop = GeneticAlgorithm::seed(population_size, ideal_genotype.len());
    for i in 0..generations {
        GeneticAlgorithm::evaluate(&mut pop, &ideal_genotype);
        pop = GeneticAlgorithm::next(pop.to_owned(), mutation_prob);
        let best = GeneticAlgorithm::best_individual(&pop);
        println!("[{}]: \"{}\", fitness {}", i, best.genotype, best.fitness);
        if best.fitness == 1.0 {
            std::process::exit(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crossover_with_pivot() {
        let i1 = Individual {
            genotype: "abcd".to_owned(),
            fitness: 0.0,
        };
        let i2 = Individual {
            genotype: "efgh".to_owned(),
            fitness: 0.0,
        };
        let crossed_individual = i1.crossover_with_pivot(i2, 1);
        assert_eq!(crossed_individual.genotype, "afgh");
    }

    #[test]
    fn evaluate() {
        let optimal_genotype = String::from("abcd");
        let terrible_genotype = String::from("1234");
        let mut population = vec![
            Individual {
                genotype: optimal_genotype.to_owned(),
                fitness: 0.0,
            },
            Individual {
                genotype: terrible_genotype.to_owned(),
                fitness: 0.0,
            },
        ];
        GeneticAlgorithm::evaluate(&mut population, &optimal_genotype);

        assert_eq!(population[0].fitness, 1.0);
        assert_eq!(population[1].fitness, 0.0);
    }

    #[test]
    fn select() {
        let optimal_genotype = String::from("abcd");
        let terrible_genotype = String::from("1234");
        let mut population = vec![
            Individual {
                genotype: optimal_genotype.to_owned(),
                fitness: 0.0,
            },
            Individual {
                genotype: terrible_genotype.to_owned(),
                fitness: 0.0,
            },
        ];
        GeneticAlgorithm::evaluate(&mut population, &optimal_genotype);

        let selected = GeneticAlgorithm::select(&population, 1);
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].genotype, optimal_genotype);
    }
}
