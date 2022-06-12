use crate::{ListNode, Solution};

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(node) if node.next.is_some() => {
                let mut next = node.next.unwrap();
                let mut tmp = Box::new(ListNode::new(node.val));

                tmp.next = Solution::swap_pairs(next.next);
                next.next = Some(tmp);

                Some(next)
            }
            Some(node) => Some(node),
            None => None,
        }
    }
}
