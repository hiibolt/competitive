use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>
    ) -> bool {
        let mut q = VecDeque::from([root.unwrap()]);
        while let Some(node) = q.pop_front() {
            if Self::equal(&Some(node.clone()), &sub_root) { return true; }

            if let Some(ref left) = node.borrow().left { q.push_back(left.clone()); }
            if let Some(ref right) = node.borrow().right { q.push_back(right.clone()); }
        }

        false
    }
    fn equal (
        tree_1: &Option<Rc<RefCell<TreeNode>>>,
        tree_2: &Option<Rc<RefCell<TreeNode>>>
    ) -> bool {
        // base cases, unpack values
        if tree_1.is_none() && tree_2.is_none() { return true; }
        let (Some(tree_1), Some(tree_2)) = (tree_1, tree_2) else { return false; };

        tree_1.borrow().val == tree_2.borrow().val &&
            Self::equal( &tree_1.borrow().left, &tree_2.borrow().left ) &&
            Self::equal( &tree_1.borrow().right, &tree_2.borrow().right )
    }
}