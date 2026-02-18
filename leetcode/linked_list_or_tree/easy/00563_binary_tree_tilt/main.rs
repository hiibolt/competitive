// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    fn sum(
        root: &Option<Rc<RefCell<TreeNode>>>
    ) -> i32 {
        let Some(root) = root else { return 0; };

        root.borrow().val + Self::sum(&root.borrow().left) + Self::sum(&root.borrow().right)
    }
    pub fn find_tilt(
        root: Option<Rc<RefCell<TreeNode>>>
    ) -> i32 {
        let root = if let Some(root) = root {
            if root.borrow().left.is_none() && root.borrow().right.is_none() {
                return 0;
            } else { root }
        } else { return 0; };

        let left_opt = root.borrow_mut().left.take();
        let right_opt = root.borrow_mut().right.take();
        
        let mut tilt = {
            let left_val = Self::sum(&left_opt);
            let right_val = Self::sum(&right_opt);
            (left_val - right_val).abs()
        };
        tilt += Self::find_tilt(left_opt);
        tilt += Self::find_tilt(right_opt);

        tilt
    }
}