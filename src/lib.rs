/// Given a number n, return the nth Fibonacci number.
pub fn fib(n: u8) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut prev = 0;
            let mut curr = 1;
            for _ in 2..=n {
                let next = prev + curr;
                prev = curr;
                curr = next;
            }
            curr
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_formula() {
        assert_eq!(fib(10), 55);
        assert_eq!(fib(20), 6765);
    }

    #[test]
    fn max_u128() {
        assert_eq!(fib(186), 332825110087067562321196029789634457848);
    }
}
