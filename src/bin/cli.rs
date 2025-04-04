use clap::{Parser, Subcommand};
use fib_rs::{fib, fib_range};

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Calculate a single Fibonacci number
    Single {
        /// The nth Fibonacci number to compute
        n: u128,
    },
    /// Calculate a range of Fibonacci numbers
    Range {
        /// Starting index (inclusive)
        start: u128,
        /// Ending index (inclusive)
        end: u128,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Single { n } => {
            let result = fib(*n);
            println!("F({}) = {}", n, result);
        }
        Commands::Range { start, end } => {
            let range = *start..=*end;
            let results = fib_range(range);

            if results.is_empty() {
                eprintln!("Invalid range: end < start");
                return;
            }

            (*start..=*end)
                .zip(results.iter())
                .for_each(|(i, result)| println!("F({}) = {}", i, result));
        }
    }
}
