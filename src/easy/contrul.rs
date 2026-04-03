fn test_with_return() -> i32 {
    let mut i = 3;
    loop {
        i -= 1;
        if i == 0 {
            println!("loop finished");
            break;
        }
    }

    while i != -3 {
        println!("while loop: {}", i);
        i -= 1;
    }
    i
}

pub fn run() {
    let return_value = test_with_return();
    println!("return value: {}", return_value);
}
