fn custom_fun(a: i32, b: i32) -> i32 {
    a + b
}

pub fn run() {
    let mut a = 0;
    println!("the value of a is {}", a);
    println!("Hello, world!");
    a = 1;
    println!("now the value of a is {}", a);
    a = custom_fun(a, 10);
    println!("now the value of a is {}", a);
    loop {
        a = a - 5;
        println!("looping..., now the value of a is {}", a);
        if a <= 0 {
            break;
        }
    }
    for i in 1..4 {
        println!("for循环...now the value of i is {}", i);
    }
}
