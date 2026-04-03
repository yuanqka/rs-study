use std::collections::HashMap;

pub fn get_run_config() -> HashMap<String, bool> {
    let mut config = HashMap::new();
    config.insert("mini_redis".to_string(), true);
    config
}
