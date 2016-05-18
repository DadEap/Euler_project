// Use of one ascending number n2, and one descending number n1

// If n1 * n2 palindrom, then no palindrom can be found for n1*(n2-1) or (n1-1)*n2
// So we change the starting value of n2, and continue

// If n1 == n2, then decrease n1, We then start again, because (n1 - 1)(n2 + 1) can be higher than n1 * n2 (always higher if n1 > n2, which is the case)
pub fn solve(){
    let mut n1 : u64 = 999;
    let mut n2 : u64 = 900;
    let mut result = 0;
    let mut last_min_n2 = n2;
    let mut tuple_result = (n1, n2);

    while n2 < n1 {
        while n1 != n2 || n2 < n1 {
            let n = n1 * n2;
            if is_palindrom(n.to_string()) && result < n {
                result = n;
                tuple_result = (n1, n2);
                last_min_n2 = n2;
            }
            n2 = n2 + 1;
        }
        n1 = n1 - 1;
        n2 = last_min_n2;
    }
    println!("Result = {} ! : n1 = {}, n2 = {}", result, tuple_result.0, tuple_result.1);
}

fn is_palindrom(str : String) -> bool {
    let length = str.len();
    if length % 2 == 0 {
        let tuple = str.split_at(length / 2);
        let reverse : String = (tuple.1).chars().rev().collect();
        tuple.0 == reverse
    } else {false}
}
