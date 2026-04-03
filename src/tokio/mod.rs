use std::collections::HashMap;

automod::dir!("src/tokio");

pub fn main() {
    let run_config = config::get_run_config();
    let mut dispatch: std::collections::HashMap<String, fn()> = HashMap::new();
    dispatch.insert("mini_redis".to_string(), mini_redis::main);
    println!("\x1b[1;34mTokio部分运行开始...");
    for (name, &should_run) in run_config.iter() {
        if should_run {
            if let Some(func) = dispatch.get(name) {
                println!(
                    "\x1b[1;33m--- 正在运行模块: {} ---\x1b[38;2;128;128;200m",
                    name
                );
                func(); // 调用函数指针
            }
        }
    }
}
