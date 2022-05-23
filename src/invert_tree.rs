use std::cell::RefCell;
use std::rc::Rc;

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
  match root {
    Some(v) => {
      let left = v.borrow().left.clone();
      let right = v.borrow().right.clone();
      v.borrow_mut().left = invert_tree(right);
      v.borrow_mut().right = invert_tree(left);
      Some(v)
    }
    None => root,
  }
}

pub fn invert_tree2(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
  match root {
    Some(v) => {
      let mut inverted_node = TreeNode::new(v.borrow().val);
      match (v.borrow().left.as_ref(), v.borrow().right.as_ref()) {
        (None, None) => (),
        (Some(left_node), None) => {
          inverted_node.right = invert_tree2(Some(Rc::clone(&left_node)));
        }
        (None, Some(right_node)) => {
          inverted_node.left = invert_tree2(Some(Rc::clone(&right_node)));
        }
        (Some(left_node), Some(right_node)) => {
          inverted_node.right = invert_tree2(Some(Rc::clone(&left_node)));
          inverted_node.left = invert_tree2(Some(Rc::clone(&right_node)));
        }
      };
      return Some(Rc::new(RefCell::new(inverted_node)));
    }
    None => root,
  }
}

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
