use std::borrow::Borrow;
use std::rc::Rc;

struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
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
impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let result = Solution::middle_node_ref(&head);

        Some(result.as_ref().unwrap().clone())
    }

    pub fn middle_node_ref(head: &Option<Box<ListNode>>) -> &Option<Box<ListNode>> {
        let mut fast = head;
        let mut slow = head;
        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &(slow.as_ref().unwrap().next);
            fast = &(fast.as_ref().unwrap().next.as_ref().unwrap().next);
        }
        slow
    }
}
