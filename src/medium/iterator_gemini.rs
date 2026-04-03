/*
`into_iter()`: 消耗集合，返回拥有所有权的迭代器 (T)。
`iter()`: 不消耗集合，返回包含不可变引用的迭代器 (&T)。
`iter_mut()`: 不消耗集合，返回包含可变引用的迭代器 (&mut T)。
*/

#[derive(Debug, Clone)]
struct Counter {
    count: u32,
}
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

/*
实现 Iterator trait 是让一个类型“成为”一个迭代器。
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

#[derive(Debug)]
#[allow(dead_code)]
struct Shoe {
    size: u32,
    style: String,
}

struct ShoeCollection {
    shoes: Vec<Shoe>,
}

// 为我们的集合实现 iter() 方法，让它可以提供一个“不可变引用”的迭代器
impl ShoeCollection {
    // 这个方法返回一个实现了 Iterator 的类型，这里是 `std::slice::Iter`
    fn iter(&self) -> std::slice::Iter<'_, Shoe> {
        self.shoes.iter()
    }
}

pub fn main() {
    println!("--- 演示自定义类型的 Iterator trait ---");
    let mut counter = Counter::new();
    // counter 本身就是一个迭代器
    println!("Counter.next(): {:?}", counter.next());
    println!("Counter.next(): {:?}", counter.next());

    println!("\n--- 演示集合的 iter() 方法 ---");
    let collection = ShoeCollection {
        shoes: vec![
            Shoe {
                size: 10,
                style: "sneaker".to_string(),
            },
            Shoe {
                size: 13,
                style: "sandal".to_string(),
            },
            Shoe {
                size: 10,
                style: "boot".to_string(),
            },
        ],
    };

    // 我们调用 .iter() 来获取一个迭代器，而不是直接在 collection 上迭代
    // 这样 collection 就不会被消耗
    for shoe in collection.iter() {
        println!("Found shoe: {:?}", shoe);
    }
    println!(
        "循环结束后，collection 依然可用: {} shoes",
        collection.shoes.len()
    );

    println!("\n--- 演示 Vec 的不同迭代方式 ---");
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
    print!("for循环默认调用 into_iter (对于 Vec 来说会转移所有权): ");
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
    // println!("{:?}", array); // 这里再打印就会编译失败, 因为所有权被 into_iter 移走了

    println!("\n--- 演示迭代器适配器 (Iterator Adapters) ---");
    // `Counter` 实现了 `Iterator`, 所以可以直接使用 `zip`, `map`, `collect` 等方法
    // 这些方法被称为“迭代器适配器”，它们本身也返回新的迭代器
    let sum_vec: Vec<u32> = Counter::new() // 创建一个 Counter 迭代器
        .zip(Counter::new().skip(1)) // 与另一个迭代器配对: (1,2), (2,3), (3,4), (4,5)
        .map(|(a, b)| a * b) // 映射成新的值: 2, 6, 12, 20
        .filter(|x| *x > 10) // 过滤: 12, 20
        .collect(); // 收集成 Vec<u32>

    println!("使用迭代器适配器处理后的结果: {:?}", sum_vec);

    let mut final_counter = Counter::new();
    // .unwrap() 会取出 Some 里的值，如果是 None 则 panic
    println!(
        "最后，Counter 的 next() 返回: {}, 此时 count 变为: {}",
        final_counter.next().unwrap(),
        final_counter.count
    );
}
