// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;
pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    return match root {
        Some(v) => match (p, q) {
            (Some(pNode), Some(qNode)) => {
                let mut min = pNode.borrow().val;
                let mut max = qNode.borrow().val;
                if pNode.borrow().val > qNode.borrow().val {
                    min = qNode.borrow().val;
                    max = pNode.borrow().val;
                }

                if min < v.borrow().val && v.borrow().val < max {
                    Some(v)
                } else if max < v.borrow().val {
                    lowest_common_ancestor(v.borrow().left.clone(), Some(pNode), Some(qNode))
                } else {
                    lowest_common_ancestor(v.borrow().right.clone(), Some(pNode), Some(qNode))
                }
            }
            _ => Option::None,
        },
        None => Option::None,
    };
}
