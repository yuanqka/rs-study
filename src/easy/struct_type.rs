//有关结构体的一些代码学习
use std::fmt;

// 使用生命周期参数 'a
#[derive(Debug)]
struct User<'a> {
    username: &'a str, //不可以直接使用&str, 因为这是对字符串切片的引用, 不可知其生命周期, 如要用引用, 应像这样指出其生命周期
    //或者username::String,
    email: &'a str,
    sign_in_count: u64,
    active: bool,
}

//
impl<'a> fmt::Display for User<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

//元组结构体, 没有字段名,只有字段类型, 可以为整个元组起一个名字,成为一个类型时较为方便.
#[derive(Debug)]
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn print_color(color: &Color) {
    println!("{:#?}", color);
}
//为Color实现一个方法
impl Color {
    // `&mut self` 是一个对 Color 实例的可变引用
    fn set_to_red(&mut self) {
        // 方式一：逐个修改字段
        (*self).0 = 255;
        // self.1 = 0;
        // self.2 = 0;
        // 方式二：整体赋值。*self 解引用得到实例本身，然后用一个新的 Color 实例的值来覆盖它。
        // 因为 Color 实现了 Copy trait，这里发生的是内存复制，而不是所有权转移。
        *self = Color(255, 0, 0);
    }
}

//无字段结构体, 类单元结构体, 常用于为某个类型实现trait
#[derive(Debug)]
struct NanFiledStruct; //不需要大括号或者小括号. 帕斯卡命名法

fn build_user<'a>(email: &'a str, username: &'a str) -> User<'a> {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

pub fn run() {
    // 字符串字面量是 &'static str 类型，'static 生命周期意味着它在整个程序运行期间都有效
    let mut user = User {
        username: "lihua",
        email: "114@514.com",
        sign_in_count: 0,
        active: false,
    };
    user.username = "lihua_changed";
    user.email = "27@example.com";
    user.sign_in_count = 1;
    user.active = true;
    println!("{}", user);
    println!("{:?}", build_user("114@514.com", "lihua"));
    let mut color = Color(122, 122, 122);
    let point = Point(1, 2, 3);
    if color.0 == point.0 {
        println!("相同");
    } else {
        println!("不同");
    }
    print_color(&color);
    color.set_to_red(); //注意这里所有权没被转移, 如果被转移(print_color函数不使用引用参数), 则无法再继续使用
    print_color(&color);
    print!("{}, {}, {}, {}", color.1, color.2, point.1, point.2);

    let nan_filed_struct = NanFiledStruct;
    println!("{:#?}", nan_filed_struct);
}
