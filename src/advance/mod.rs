automod::dir!("src/advance");

use std::collections::HashMap;

pub fn main() {
    let run_config = config::get_run_config();
    let mut dispatch: HashMap<&str, fn()> = HashMap::new();

    dispatch.insert("sync", sync::main);
    dispatch.insert("macro_learn", macro_learn::main);
    println!("\x1b[1;34m高级部分运行开始...");
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

    println!("\x1b[1;32m高级部分运行结束...\x1b[0m");
}
