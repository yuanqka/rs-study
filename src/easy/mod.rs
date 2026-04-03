//自动加载目录下文件
automod::dir!("src/easy");

use std::collections::HashMap;

pub fn main() {
    let run_config = config::get_run_config();

    // 创建一个从模块名到其 run 函数的映射
    let mut dispatch: HashMap<&'static str, fn()> = HashMap::new();
    dispatch.insert("hello", hello::run);
    dispatch.insert("print", print::run);
    dispatch.insert("display", display::run);
    dispatch.insert("list", list::run);
    dispatch.insert("formatting", formatting::run);
    dispatch.insert("primitives", primitives::run);
    dispatch.insert("tuples", tuples::run);
    dispatch.insert("array", array::run);
    dispatch.insert("structure", structure::run);
    dispatch.insert("guess_number", guess_number::run);
    dispatch.insert("data_types", data_types::run);
    dispatch.insert("contrul", contrul::run);
    dispatch.insert("ownership", ownership::main);
    dispatch.insert("slice", slice::run);
    dispatch.insert("struct_type", struct_type::run);
    dispatch.insert("test", test::run);

    //println!("\n\x1b[38;2;128;128;200m运行开始...\x1b[0m");
    println!("\x1b[1;34m基础部分运行开始...");
    // 遍历配置，执行需要运行的模块
    for (name, should_run) in run_config.iter() {
        if *should_run {
            if let Some(func) = dispatch.get(name.as_str()) {
                println!(
                    "\x1b[1;33m--- 正在运行模块: {} ---\x1b[38;2;128;128;200m",
                    name
                );
                func(); // 调用函数指针
            }
        }
    }

    println!("\x1b[1;32m基础部分运行结束...\x1b[0m");
}
//field of translation
/*

*/
