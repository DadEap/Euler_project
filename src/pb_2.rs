pub fn solve() {
    let mut vec = Vec::new();

    vec.push(0);
    vec.push(1);

    let mut num = 1;

    while num < 4000000 {
        let len = vec.len();
        num = vec[len-1] + vec[len-2];
        vec.push(num);
    }

    println!("{}", vec.iter().fold(0, |sum, xs| if xs % 2 == 0 {sum + xs} else {sum}));
}
