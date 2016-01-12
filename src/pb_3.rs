pub fn solve() {
    //let num = 600851475143;
    //let num = 13195;
    /*let mut prim_factors = vec!(2, 3, 5, 7, 11, 13, 17, 19, 23, 29);/*, 31, 37, 41, 43, 47,
        53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131,
        137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 197, 199);*/
    prim_factors.reverse();*/
    let vec = (1..1000).filter(|&x| is_odd(x));
    for i in vec {
        println!("{}", i);
    }

    /*
    if (is_factor(num, prim_factors.first()) && reste!=1{
        mettre de côté le reste
        refaire avec num=reste, et reboucler sur le sous-vecteur
    } else if !(is_factor(num, prim_factors.first()))
        reboucler sur un sous-vecteur
    else if (reste==1)
        ecrire résultat
        */

    /*
    for i in prim_factors {
        println!("{}", i);
    }
    */
}



pub fn is_odd(n : i32) -> bool {
    (n % 2) == 0
}
/*
pub fn is_factor(n : i32, d : i32) -> bool {
    (n % d) == 0
}
*/
