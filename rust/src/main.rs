extern crate rand;

use rand::Rng;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz ";

struct GeneticAlgorithm {
    population: Vec<Individual>,
}

impl GeneticAlgorithm {
    fn seed(&mut self, population_size: usize, genotype_size: usize) {
        let mut individuals: Vec<Individual> = vec![];
    
        for _ in 0..population_size {
            individuals.push(GeneticAlgorithm::generate_individual(genotype_size));
        }

        self.population = individuals;
    }

    fn evaluate(&mut self, ideal_genotype: &String) {
        // TODO: parallelize
        for individual in &mut self.population {
            let mut matches: i64 = 0;

            // Assumption: only ascii characters in the genotype
            let optimal_genes = ideal_genotype.as_bytes();
            let self_genes = individual.genotype.as_bytes();

            for i in 0..individual.genotype.len() {
                if self_genes[i] == optimal_genes[i] {
                    matches += 1;
                }
            }

            individual.fitness = matches as f64 / (optimal_genes.len() as f64);
        }
    }

    fn select(&self, top: usize) -> Vec<Individual> {
        let mut members = self.population.to_vec();
        members.sort_by(|individual1, individual2| individual1.fitness.partial_cmp(&individual2.fitness).unwrap());
        return members.iter().rev().take(top).cloned().collect();
    }

    fn next(&mut self, mutation_probability: f64) {
        let mut new_population = self.select(self.population.len() / 4);
        let random_individuals_needed = self.population.len() / 4;
        let crossover_individuals_needed = self.population.len() - new_population.len() - random_individuals_needed;
        for i in &new_population {
            i.mutate(mutation_probability);
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

    fn best_individual(&self) -> Individual {
        let mut members = self.population.to_vec();
        members.sort_by(|individual1, individual2| individual1.fitness.partial_cmp(&individual2.fitness).unwrap());
        let best: Vec<Individual> = members.iter().rev().take(1).cloned().collect();
        return best[0].to_owned();
    }

    fn generate_individual(genotype_size: usize) -> Individual {
        let mut rng = rand::thread_rng();
    
        let genotype: String = (0..genotype_size)
            .map(|_| {
                let idx = rng.gen_range(0, CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
        
        Individual{
            genotype: genotype,
            fitness: -1.0
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
    fitness: f64
}

impl Individual {
    fn mutate(&self, per_site_mut_rate: f64) -> String {
        let mut rng = rand::thread_rng();
        let mutated_genotype: String = 
            (0..self.genotype.len())
            .map(|i| {
                let probability = rng.gen_range(0.0, 1.0);
                if probability <= per_site_mut_rate {
                    let idx = rng.gen_range(0, CHARSET.len());
                    CHARSET[idx] as char
                } else {
                    self.genotype.as_bytes()[i] as char
                }
                
            })
            .collect();

        mutated_genotype
    }

    fn crossover_with_pivot(&self, other: Individual, pivot: usize) -> Individual {
        let first_half = self.genotype[0..pivot].to_owned();
        let second_half = &other.genotype[pivot..];
        
        return Individual{genotype: first_half + second_half, fitness: 0.0};
    }

    fn crossover(&self, other: Individual) -> Individual {
        let mut rng = rand::thread_rng();
        let pivot = rng.gen_range(0, self.genotype.len());

        return self.crossover_with_pivot(other, pivot);
    }
}

fn main() {
    let ideal_genotype = String::from("The quick brown fox jumped over the lazy dog");
    let population_size = 2000;
    let generations = 100;
    let mutation_prob = 1.0 / ideal_genotype.len() as f64;
    let mut ga = GeneticAlgorithm{
        population: vec![],
    };
    ga.seed(population_size, ideal_genotype.len());
    for i in 0..generations {
        ga.evaluate(&ideal_genotype);
        ga.next(mutation_prob);
        let best = ga.best_individual();
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
    fn test_individual_crossover_with_pivot() {
        let i1 = Individual{genotype: "abcd".to_owned(), fitness: 0.0};
        let i2 = Individual{genotype: "efgh".to_owned(), fitness: 0.0};
        let crossed_individual = i1.crossover_with_pivot(i2, 1);
        assert_eq!(crossed_individual.genotype, "afgh");
    }

    #[test]
    fn test_individual_fitness() {
        let optimal_genotype = String::from("abcd");
        let terrible_genotype = String::from("1234");
        let mut ga = GeneticAlgorithm{
            population: vec![
                Individual{genotype: optimal_genotype.to_owned(), fitness: 0.0},
                Individual{genotype: terrible_genotype.to_owned(), fitness: 0.0}
            ]
        };
        ga.evaluate(&optimal_genotype);

        assert_eq!(ga.population[0].fitness, 1.0);
        assert_eq!(ga.population[1].fitness, 0.0);
    }

    #[test]
    fn test_ga_select() {
        let optimal_genotype = String::from("abcd");
        let terrible_genotype = String::from("1234");
        let mut ga = GeneticAlgorithm{
            population: vec![
                Individual{genotype: optimal_genotype.to_owned(), fitness: 0.0},
                Individual{genotype: terrible_genotype.to_owned(), fitness: 0.0}
            ]
        };
        ga.evaluate(&optimal_genotype);

        let selected = ga.select(1);
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].genotype, optimal_genotype);
    }
}