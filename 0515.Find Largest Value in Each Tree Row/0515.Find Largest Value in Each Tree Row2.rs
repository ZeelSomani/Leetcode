
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, depth: usize, res: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }
        let node = root.as_ref().unwrap().borrow();
        if res.len() == depth {
            res.push(node.val);
        } else {
            res[depth] = res[depth].max(node.val);
        }
        Self::dfs(&node.left, depth + 1, res);
        Self::dfs(&node.right, depth + 1, res);
    }

    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        Self::dfs(&root, 0, &mut res);
        res
    }
}
