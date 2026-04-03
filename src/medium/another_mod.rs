use crate::medium::mod_filed;

pub fn another_main() {
    println!("这是anoter_mod的公共方法");
    if false {
        //不要改成true, 否则无限递归了
        crate::medium::mod_filed::another_main();
        super::mod_filed::mod_filed_mod::another_main();
        mod_filed::another_main();
    }
}
