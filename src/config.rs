use std::collections::HashMap;

pub fn get_run_config() -> HashMap<String, bool> {
    let mut config = HashMap::new();
    config.insert("easy".to_string(), false);
    config.insert(String::from("medium"), false);
    config.insert(String::from("advance"), false);
    config.insert(String::from("tokio"), true);
    config
}
