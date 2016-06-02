use utils::prim;

// Eratothene solution
// A little bit slower than bruteforce solution (?!?)
// Must have a problem in implementation, and dispensable iterations
pub fn solve() {
    let nb_prim = 10_001;
    let vec = prim::eratosthene_prim_list(105000);
    println!("The {}th prime number is {}", nb_prim, vec[nb_prim]);
}


// Bruteforce, to see the perfs (which should be terrible)
// Was not so bad actually
pub fn solve_brute() {
    let mut n = 1;
    let mut nb_prim = 0;
    while nb_prim != 10_001 {
        n = n + 1;
        if prim::is_prim(n) { nb_prim = nb_prim + 1;}
    }
    println!("The {}th prime number is {}", nb_prim, n);
}
