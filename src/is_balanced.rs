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
pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let balanced_result = is_balanced_tree(root);
    return balanced_result.is_balanced;
}

fn is_balanced_tree(root: Option<Rc<RefCell<TreeNode>>>) -> NodeResult {
    match root {
        Some(r) => {
            let left = is_balanced_tree(r.borrow().left.clone());
            if !left.is_balanced {
                return NodeResult {
                    height: 0,
                    is_balanced: false,
                };
            }
            let right = is_balanced_tree(r.borrow().right.clone());
            return NodeResult {
                height: left.height.max(right.height) + 1,
                is_balanced: left.is_balanced
                    && right.is_balanced
                    && ((left.height - right.height).abs() < 2),
            };
        }
        None => NodeResult {
            height: 0,
            is_balanced: true,
        },
    }
}

struct NodeResult {
    height: i32,
    is_balanced: bool,
}
