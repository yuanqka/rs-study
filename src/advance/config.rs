use std::collections::HashMap;

pub fn get_run_config() -> HashMap<String, bool> {
    let mut config = HashMap::new();
    config.insert(String::from("sync"), false);
    config.insert("macro_learn".to_string(), true);
    config
}
