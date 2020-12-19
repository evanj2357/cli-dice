//! Command-line program to simulate dice rolls.

use clap::{crate_authors, crate_version, values_t, App, Arg};

mod dice;

fn main() {
    let matches = App::new("roll")
        .version(crate_version!())
        .author(crate_authors!())
        .about("a simple command-line dice roll simulator")
        .arg(
            Arg::with_name("dice spec")
                .takes_value(true)
                .multiple(true)
                .help("specify dice in [<number>]d<number> format, i.e. 'd6' or '3d10'"),
        )
        .get_matches();

    // parse dice to roll from command line args, relying on the FromStr trait
    let dice = values_t!(matches, "dice spec", dice::Dice).unwrap_or_else(|err| err.exit());
    let labels = matches
        .values_of("dice spec")
        .unwrap()
        .collect::<Vec<&str>>();

    let mut total = 0;
    for (i, d) in dice.iter().enumerate() {
        let roll = d.roll();
        println!("Rolled {}, got: {}", labels[i], roll);
        total += roll;
    }
    println!("Total: {}", total);
}
