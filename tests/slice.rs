#[test]
fn main() {
    let a = vec![1, 2, 3, 4];
    let b = &a[0..4][0];
    let c = &a[0..4][1];
    println!("引用比较{}", b < c);
    let mut strs = vec!["Hello".to_string(), "world".to_string()];
    for it in &mut strs {
        unsafe {
            it.as_bytes_mut()[0] = b'C';
        }
        println!("{}", it);
    }
    //let d = b"nihao";
    strs.push(String::from("你好, 世界"));
    test(&mut strs);
    for it in &mut strs {
        /*unsafe {
            it.as_bytes_mut()[0] = b'C';
        }*///运行错误, 因为有非ascii长字符被修改单一字节
        println!("{}", it);
    }
}

fn test(a: &mut Vec<String>) {
    for it in a {
        println!("{}", it);
    }
}
