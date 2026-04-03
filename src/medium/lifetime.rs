pub fn main() {
    //{ //报错, 因为悬垂引用, c++中只能运行时出错, rust编译时检测出了
    //let r;
    //{
    //let x = 5;
    //r = &x;
    //}
    //println!("r: {}", r);
    //}

    let a: String = "hello, world!".to_string();
    let b: String = "ciallo, world!".to_string();
    lifetime(&a, &b);
}

fn lifetime<'t>(a: &'t String, b: &'t String) -> &'t String {
    println!("{}", if a.len() > b.len() { a } else { b });
    //let c: String = "ciallo, world!".to_string();
    //c //取消注释报错, 因为生命周期应该是与
    a
}
