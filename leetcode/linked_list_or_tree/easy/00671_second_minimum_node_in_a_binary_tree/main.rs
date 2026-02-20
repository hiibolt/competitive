use std::collections::BinaryHeap;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn find_second_minimum_value(
        root: Option<Rc<RefCell<TreeNode>>>
    ) -> i32 {
        let mut lowest = -1;
        let mut second = -1;

        Self::check(&root, &mut lowest, &mut second);

        second
    }
    fn check(
        root: &Option<Rc<RefCell<TreeNode>>>,
        lowest: &mut i32,
        second: &mut i32
    ) {
        let Some(root) = root else { return; };
        
        let val = root.borrow().val;
        if *lowest == -1 || val < *lowest {
            *second = *lowest;
            *lowest = val;
        } else if val != *lowest && (*second == -1 || val < *second) {
            *second = val;
        } 

        Self::check(&root.borrow().left, lowest, second);
        Self::check(&root.borrow().right, lowest, second);
    }
}