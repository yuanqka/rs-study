enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn main() {
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter = Coin::Quarter;
    println!("{} cents", value_in_cents(penny));
    let mut dice_roll = 4;
    loop {
        match dice_roll {
            3 => add_fancy_hat(),
            7 => (),
            9 => break,
            _ => dice_roll = reroll(),
        }
    }
    fn add_fancy_hat() {
        //函数可以定义在函数里?rust作用域?
        println!("Fancy hat!");
    }
    fn remove_fancy_hat() {
        println!("No fancy hat!");
    }
    fn reroll() -> i32 {
        println!("Re-rolling!");
        9
    }
    let some_u8_value = Some(3 as u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
