use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    inner_is_valid_bst(root, None, None)
}

fn inner_is_valid_bst(
    root: Option<Rc<RefCell<TreeNode>>>,
    min: Option<i32>,
    max: Option<i32>,
) -> bool {
    match root {
        Some(n) => {
            let node = n.borrow();

            match (node.left.as_ref(), node.right.as_ref()) {
                (None, None) => true,
                (Some(left), None) if left.borrow().val < node.val => match min {
                    Some(val) if left.borrow().val <= val => false,
                    _ => inner_is_valid_bst(Some(left.clone()), min, Some(node.val)),
                },
                (None, Some(right)) if right.borrow().val > node.val => match max {
                    Some(val) if right.borrow().val >= val => false,
                    _ => inner_is_valid_bst(Some(right.clone()), Some(node.val), max),
                },
                (Some(left), Some(right))
                    if left.borrow().val < node.val && right.borrow().val > node.val =>
                {
                    let x = match min {
                        Some(val) if left.borrow().val <= val => false,
                        _ => inner_is_valid_bst(Some(left.clone()), min, Some(node.val)),
                    };
                    let y = match max {
                        Some(val) if right.borrow().val >= val => false,
                        _ => inner_is_valid_bst(Some(right.clone()), Some(node.val), max),
                    };

                    x && y
                }
                _ => false,
            }
        }
        None => true,
    }
}

fn main() {}
