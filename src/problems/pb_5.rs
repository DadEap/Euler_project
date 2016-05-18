// Boring Bruteforce, but the algorithm was obvious
pub fn solve() {
    let mut n = 2520;
    let mut we_found_it = false;
    let upper_bound = 20;
    loop {
        for x in 1..upper_bound+1 {
            if n % x != 0 {break;}
            if x == upper_bound {we_found_it = true;}
        }
        if we_found_it {break;}
        n = n + 2520;
    }
    println!("{}", n);
}
