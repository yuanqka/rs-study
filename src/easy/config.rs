//这是一个该项目的模块函数是否运行的文件,定义一些键值对
use std::collections::HashMap;

/// 返回一个 HashMap，其中包含了每个模块是否应该运行的配置。
/// true: 运行
/// false: 跳过
pub fn get_run_config() -> HashMap<String, bool> {
    let mut config = HashMap::new();

    config.insert("hello".to_string(), false);
    config.insert("print".to_string(), false);
    config.insert("display".to_string(), false);
    config.insert("list".to_string(), false);
    config.insert("formatting".to_string(), false);
    config.insert("primitives".to_string(), false);
    config.insert("tuples".to_string(), false);
    config.insert("array".to_string(), false);
    config.insert("structure".to_string(), false);
    config.insert("guess_number".to_string(), false);
    config.insert("data_types".to_string(), false);
    config.insert("contrul".to_string(), false);
    config.insert(String::from("ownership"), false);
    config.insert("slice".to_string(), false);
    config.insert("struct_type".to_string(), false);
    config.insert("test".to_string(), true);

    config
}
