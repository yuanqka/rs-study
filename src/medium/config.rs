use std::collections::HashMap;

pub fn get_run_config() -> HashMap<String, bool> {
    let mut config = HashMap::new();
    config.insert(String::from("method"), false);
    config.insert(String::from("enumerations"), false);
    config.insert(String::from("match_control_flow"), false);
    config.insert(String::from("iterator"), false);
    config.insert(String::from("iterator_gemini"), false);
    config.insert(String::from("option"), false);
    config.insert("lifetime".to_string(), false);
    config.insert("generic_data_types".to_string(), false);
    config.insert("mod_filed".to_string(), false);
    config.insert("point".to_string(), true);
    config
}
