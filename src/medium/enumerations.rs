//枚举

enum IpAddrKind {
    V4,
    V6,
}

#[allow(unused)]
#[derive(Debug)]
enum NewIpAddrKind {
    V4(String),
    V6(String),
}

//#[allow(dead_code)]
impl NewIpAddrKind {
    fn print(&self) {
        println!("要打印的IP是: {:?}", self);
    }
}

//#![allow(dead_code)]
struct IpAddr {
    kind: IpAddrKind,
    _address: String,
}

fn route(_ip_kind: IpAddrKind) {}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    let local = IpAddr {
        kind: IpAddrKind::V4,
        _address: String::from("127.0.0.1"),
    };
    let _public = IpAddr {
        kind: IpAddrKind::V6,
        _address: String::from("::1"),
    };
    route(four);
    route(six);
    route(local.kind);
    let _local = NewIpAddrKind::V4(String::from("127.0.0.1"));
    let _public = NewIpAddrKind::V6(String::from("::1"));
    _local.print();
    _public.print();

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
}

/*
Rust 并没有空值，不过它确实拥有一个可以编码存在或不存在概念的枚举。这个枚举是 Option<T>, 该枚举定义于标准库中. 且被包含于prelude中, 不必显式引入作用域
可以不需要 Option:: 前缀来直接使用 Some 和 None。即便如此 Option<T> 也仍是常规的枚举，Some(T) 和 None 仍是 Option<T> 的成员
enum Option<T> {//T, 泛型参数
    Some(T),
    None,
}

*/
