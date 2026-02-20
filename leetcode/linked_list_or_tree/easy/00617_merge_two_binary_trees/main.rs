use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let root1_left = root1.as_ref().map(|root1| root1.borrow_mut().left.take()).flatten();
        let root2_left = root2.as_ref().map(|root2| root2.borrow_mut().left.take()).flatten();
        let root1_right = root1.as_ref().map(|root1| root1.borrow_mut().right.take()).flatten();
        let root2_right = root2.as_ref().map(|root2| root2.borrow_mut().right.take()).flatten();

        let mut new_root = match (root1, root2) {
            (Some(root1), Some(root2)) => {
                root1.borrow_mut().val += root2.borrow().val;
                root1
            },
            (Some(root), None) | (None, Some(root)) => { root },
            (None, None) => { return None; }
        };
        new_root.borrow_mut().left = Self::merge_trees(root1_left, root2_left);
        new_root.borrow_mut().right = Self::merge_trees(root1_right, root2_right);

        Some(new_root)
    }
}