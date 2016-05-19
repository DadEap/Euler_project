use utils::prim;

// Bruteforce, to see the perfs (which should be terrible)
// Was not so bad actually
pub fn solve() {
    let mut n = 1;
    let mut nb_prim = 0;
    while nb_prim != 10_001 {
        n = n + 1;
        if prim::is_prim(n) { nb_prim = nb_prim + 1;}
    }
    println!("The {}th prime number is {}", nb_prim, n);
}
