use std::borrow::Borrow;
use std::rc::Rc;

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
struct Solution {}
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut vals: Vec<i32> = vec![];

        let mut cur = head.as_ref();
        while cur.is_some() {
            vals.push(cur.unwrap().val);
            cur = cur.unwrap().next.as_ref();
        }
        vals.reverse();
        Solution::build_linked_list(&vals, 0)
    }
    pub fn build_linked_list(vals: &Vec<i32>, i: usize) -> Option<Box<ListNode>> {
        if i >= vals.len() {
            return None;
        }
        let mut result = Box::new(ListNode::new(vals[i]));
        result.next = Solution::build_linked_list(vals, i + 1);
        Some(result)
    }

    pub fn reverse_list2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Solution::reverse_list_helper(head, None)
    }
    pub fn reverse_list_helper(
        mut head: Option<Box<ListNode>>,
        prev: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if let Some(mut node) = head {
            head = node.next.take();
            node.next = prev;
            return Solution::reverse_list_helper(head, Some(node));
        }
        prev
    }
}
