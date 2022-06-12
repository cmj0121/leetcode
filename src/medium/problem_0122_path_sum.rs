use crate::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            Some(n) => {
                let node = n.borrow();

                match (node.left.as_ref(), node.right.as_ref()) {
                    (None, None) => target_sum == node.val,
                    (Some(left), None) => {
                        Solution::has_path_sum(Some(left.clone()), target_sum - node.val)
                    }
                    (None, Some(right)) => {
                        Solution::has_path_sum(Some(right.clone()), target_sum - node.val)
                    }
                    (Some(left), Some(right)) => {
                        Solution::has_path_sum(Some(left.clone()), target_sum - node.val)
                            || Solution::has_path_sum(Some(right.clone()), target_sum - node.val)
                    }
                }
            }
            None => false,
        }
    }
}
