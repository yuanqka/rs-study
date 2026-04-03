pub fn main() {
    let mut strs: Vec<String> = vec!["hello".to_string(), ",".to_string()];
    strs.push(" world".to_string());
    println!("{}", bigest(strs.clone()).unwrap());
    println!("{}", find_max(&strs).unwrap());
}

fn bigest<T>(a: T) -> Option<T::Item>
where
    //T: std::ops::Index,
    T: IntoIterator,
    <T as IntoIterator>::Item: PartialOrd,
{
    let mut a = a.into_iter();
    //let init_val = a[0];
    //a.iter().fold(init_val, |max, i| if max > i { max } else { i })
    //上面两条语句都不对, iterator trait没有提供iter方法, 至于索引还不清楚
    let mut result = a.next()?;
    for it in a {
        result = if result > it { result } else { it };
    }
    Some(result)
}

fn find_max<'a, T>(collection: &'a T) -> Option<<&'a T as IntoIterator>::Item>
where
    &'a T: IntoIterator,                       // 要求 T 的引用可以转化成迭代器
    <&'a T as IntoIterator>::Item: PartialOrd, // 迭代出来的元素需要能比较
{
    let mut max = None;

    for item in collection.into_iter() {
        match max {
            None => max = Some(item),
            Some(current_max) if item > current_max => max = Some(item),
            _ => {}
        }
    }
    max
}

fn _clone(c: impl Clone) -> Option<impl Clone> {
    let _ = Some(c.clone());
    Some("hello, world".to_string())
}

fn __clone<T>(c: T) -> Option<T>
where
    T: Clone,
{
    Some(c.clone())
    //和_clone函数做对比, _clone要求返回的实现了copy trait就可以, 但是不能推导出要和参数类型一致
    //这就体现了where的作用, 就类似函数与闭包, where适合复杂要求, 但世界用impl trait表示泛型trait就像是闭包一样方便, 适合简单要求
    //下面再次返回一个字符串就不对了, 因为虽然和泛型T一样都实现了copy trait, 但是不能确定是T类型
    //Some("hello, world".to_string())
}
