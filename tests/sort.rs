use std::collections::BinaryHeap;

#[test]
fn test() {
    let mut vector: Vec<i32> = vec![1, 2, 3, 4, 3, 2, 1];
    let mut vector2: Vec<i32> = vec![5, 1, 9, 3, 7, 4, 8, 6, 2];
    let mut vector3: Vec<i32> = vec![1, 1, 1, 1, 1, 3, 1, 4, 2, 8];
    println!("快速排序前: {:?}", vector);
    if !vector.is_empty() {
        let len = vector.len() - 1; //左闭右闭区间
        quick_sort(&mut vector, 0, len);
    }
    println!("快速排序后: {:?}", vector);
    assert_eq!(vector, vec![1, 1, 2, 2, 3, 3, 4]);

    println!("堆排序前: {:?}", vector2);
    if !vector2.is_empty() {
        heap_sort(&mut vector2);
    }
    println!("堆排序后: {:?}", vector2);
    assert_eq!(vector2, (1..=9).collect::<Vec<i32>>());

    println!("原地堆排序前: {:?}", vector3);
    if !vector.is_empty() {
        heap_sort_local(&mut vector3);
    }
    println!("原地堆排序后: {:?}", vector3);
    //assert_eq!(vector, vec![1, 1, 2, 2, 3, 3, 4]);
}

fn quick_sort(vector: &mut Vec<i32>, left: usize, right: usize) {
    if left >= right {
        return;
    }
    let pivot = vector[left];
    let mut i = left;
    let mut j = right;
    while i < j {
        while i < j && vector[j] >= pivot {
            j -= 1;
        }
        while i < j && vector[i] <= pivot {
            i += 1;
        }
        vector.swap(i, j);
    }
    vector.swap(left, i);
    if i > 0 {
        quick_sort(vector, left, i - 1);
    }
    quick_sort(vector, i + 1, right);
}

fn heap_sort(vector: &mut Vec<i32>) {
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
    vector.iter().for_each(|&x| heap.push(x));
    let mut flag = heap.len() as i32 - 1;
    while flag >= 0 {
        if let Some(temp) = heap.pop() {
            vector[flag as usize] = temp;
        }
        flag -= 1;
    }
}

#[allow(unused)]
fn heap_sort_local(vector: &mut Vec<i32>) {
    fn sink(vector: &mut Vec<i32>, node: usize, len: usize) {
        let mut i = node;
        let node = vector[node];
        /*待填坑
        while i + i + 1 < len {
            i += i + 1;
            if i + i == len && vector[i + i] > vector[i] {
                vector.swap(i, i + i);
                i += i;
                break;
            }
            if vector[i] < vector[i + i] && vector[i + i] > vector[i * i + 1] {
                vector.swap(i, i + i);
                i += i;
            }
            if vector[i] < vector[i + i + 1] && vector[i + i] <= vector[i * i + 1] {
                vector.swap(i, i + i + 1);
                i += i + 1;
            }
        }
        */
    }
    let len = vector.len();
    for i in (0..vector.len() / 2).rev() {
        sink(vector, i, len);
    }
    for flag in 0..len {
        vector.swap(0, flag);
        sink(vector, 0, flag);
    }
}
