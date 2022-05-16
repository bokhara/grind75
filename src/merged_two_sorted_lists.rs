
pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
   match(list1,list2) {
       (Some(v1),None) => Some(v1),
       (None,Some(v2)) => Some(v2),
       (Some(mut v1),Some(mut v2)) => {
           if v1.val < v2.val {
               let n = v1.next.take();
               v1.next = merge_two_lists(n,Some(v2));
               Some(v1)
           } else {
               let n = v2.next.take();
               v2.next = merge_two_lists(Some(v1), n);
               Some(v2)
           }
       }
       _ => None
   }
}
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_two_sorted_list() {
        assert_eq!(merge_two_lists(None,None),None);
    }
}
