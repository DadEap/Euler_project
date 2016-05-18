use utils::prim;

pub fn solve() {
    let a = prim::prime_factors(600_851_475_143);
    println!("{}", a[a.len()-1]);
}
