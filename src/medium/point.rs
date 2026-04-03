use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

//在rust中没有null, 但是rust的特色之一就是大量的枚举类型
//所以可以将指针包装在option枚举中, 用None表示空指针
//在比较的特质Ord中也是用枚举类型标识偏序关系
pub fn main() {
    //RefCell相当于在rust严格的所有权规则下开了个后门
    //使得可以修改不可变引用的数据
    let root = Rc::new(RefCell::new(TreeNode::new(10)));
    let left = Rc::new(RefCell::new(TreeNode::new(5)));
    let right = Rc::new(RefCell::new(TreeNode::new(-3)));
    root.borrow_mut().left = Some(left.clone());
    root.borrow_mut().right = Some(right.clone());
    let left_left = Rc::new(RefCell::new(TreeNode::new(3)));
    let left_right = Rc::new(RefCell::new(TreeNode::new(2)));
    let right_left = Rc::new(RefCell::new(TreeNode::new(11)));
    left.borrow_mut().left = Some(left_left.clone());
    left.borrow_mut().right = Some(left_right.clone());
    right.borrow_mut().left = Some(right_left.clone());
    println!("{}", _Solution::path_sum(Some(root), 8));
}

//下面是力扣437的解, 直接拿来了
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct _Solution {}
impl _Solution {
    fn dfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        target_sum: i64,
        mut sum: i64,
        map: &mut HashMap<i64, i32>,
    ) -> i32 {
        if root.is_none() {
            return 0;
        }
        let root = root.unwrap();
        sum += root.borrow().val as i64;
        let mut result = *(map.get(&(sum - target_sum)).unwrap_or(&0));
        *(map.entry(sum).or_insert(0)) += 1;
        result += Self::dfs(root.borrow().left.clone(), target_sum, sum, map);
        result += Self::dfs(root.borrow().right.clone(), target_sum, sum, map);
        if let Some(ele) = map.get_mut(&sum) {
            *ele -= 1;
        }
        result
    }
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut map: HashMap<i64, i32> = HashMap::from([(0, 1)]);
        Self::dfs(root, target_sum as i64, 0, &mut map)
    }
}

#[allow(dead_code)]
struct Node {
    key: i32,
    val: i32,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

#[allow(dead_code)]
impl Node {
    fn new(key: i32, val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            key,
            val,
            left: None,
            right: None,
        }))
    }
}

#[allow(dead_code)]
struct LRUCache {
    capacity: usize,
    dummy: Rc<RefCell<Node>>,
    map: HashMap<i32, Rc<RefCell<Node>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl LRUCache {
    fn new(capacity: i32) -> Self {
        let dummy = Node::new(0, 0);
        // 创建循环引用，dummy 既是头也是尾
        dummy.borrow_mut().left = Some(dummy.clone());
        dummy.borrow_mut().right = Some(dummy.clone());

        LRUCache {
            capacity: capacity as usize,
            dummy,
            map: HashMap::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.map.get(&key) {
            let node = node.clone(); // 克隆 Rc 指针
            self.remove(node.clone());
            self.insert(node.clone());
            return node.borrow().val;
        }
        -1
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.map.get(&key) {
            let node = node.clone();
            node.borrow_mut().val = value;
            self.remove(node.clone());
            self.insert(node);
        } else {
            let node = Node::new(key, value);
            self.map.insert(key, node.clone());
            self.insert(node.clone());
            if self.map.len() > self.capacity {
                // 移除链表尾部（dummy.left）
                let back = self.dummy.borrow().left.as_ref().unwrap().clone();
                self.map.remove(&back.borrow().key);
                self.remove(back);
            }
        }
    }

    // 辅助函数：从链表中移除节点
    fn remove(&self, node: Rc<RefCell<Node>>) {
        let left = node.borrow().left.as_ref().unwrap().clone();
        let right = node.borrow().right.as_ref().unwrap().clone();
        left.borrow_mut().right = Some(right.clone());
        right.borrow_mut().left = Some(left);
    }

    // 辅助函数：插入节点到头部
    fn insert(&self, node: Rc<RefCell<Node>>) {
        let right = self.dummy.borrow().right.as_ref().unwrap().clone();
        node.borrow_mut().left = Some(self.dummy.clone());
        node.borrow_mut().right = Some(right.clone());
        self.dummy.borrow_mut().right = Some(node.clone());
        right.borrow_mut().left = Some(node);
    }
}
