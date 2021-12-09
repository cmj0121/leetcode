pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match head {
        Some(node) if node.next.is_some() => {
            let mut next = node.next.unwrap();
            let mut tmp = Box::new(ListNode::new(node.val));

            tmp.next = swap_pairs(next.next);
            next.next = Some(tmp);

            Some(next)
        }
        Some(node) => Some(node),
        None => None,
    }
}

fn main() {}
