use clap::Parser;

#[derive(Parser)]
struct Args {
    /// The nth Fibonacci number to compute
    n: u128,
}

fn main() {
    let args = Args::parse();
    println!("{}", fib_rs::fib(args.n));
}
