use std::mem;

// 此函数借用一个 slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

pub fn run() {
    // 定长数组（类型标记是多余的）
    //let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let xs = [2, 4, 6, 8i32, 10i32];

    // 所有元素可以初始化成相同的值
    let ys: [i32; 500] = [0; 500];

    // 下标从 0 开始
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` 返回数组的大小
    println!("array size: {}", xs.len());

    // 数组是在栈中分配的
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // 数组可以自动被借用成为 slice
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // slice 可以指向数组的一部分
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1..4]);

    // 越界的下标会引发致命错误（panic）
    //println!("{}", xs[5]);

    // --- 堆分配的例子 ---
    // `vec!` 宏创建了一个在堆上分配的 `Vec<i32>`
    //rust版的std::vector
    let mut vec_on_heap = vec![10, 20, 30, 40, 50];
    vec_on_heap.push(60);

    // 这个 slice 指向了堆上的数据
    println!("\nborrow a vec on the heap as a slice");
    analyze_slice(&vec_on_heap);
}
