// Factored
pub fn solve() {
    let n = 100;
    let result = n * (n*n - 1) * (3*n + 2) / 12;
    println!("{}", result);
}

// use pow is a pain ... :/
pub fn solve2() {
    let n = 100;
    let square_of_sum = ((n*(n+1) / 2) as u32).pow(2);
    let sum_of_squares = n*(n+1)*(2*n+1) / 6;
    println!("{}", square_of_sum - sum_of_squares);
}
