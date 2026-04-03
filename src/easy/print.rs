pub fn run() {
    // 通常情况下，`{}` 会被任意变量内容所替换。
    // 变量内容会转化成字符串。
    // format! 宏会返回一个格式化后的字符串（String），而不是打印到控制台。
    let str = "World";
    let formatted_str = format!("Hello, {}!", str);
    println!("This string was created by format!: {}", formatted_str);

    println!("{} days", 31i64);

    // 不加后缀的话，31 就自动成为 i32 类型。
    // 你可以添加后缀来改变 31 的类型（例如使用 31i64 声明 31 为 i64 类型）。

    // 用变量替换字符串有多种写法。
    // 比如可以使用位置参数。
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 可以使用命名参数。
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // 可以在 `:` 后面指定特殊的格式。
    println!("{} of {:?} people know binary, the other half don't", 1, 2);

    // 你可以按指定宽度来右对齐文本。
    // 下面语句输出 "     1"，5 个空格后面连着 1。
    println!("{number:>width$}", number = 1, width = 6);

    //[fill]< - 参数在 width 列中左对齐
    //[fill]^ - 参数在 width 列中居中对齐
    //[fill]> - 参数在 width 列中右对齐
    // 你可以在数字左边补 0。下面语句输出 "000001"。
    //println!("{number:-^width$}", number = 1, width = 6);
    println!("{:-^5}", "1");
    println!("Hello {:^15}!", format!("{:?}", Some("hi"))); // => "Hello   Some("hi")   !"
    // println! 会检查使用到的参数数量是否正确。
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // 改正 ^ 补上漏掉的参数："James"

    // 创建一个包含单个 `i32` 的结构体（structure）。命名为 `Structure`。
    //#[allow(dead_code)]
    #[derive(Debug)] //所有类型都能推导fmt::Debug的实现,但是fmt::Display需要手动实现
    struct Structure(i32);
    println!("structure: {}", Structure(3).0);
    // 但是像结构体这样的自定义类型需要更复杂的方式来处理。
    // 如果{}则下面语句无法运行。或者{:#?}提供了基础美化,虽然感觉更丑了
    println!("This struct `{:?}` won't print...", Structure(3));
    // 改正 ^ 注释掉此行。
}
