extern crate ga;

use ga::{GeneticAlgorithm, Individual, Sentence};

fn main() {
    let generations = 1000;
    let population_size = 10000;

    let mut ga = GeneticAlgorithm::<Sentence>::new(population_size);

    for i in 0..generations {
        ga.evaluate();
        ga.evolve();

        let best = ga.best_individual();

        println!("[{}]: {}", i, best);
        if let Some(fitness) = best.fitness() {
            if (fitness - 1.0).abs() < ga::ALLOWED_FITNESS_ERROR {
                std::process::exit(0);
            }
        }
    }
}
