extern crate ga;

use ga::{GeneticAlgorithm, Individual, Sentence};
use rand::{rngs::StdRng, SeedableRng};
use std::env;
use std::time::SystemTime;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 || args.len() > 4 {
        println!(
            "usage: {} <population size> <generations> [seed value]",
            args[0]
        );
        return Err("invalid args");
    }

    let population_size =
        str::parse(args.get(1).unwrap()).expect("Couldn't parse population size value");
    let generations = str::parse(args.get(2).unwrap()).expect("Couldn't parse generations value");
    let seed = match args.get(3) {
        Some(seed) => str::parse(seed).expect("Couldn't parse seed value"),
        None => time_since_epoch(),
    };

    println!("Seed value: {}", seed);
    let rng = StdRng::seed_from_u64(seed);
    let mut ga = GeneticAlgorithm::<_, Sentence>::new(population_size, rng);

    for i in 0..generations {
        ga.evaluate();
        ga.evolve();

        let best = ga.best_individual();

        println!("[{}]: {}", i, best);
        if let Some(fitness) = best.fitness() {
            if (fitness - 1.0).abs() < ga::ALLOWED_FITNESS_ERROR {
                break;
            }
        }
    }

    Ok(())
}

fn time_since_epoch() -> u64 {
    let time_since_epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Failed to get time-based seed value");
    time_since_epoch.as_secs()
}
