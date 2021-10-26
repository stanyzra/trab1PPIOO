use trab::tokens_array;

fn main() {
    let _fon:Vec<String> = tokens_array(&"-20 + -51 + 20 + -68 * -11 + -35 * -14 - 95 - 32 + -52 * -23 - -90 * -42".replace(' ',&"".to_string()));
    println!("{:?}",_fon);
}

/*fn main() {
    let a = vec![1, 2, 3, 4, 5];

    // With a start and an end
    println!("{:?}", &a[0..5]);
    //[1, 2, 3, 4, 5]

    // With a start and an end, inclusive
    println!("{:?}", &a[1..=3]);
    //[2, 3, 4]

    // With just a start
    println!("{:?}", &a[2..]);

    // With just an end
    println!("{:?}", &a[..3]);

    // With just an end, inclusive
    println!("{:?}", &a[..=2]);

    // All elements
    println!("{:?}", &a[..]);
}*/
