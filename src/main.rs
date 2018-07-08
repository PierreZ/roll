extern crate rand;

use rand::distributions::{Distribution, Uniform};

use std::env;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Bad input, expecting something like 1d100");
        std::process::exit(1);
    }

    let splitted: Vec<&str> = args[1].split("d").collect();

    let dices: i32 = splitted[0].parse().expect("Wanted a number");
    if dices < 1 || dices > 100 {
        println!("Bad input for dices, 1 < dices < 100");
        std::process::exit(1);
    }

    let sides: i32 = splitted[1].parse().expect("Wanted a number");
    if sides < 2 || sides > 100 {
        println!("Bad input for sides, 2 < sides < 100");
        std::process::exit(1);
    }

    let between = Uniform::new_inclusive(1, 100);
    let mut sum = 0;
    let mut picks: Vec<i32> = Vec::new();

    for _ in 0..dices {
        let mut rng = rand::thread_rng();
        let pick = between.sample(&mut rng);

        sum += pick;
        picks.push(pick);
    }

    println!("{}: {:?}", sum, picks);

    Ok(())
}
