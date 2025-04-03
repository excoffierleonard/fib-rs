use clap::Parser;
use fib_rs::fib;

#[derive(Parser)]
struct Args {
    /// The nth Fibonacci number to compute
    n: u128,
}

fn main() {
    let args = Args::parse();
    let result = fib(args.n);
    println!("{}", result);
}
