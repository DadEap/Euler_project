pub fn solve() {
    let mut vec = Vec::new();

    vec.push(0);
    vec.push(1);

    let mut num = 1;

    while num < 4000000 {
        let len = vec.len();
        num = vec.get(len-1).unwrap() + vec.get(len-2).unwrap();
        vec.push(num);
    }

    println!("{}", vec.iter().fold(0, |sum, xs| if xs % 2 == 0 {sum + xs} else {sum}));
}
