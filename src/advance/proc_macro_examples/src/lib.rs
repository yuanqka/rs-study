#![crate_type = "proc-macro"]
extern crate proc_macro;
use proc_macro::TokenStream;

//过程宏
#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

//派生宏
//下面是派生宏的一个示例。它没有对输入执行任何有用的操作，只是追加了一个函数 answer。
#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
    "fn derive_answer() -> u32 { 42 }".parse().unwrap()
}

//属性宏
#[proc_macro_attribute]
pub fn return_as_is(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    item
}

//过程宏本质上是一个编译器插件。为了运行宏，编译器必须先将其代码编译成机器码，然后动态加载到编译器中执行。
//编译顺序矛盾：编译器需要先完成整个 Crate 的编译才能得到可执行的宏。但如果在当前 Crate 中调用这个宏，编译器又必须在编译过程中先运行它。这产生了一个“先有鸡还是先有蛋”的循环依赖。
//特定的 Crate 类型：定义过程宏的 Crate 必须声明为 crate-type = ["proc-macro"]。这种类型的 Crate 导出的只能是过程宏，不能包含普通的函数或结构体供外部直接使用。
