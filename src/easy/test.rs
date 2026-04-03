fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..] //当在一个引用类型上使用 . 或 [] 操作符时，Rust 编译器会自动地、隐式地插入解引用操作（*）, 所以s[..]实际是(*s)[..]
}

#[allow(dead_code)]
fn test() {
    let mut string = String::from("hello world!");
    let word = first_word(&string);

    // string.clear(); // 这行会报错，因为 word 还在借用 string

    println!("the first word is: {}", word);

    // 如果想修改字符串，需要操作可变的 String
    string.replace_range(..1, "H");
    println!("modified string: {}", string);
}

pub fn run() {}
