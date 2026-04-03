use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", format!("{:#?}{}", self.name, self.age))
    }
}

// 单元结构体
struct Unit;

// 元组结构体
struct Pair(i32, f32);

// 带有两个字段的结构体
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// 结构体可以作为另一个结构体的字段
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    // 可以在空间中给定左上角和右下角在空间中的位置来指定矩形。
    top_left: Point,
    bottom_right: Point,
}

//传递的是对变量的引用, 否则将是所有权
fn rect_area(rect: &Rectangle) -> f32 {
    let Rectangle {
        top_left,
        bottom_right,
    } = rect; // 这里 rect 是一个引用，但解构时字段会被自动引用或拷贝
    let width = bottom_right.x - top_left.x;
    let height = bottom_right.y - top_left.y;
    width * height
}

//这里传递的是所有权
fn square(point: Point, edge_len: f32) -> Rectangle {
    let Point { x, y } = point;
    let Point { x: x1, y: y1 } = point;
    Rectangle {
        top_left: Point { x, y },
        bottom_right: Point {
            x: Point { x: x1, y: y1 }.x + edge_len,
            y: Point { x: x1, y: y1 }.y - edge_len,
        },
    }
}

pub fn run() {
    // 使用简单的写法初始化字段，并创建结构体
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // 以 Debug 方式打印结构体
    println!("{:?}", peter);

    // 实例化结构体 `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // 访问 point 的字段
    println!("point coordinates: ({}, {})", point.x, point.y);

    // 使用结构体更新语法创建新的 point，
    // 这样可以用到之前的 point 的字段
    let bottom_right = Point { x: 5.2, y: point.y };

    // `bottom_right.y` 与 `point.y` 一样，因为这个字段就是从 `point` 中来的
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // 使用 `let` 绑定来解构 point
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;
    /*
    let p = Point {
        x: 1f32,
        y: point.y,
    };
    */
    println!("{}", point.x);
    let _rectangle = Rectangle {
        // 结构体的实例化也是一个表达式
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };
    // 传递 _rectangle 的引用，而不是所有权
    println!("rectangle area: {}", rect_area(&_rectangle));
    // _rectangle 在这里仍然有效，可以继续使用. 但如果将语句移到上一条的前面会出错, 因为square函数传递的是所有权, 执行后变量的所有权也就无了
    println!("rectangle location: {:?}", square(_rectangle.top_left, 5.0));

    //)

    // 实例化一个单元结构体
    let _unit = Unit;

    // 实例化一个元组结构体
    let pair = Pair(1, 0.1);

    // 访问元组结构体的字段
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // 解构一个元组结构体
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}
