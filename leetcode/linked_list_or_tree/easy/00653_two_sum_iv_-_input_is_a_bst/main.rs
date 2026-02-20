use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn find_target(
        root: Option<Rc<RefCell<TreeNode>>>,
        k: i32
    ) -> bool {
        let mut hm = HashMap::new();
        Self::build_hm(&root, &mut hm);

        for (&num, count) in &hm {
            let needed_num = k - num;
            if needed_num == num {
                if *count >= 2 {
                    return true;
                }
            } else if hm.contains_key(&needed_num) {
                return true;
            }
        }

        false
    }
    fn build_hm(
        root: &Option<Rc<RefCell<TreeNode>>>,
        hm: &mut HashMap<i32, i32>
    ) {
        let Some(root) = root else { return };
        *(hm.entry(root.borrow().val).or_insert(0i32)) += 1;

        Self::build_hm(&root.borrow().left, hm);
        Self::build_hm(&root.borrow().right, hm);
    }
}