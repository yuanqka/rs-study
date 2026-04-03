//pub mod config;
mod advance;
mod easy;
mod medium;
mod tokio;
automod::dir!("src");
//automod仅导入参数目录下的文件模块, 且是私有的
pub use config::get_run_config;
//use medium::mod_filed::main;
//pub use crate::medium::another_mod::another_main;
pub use std::collections::HashMap;
/// 整个应用程序的入口点。
///
/// 读取配置，并根据配置运行特定模块。
pub fn run() {
    //use config::get_run_config;
    let run_config = get_run_config();
    let mut dispatch: HashMap<&'static str, fn()> = HashMap::new();
    dispatch.insert("easy", easy::main);
    dispatch.insert("medium", medium::main);
    dispatch.insert("advance", advance::main);
    dispatch.insert("tokio", tokio::main);

    for (name, should_run) in run_config.iter() {
        if *should_run {
            if let Some(func) = dispatch.get(name.as_str()) {
                func(); // 调用函数指针
            }
        }
    }
}
