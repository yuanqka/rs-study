//声明式宏 (Declarative Macros)
//expr: 表达式（如 2 + 2）
//ident: 标识符（如函数名、变量名）
//ty: 类型（如 i32、Vec<String>）
//path: 路径（如 std::collections::HashMap）
//tt: 标记树（万能的，匹配几乎任何东西）
macro_rules! hello {
    ($name: expr) => {
        println!("Hello, {}!", $name);
    };
}

#[macro_export] //使用这个导出声明宏后不可以设置proc-macro = true
//过程宏（Proc-macro）和声明式宏（Declarative macro）属于完全不同的系统
//过程宏create中 Rust 禁止导出非过程宏的内容
macro_rules! hello_each {
    ($($name: expr), *) => {
        println!("Hello, {}!", stringify!($($name)*));
        $(println!("Hello, {}!", $name);)*
    };
}

macro_rules! pat {
    ($i:ident) => {
        Some($i)
    };
}

macro_rules! Tuple {
    { $A:ty, $B:ty } => { ($A, $B) };
}

macro_rules! const_maker {
    ($t:ty, $v:tt) => {
        const CONST: $t = $v;
    };
}

trait T {
    const_maker! {i32, 7} //展开变成const CONST: i32 = 7;
}

impl T for i32 {
    const CONST: i32 = 7;
}

//---  过程宏  ---
//在proc_macro_examples子create中定义, 不可以在此处定义
//过程宏本质上是一个编译器插件。为了运行宏，编译器必须先将其代码编译成机器码，然后动态加载到编译器中执行。
//编译顺序矛盾：编译器需要先完成整个 Crate 的编译才能得到可执行的宏。但如果在当前 Crate 中调用这个宏，编译器又必须在编译过程中先运行它。这产生了一个“先有鸡还是先有蛋”的循环依赖。
//特定的 Crate 类型：定义过程宏的 Crate 必须声明为 crate-type = ["proc-macro"]。这种类型的 Crate 导出的只能是过程宏，不能包含普通的函数或结构体供外部直接使用。

//类函数宏(function-like macros)
use proc_macro_examples::make_answer;
make_answer!();

//派生宏
use proc_macro_examples::AnswerFn;
#[derive(AnswerFn)]
struct _Struct;

//属性宏
/*
use proc_macro_examples::show_streams;
// 示例: 基础函数
#[show_streams]
fn _invoke1() {}
// out: attr: ""
// out: item: "fn _invoke1() { }"

// 示例: 带输入参数的属性
#[show_streams(bar)]
fn _invoke2() {}
// out: attr: "bar"
// out: item: "fn _invoke2() {}"

// 示例: 输入参数中有多个 token 的
#[show_streams(multiple => tokens)]
fn _invoke3() {}
// out: attr: "multiple => tokens"
// out: item: "fn _invoke3() {}"

// 示例:
#[show_streams { delimiters }]
fn _invoke4() {}
// out: attr: "delimiters"
// out: item: "fn _invoke4() {}"

*/
pub fn main() {
    hello!("Rust");
    hello_each!("Rust", "World");
    type N2 = Tuple!(i32, i32);
    let _: N2 = (1, 2);
    if let pat!(x) = Some(1) {
        assert_eq!(x, 1);
    }
    println!("{}", i32::CONST);
    println!("{}", answer());
    assert_eq!(42, derive_answer());
    //_invoke1();
    //_invoke2();
    //_invoke3();
    //_invoke4();
}
