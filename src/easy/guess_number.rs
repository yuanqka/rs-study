//use std::io;
//use core::num;
use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

//use crate::print;
pub fn run() {
    println!("guess the number!");
    let answer = rand::thread_rng().gen_range(1..101);
    println!("the secret number is: {}", answer);
    loop {
        println!("please input your answer and press enter: ");

        let mut guess = String::new();
        stdin()
            //io::stdin()
            .read_line(&mut guess) //&guess不可变引用, 此处一共有mut
            //前面的是一个函数与方法的结束,并返回了io::Result实例,后面的是该实例的方法
            .expect("failed to read line");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please input a number!");
                continue;
            }
        };
        match guess.cmp(&answer) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}
