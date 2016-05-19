use std::f64;

// https://en.wikipedia.org/wiki/Fermat's_factorization_method
pub fn fermat_factors(n_u64 : u64) -> Vec<u64> {
    let n = n_u64 as f64;
    let mut a = f64::ceil(f64::sqrt(n));
    let mut b2 = a*a - n;

    // By definition, b2 is an integer
    while !is_square(b2 as u64) {
        a = a + 1.0;
        b2 = a*a - n;
    }

    vec!( (a - f64::sqrt(b2)) as u64, (a + f64::sqrt(b2)) as u64)
}

pub fn prime_factors(number : u64) -> Vec<u64> {
    let mut n = number;
    let mut factors = Vec::new();
    let mut d = 2;
    while n > 1 {
        while n % d == 0 {
            factors.push(d);
            n = n / d;
        }
        d = d + 1;
    }
    factors
}


pub fn is_prim(n : i32) -> bool {
    for i in 2..n-1 {
        // Why can't I just "{ false } ?"
        if (n % i) == 0 { return false; }
    }
    true
}

fn is_pair(n : i32) -> bool {
    (n % 2) == 0
}

fn is_square(n_u64 : u64) -> bool {
    let n = n_u64 as f64;
    f64::powi(f64::trunc(f64::sqrt(n)), 2) == n
}

#[test]
fn is_odd_test() {
    assert_eq!(true, is_pair(2));
}

#[test]
fn is_square_test() {
    assert_eq!(true, is_square(4));
    assert_eq!(true, is_square(344_569));
    assert_eq!(true, is_square(623_553_439_716));
    assert_eq!(false, is_square(7));
    assert_eq!(false, is_square(21));
    assert_eq!(false, is_square(125));
}

#[test]
fn fermat_factors_test() {
    let f = fermat_factors(600_851_475_143);
    assert_eq!(486_847, f[0]);
    assert_eq!(1_234_169, f[1]);
}

#[test]
fn prime_factors_test() {
    let mut f = prime_factors(13_195);
    assert_eq!(29, f[f.len() -1]);

    f = prime_factors(600_851_475_143);
    assert_eq!(6857, f[f.len() -1]);
}
