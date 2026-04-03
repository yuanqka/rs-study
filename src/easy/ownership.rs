// 函数接收 &str 和 &String 类型
fn print_refs(str_var: &str, string_ref: &String) {
    println!("函数内打印: {}\n函数内打印: {}", str_var, *string_ref);
}

fn char_to_string(var: char) -> String {
    return var.to_string();
}

//作用域
fn scope(non_borrow_char: char, borrow_char: &mut char) {
    println!(
        "没有引用的char:{}, 引用的char:{}",
        non_borrow_char, borrow_char
    );
    println!("再次打印, 未引用:{}, 引用:{}", non_borrow_char, borrow_char);
    //let mut temp = 'c';
    //borrow_char = &mut 'c';
    // 使用解引用操作符 `*` 来修改引用所指向的值
    let temp = 'c';
    *borrow_char = temp; //borrow_char = &mut temp不合法,因为只是
    println!(
        "再次打印, 未引用:{}, 变更引用:{}",
        non_borrow_char, borrow_char
    );
    let mut temp = char_to_string('d');
    let c = &mut temp;
    println!("{}", c);
}

fn change(some_string: &mut String) {
    (*some_string).push_str(", world");//方法调用的自动解引用（Automatic Dereferencing）。所以也可以如下
    //*some_string.push_str(", world");
}

pub fn main() {
    let str_var = "这是一个字符串";
    let string = String::from("这是一个字符串");

    println!("传参前: {}, {}", str_var, string);

    // str_var 被复制，string 被借用
    print_refs(str_var, &string);

    println!(
        "传参后，因为 &str 是 Copy 类型, str_var 依然有效: {}",
        str_var
    );
    println!(
        "传参后，因为我们传递的是引用, string 的所有权没有移动，依然有效: {}",
        string
    );
    scope('a', &mut 'b');

    // 必须创建一个可变变量，才能获取它的可变引用
    // let mut my_char = 'b';
    // println!("调用 scope 前, my_char 的值是: {}", my_char);
    // scope('a', &mut my_char);
    // println!("调用 scope 后, my_char 的值是: {}", my_char);

    let mut change_string = String::from("Hello");
    change_string.push_str(", world");
    print!("{}", {
        change(&mut change_string);
        change_string
    });
    //println!("{}", change_string);
}
