use std::{collections::HashSet, i32};

extern crate rs_study as root;
use root::run;

fn test1(fun: fn(&mut Vec<i32>, &mut Vec<i32>), test1: &mut Vec<i32>, test2: &mut Vec<i32>) {
    fun(test1, test2);
    run();
}

fn fun(test1: &mut Vec<i32>, test2: &mut Vec<i32>) {
    print!("{:?}\n{:?}", test1, test2);
}

#[test]
fn owership() {
    let r: &usize;
    let x = 5usize;
    {
        r = &x;
    }
    print!("{}", r);
    let mut test1: Vec<i32> = Vec::new();
    let mut test2: Vec<i32> = Vec::new();
    fun(&mut test1, &mut test2);
    self::test1(fun, &mut test1, &mut test2);
    let s: String = "hello word".to_string();
    let v = vec![0; 10];
    let mut vv: Vec<i32> = v;
    vv.push(1);
    let ss = s;
    print!("{}{:?}", ss, vv);
    struct _Test {
        one: Box<i32>,
        _two: i32,
    }
    //let strs = "Hello".to_string();
    //let single = strs.chars();
    //print!("{}", single);
    let n = _Test {
        one: Box::new(1),
        _two: 2,
    };
    let m = n;
    print!("{:?}", *(m.one));
    println!("成功了吗?");
    let mut set: Vec<HashSet<i32>> = Vec::new();
    set.push([1, 3].into_iter().collect());
    for it in &set[0] {
        print!("{}", it);
    }
    for it in &set[0] {
        print!("{}", it);
    }
    #[allow(unused_parens)]
    let _aaa = 1..3;
    for _ in _aaa.clone() {}
    let aa = vec![vec![1, 2]; 3];
    let _aaaa = &aa[_aaa];
    let _temp = [1, 2, 3];
    let _temp = [1..3];
    for (_, it) in aa.into_iter().enumerate() {
        let _temp = it[0];
    }
}
