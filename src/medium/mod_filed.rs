#[allow(unused)]
pub use crate::medium::another_mod::another_main;
//或者use super::anothor_mod::another_main;

pub mod mod_filed_mod {
    pub use super::another_main;
    fn _test() {
        another_main();
    }
}

pub fn main() {
    //anothor_mod::another_main();错误的, 公开的只有方法, 没有模块
    println!("下面在mod_filed模块执行兄弟模块的方法");
    another_main();
}

//文件结构
//rs_study
//└── src
//    ├── easy/
//    └── medium/
//        ├── another_mod.rs
//        ├── config.rs
//        ├── enumerations.rs
//        ├── generic_data_types.rs
//        ├── iterator_gemini.rs
//        ├── iterator.rs
//        ├── lifetime.rs
//        ├── match_control_flow.rs
//        ├── method.rs
//        ├── mod_filed.rs
//        ├── mod.rs
//        ├── option.rs
//        └── point.rs

//模块树
//crate
//└── medium
//    ├── another_mod
//    ├── config
//    ├── enumerations
//    ├── generic_data_types
//    ├── iterator
//    ├── iterator_gemini
//    ├── lifetime
//    ├── match_control_flow
//    ├── method
//    ├── mod_filed
//    ├── option
//    └── point
