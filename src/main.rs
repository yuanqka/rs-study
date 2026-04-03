use std::collections::HashMap;

use rs_study::get_run_config;

#[allow(unused)]
fn main() {
    //use rs_study::config::get_run_config;
    //因为lib库将get_run_config函数引入作用域并声明为公有的, 所以不需要再次引入
    //且再次引入也不对, 因为公共作用域只有该函数, 没有模块config
    // 调用库 crate 中的 run 函数
    rs_study::run();
    let _config = get_run_config();
    let _map: HashMap<i32, i32> = HashMap::new();
}
#[cfg(test)]
mod tests {
    // 注意：现在我们从库 crate 的根部导入
    use rs_study::run;

    #[test]
    fn test() {
        run();
    }
}
