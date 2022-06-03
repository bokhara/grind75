use crate::invert_tree::TreeNode;
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

struct Solution {}
struct DiameterResult {
    left: i32,
    max: i32,
    right: i32,
}
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let result = Solution::diamete_helper(root);
        return result.max;
    }
    pub fn diamete_helper(root: Option<Rc<RefCell<TreeNode>>>) -> DiameterResult {
        match root {
            Some(head) => {
                let left = Solution::diamete_helper(head.borrow().left.clone());
                let right = Solution::diamete_helper(head.borrow().right.clone());
                let left_max = max(left.left, left.right) + 1;
                let right_max = max(right.left, right.right) + 1;
                let max = max(max(left_max + right_max + 1, left.max), right.max);
                return DiameterResult {
                    left: left_max,
                    max,
                    right: right_max,
                };
            }
            None => {
                return DiameterResult {
                    left: 0,
                    max: 0,
                    right: 0,
                }
            }
        }
    }
}
