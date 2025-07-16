/**
 * 98. "Validate Binary Search Tree"
 * 
 * Difficulty: Medium
 * Tags: Tree, Depth-first Search, Binary Search Tree
 * Runtime: Beats 100%
 */
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_valid_bst( root: Option<Rc<RefCell<TreeNode>>> ) -> bool {
        Self::is_valid_bst_helper(root).0
    }
    fn is_valid_bst_helper (
        root: Option<Rc<RefCell<TreeNode>>> 
    ) -> (bool, i32, i32) {
        let root_ptr = root.expect("Problem lied about non-null tree");
        let root_node = Rc::into_inner(root_ptr).expect("Cyclic tree")).into_inner();
        let mut min_val = root_node.val;
        let mut max_val = root_node.val;

        if let Some(left_ptr) = root_node.left {
            let left_val = left_ptr.borrow().val;
            let (
                valid_subtree,
                left_min_val, left_max_val
            ) = Self::is_valid_bst_helper(Some(left_ptr));

            min_val = min_val.min(left_min_val);
            max_val = max_val.max(left_max_val);

            if left_val >= root_node.val || 
                !valid_subtree ||
                left_max_val >= root_node.val
            {
                return ( false, min_val, max_val );
            }
        }
        if let Some(right_ptr) = root_node.right {
            let right_val = right_ptr.borrow().val;
            let (
                valid_subtree,
                right_min_val, right_max_val
            ) = Self::is_valid_bst_helper(Some(right_ptr));

            min_val = min_val.min(right_min_val);
            max_val = max_val.max(right_max_val);

            if right_val <= root_node.val || 
                !valid_subtree || 
                right_min_val <= root_node.val
            {
                return ( false, min_val, max_val );
            }
        }

        return ( true, min_val, max_val );
    }
}