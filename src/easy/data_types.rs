//use std::fmt::Display;

//use rand::seq::index;

pub fn run() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!(
        "the char c is:{}, z is:{}, heart_eyed_cat is:{}",
        c, z, heart_eyed_cat
    );
    let (x, y, z) = (1, 2, 3);
    println!(
        "the tuple x is:({},{},{})",
        (x, x, c).0,
        (x, y, z).1,
        (x, y, c).2
    );
    let _index = "test";
    println!("{}", &_index);
}
