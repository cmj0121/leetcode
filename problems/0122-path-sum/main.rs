use std::rc::Rc;
use std::cell::RefCell;

pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    match root {
        Some(n) => {
            let node = n.borrow();

            match (node.left.as_ref(), node.right.as_ref()) {
                (None, None) => target_sum == node.val,
                (Some(left), None) => has_path_sum(Some(left.clone()), target_sum - node.val),
                (None, Some(right)) => has_path_sum(Some(right.clone()), target_sum - node.val),
                (Some(left), Some(right)) => {
                    has_path_sum(Some(left.clone()), target_sum - node.val) || 
                    has_path_sum(Some(right.clone()), target_sum - node.val)
                }
            }
        }
        None => false,
    }
}

fn main() {
}
