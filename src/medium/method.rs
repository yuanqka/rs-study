//rust的方法语句. 在结构体, 枚举或trait上下文定义. 第一个参数是self, 代表调用该方法的实例
//方法可以在impl块中定义, 且不必全在一个块中.

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

//impl Solution 块的唯一目的是为 Solution 类型（即使你没有显式定义 struct Solution，Rust 也将其视为一个类型）定义其关联项 (Associated Items)。这些关联项只能是以下几种：
//关联函数（fn）：
//关联常量（const）： 必须是编译时可确定的值。
//关联类型（type）： 用于泛型等复杂场景。

impl Rectangle {
    const _TEST: i32 = 1;
    fn _print() {
        println!("{}", Self::_TEST);
    }
    fn area(&self) -> i32 {
        self.width * self.height
    }
    fn move_value(self) {
        println!("{:?} 的所有权已被转移", self);
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn height(&self) -> bool {
        self.height > 0
    }
}

impl Rectangle {
    fn resize(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
    }
    fn clone(&self) -> Rectangle {
        Rectangle {
            width: self.width,
            height: self.height,
        }
    }
}

//关联函数
impl Rectangle {
    fn square(/*&self, */ size: i32) -> Rectangle {
        //此处的第一个参数不是自身, 所以是函数不是方法
        Rectangle {
            width: size,
            height: size,
        }
    }
}

pub fn main() {
    println!("--- 演示 Option 和方法调用 ---");
    // 将 Rectangle 包装在 Option 中
    let mut rect_option: Option<Rectangle> = Some(Rectangle {
        width: 30,
        height: 30,
    });

    // 1. 通过引用（borrow）调用方法
    // 使用 `as_ref()` 或 `&` 来借用 Option 内部的值，而不转移所有权。
    // `rect` 的类型是 `&Rectangle`。
    if let Some(rect) = &rect_option {
        println!(
            "通过引用检查，矩形 {:?} 的面积是 {}。`rect_option` 仍为 Some。",
            rect,
            rect.area()
        );
    }
    println!("调用 area() 后: {:?}\n", rect_option);

    // 2. 通过 .take() 转移所有权
    // .take() 将值从 Option 中取出，留下 None。这允许我们获得值的所有权，同时保持 option 变量本身可用。
    if let Some(rect) = rect_option.take() {
        println!("通过 .take() 获取了值的所有权。");
        rect.move_value();
    }
    println!("调用 move_value() 后: {:?}\n", rect_option);

    println!("--- 演示关联函数和可变方法 ---");
    let mut rect = Rectangle::square(30);
    println!("Rectangle::square关联函数初始化{:?}", rect);
    println!("resize方法之后是{:?}", {
        rect.resize(50, 20);
        rect.clone()
    });
    println!(
        "rect长: {}, 宽: {} 是合法的: {}",
        rect.width,
        rect.height,
        rect.width() && rect.height()
    );
}
