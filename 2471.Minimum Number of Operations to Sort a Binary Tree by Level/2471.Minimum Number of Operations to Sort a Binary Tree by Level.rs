
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::new();
        queue.push_back(root);
        let mut ans = 0;
        while !queue.is_empty() {
            let n = queue.len();
            let mut row = Vec::new();
            for _ in 0..n {
                let mut node = queue.pop_front().unwrap();
                let mut node = node.as_mut().unwrap().borrow_mut();
                row.push(node.val);
                if node.left.is_some() {
                    queue.push_back(node.left.take());
                }
                if node.right.is_some() {
                    queue.push_back(node.right.take());
                }
            }
            for i in 0..n - 1 {
                let mut min_idx = i;
                for j in i + 1..n {
                    if row[j] < row[min_idx] {
                        min_idx = j;
                    }
                }
                if i != min_idx {
                    row.swap(i, min_idx);
                    ans += 1;
                }
            }
        }
        ans
    }
}
