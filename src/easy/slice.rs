//切片, 对数据集某一部分的引用. 例如&string[x..y],是对string字符串[x, y)的数据集合的引用, 左闭右开.
// str 是一个动态大小类型 (DST)，被直接硬编码到binary文件, 代表连续的字符串数据。我们不能直接持有它。
// &str 是对 str 的引用（一个“胖指针”），它包含了地址和长度，大小是固定的。我们通常说的“字符串切片”就是指 &str 类型。

fn first_word(s: &String) -> &str {
    &s[..1]
}

pub fn run() {
    let s = String::from("Hello, world!"); //s是String类型的,不是&str, 如果想要,应是let s1: &str = "banana";
    //s = String::from("test");
    let not_ascii_str = String::from("这是UTF-8编码的字符串");
    let hello = &s[0..5];
    //let world = &s[6..11];
    // &str 类型不能被解引用（*hello）来获得一个 str 值，因为 str 是一个 DST，无法直接在栈上创建。
    // println! 宏会自动处理 &str，所以直接打印即可。
    println!("不显式解引用的引用hello:{}, 引用world:{}", hello, &s[6..11]);
    // s.clear(); // 这行会报错！因为 s 已经被不可变地借用给了 hello。只要 hello 还存活，s 就不能被修改。
    // println!("{}", hello); // hello 在这里仍然有效，所以上面的 s.clear() 会失败。

    let mut s2 = String::from("test");
    println!("first_word:{}, {}", &s2[..1], first_word(&s2));
    s2.clear(); // 这里是合法的，因为 first_word(&s2) 返回的切片没有被保存，它的生命周期在 println! 结束后就结束了。
    //let not_ascii_slice = &not_ascii_str[1..2];
    //println!("not_ascii_slice:{}", not_ascii_slice);
    //如果切片的边界落在了某个非ascii字符,也就是多字节字符的中间, 程序会崩溃退出, 虽然可以编译成功
    println!("not_ascii_str:{}", &not_ascii_str[0..not_ascii_str.len()]);

    //下面的代码测试中, String 没有实现 Copy trait。
    //但这段代码并不是“拷贝赋值”，而是 “移动赋值”。
    //在 Rust 中，赋值语句的语义取决于类型是否实现了 Copy：
    //✅ 如果类型实现了 Copy：赋值时会复制值。
    //🚫 如果类型没有实现 Copy：赋值时会移动所有权。
    let mut _string_test_copy = "hello".to_string();
    _string_test_copy = "world".to_string(); //"hello": String的值被释放, string_test_copy获得新的值得所有权.
    println!("string_test_copy实现了copy吗:{}", _string_test_copy);
}
