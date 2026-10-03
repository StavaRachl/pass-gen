mod cli;
mod generator;

use clap::Parser;
use rand::rng;
use cli::Args;
use generator::{PasswordConfig, PasswordGenerator};

fn main() {
    let args: Args = Args::parse();
    
    let config = PasswordConfig {
        length: args.length,
        include_lower: args.lower,
        include_upper: args.upper,
        include_special: args.spec,
        word: args.word
    };
    
    let mut rng = rng();
    
    let password = PasswordGenerator::generate(&config, &mut rng);
    
    println!("{}", password)
}