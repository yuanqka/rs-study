//自动加载目录下文件
automod::dir!("src/medium");

use std::collections::HashMap;

pub fn main() {
    let run_config = config::get_run_config();
    // 创建一个从模块名到其 run 函数的映射
    let mut dispatch: HashMap<&'static str, fn()> = HashMap::new();

    dispatch.insert("method", method::main);
    dispatch.insert("enumerations", enumerations::main);
    dispatch.insert("match_control_flow", match_control_flow::main);
    dispatch.insert("iterator", iterator::main);
    dispatch.insert("iterator_gemini", iterator_gemini::main);
    dispatch.insert("option", option::main);
    dispatch.insert("lifetime", lifetime::main);
    dispatch.insert("generic_data_types", generic_data_types::main);
    dispatch.insert("mod_filed", mod_filed::main);
    dispatch.insert("point", point::main);

    println!("\x1b[1;34m进阶部分运行开始...");
    for (name, should_run) in run_config.iter() {
        if *should_run {
            if let Some(func) = dispatch.get(&name[..]) {
                println!(
                    "\x1b[1;33m--- 正在运行模块: {} ---\x1b[38;2;128;128;200m",
                    name
                );
                func(); // 调用函数指针
            }
        }
    }

    println!("\x1b[1;32m进阶部分运行结束...\x1b[0m");
}
