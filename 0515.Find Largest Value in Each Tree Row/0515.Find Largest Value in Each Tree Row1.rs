
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut q = VecDeque::new();
        if root.is_some() {
            q.push_back(root.clone());
        }
        while !q.is_empty() {
            let mut x = i32::MIN;
            for _ in 0..q.len() {
                let node = q.pop_front().unwrap();
                let node = node.as_ref().unwrap().borrow();
                x = x.max(node.val);
                if node.left.is_some() {
                    q.push_back(node.left.clone());
                }
                if node.right.is_some() {
                    q.push_back(node.right.clone());
                }
            }
            ans.push(x);
        }
        ans
    }
}