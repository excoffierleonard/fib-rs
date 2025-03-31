use clap::Parser;
use fibtest::fib;

#[derive(Parser)]
struct Args {
    /// The nth Fibonacci number to compute
    n: u8,
}

fn main() {
    let args = Args::parse();
    println!("{}", fib(args.n));
}
