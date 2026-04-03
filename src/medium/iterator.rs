/*
.into_iter(), 返回的是所有权, 因此用过后再用就寄了
.iter() 方法实现的迭代器，调用 next 方法返回的类型是 Some(&T)
.iter_mut() 方法实现的迭代器，调用 next 方法返回的类型是 Some(&mut T)，因此在 if let Some(v) = values_iter_mut.next() 中，v 的类型是 &mut i32，最终我们可以通过 *v = 0 的方式修改其值
*/

#[derive(Debug)]
struct Counter {
    count: u32,
}
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
    fn clone(&self) -> Counter {
        Counter { count: self.count }
    }
}

/*
Iterator 特征中，不仅仅是只有 next 一个方法，那为什么我们只需要实现它呢？因为其它方法都具有默认实现，所以无需像 next 这样手动去实现，而且这些默认实现的方法其实都是基于 next 方法实现的。
*/
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

pub fn main() {
    let array = vec![1, 2, 3, 4, 5];
    let mut iter = array.iter();
    assert_eq!(iter.next(), Some(&1));
    print!("手动迭代: ");
    loop {
        match iter.next() {
            Some(x) => print!("{} ", x),
            None => break,
        }
    }
    println!();
    print!("for循环自动迭代(但是会转移所有权, 是into_iter): ");
    for element in array.clone() {
        print!("{} ", element);
    }
    println!();
    let mut iter = array.into_iter();
    loop {
        match iter.next() {
            Some(x) => print!("{}", x),
            None => break,
        }
    }
    println!();
    //println!("{:?}", array);
    //这里再打印就panic, 因为没有所有权了. into_iter搞走了

    let counter = Counter { count: 0 }/*.iter()这里是不对的, 只实现了trait, 没有method*/;
    for element in counter
        .clone()
        .zip(Counter::new())
        .map(|(a, b)| a + b) //映射迭代器元素生成新的值
        .collect::<Vec<_>>()
    //收集, 例如我们可以将一个实现了iterator的变成迭代器, map映射变换每一个元素, 再收集, 就无需冗杂的语句了
    {
        //如果没有Iterator trait会panic
        print!("{:?} ", element);
    }

    let mut counter = Counter { count: 0 };
    println!("{} {}", counter.next().unwrap(), counter.count);
}
