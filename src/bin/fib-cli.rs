use clap::Parser;
use fib::fib;

#[derive(Parser)]
struct Args {
    /// The nth Fibonacci number to compute
    n: u128,
}

fn main() {
    let args = Args::parse();
    println!("{}", fib(args.n));
}
