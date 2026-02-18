/**
 * 234. "Palindrome Linked List"
 * 
 * Difficulty: Easy
 * Tags: Linked List, Two Pointers, Recursion
 * Runtime: Beats 95%
 */
impl Solution {
    pub fn is_palindrome(
        mut head: Option<Box<ListNode>>
    ) -> bool {
        let mut values = Vec::new();

        while let Some(node_box) = &mut head {
            values.push((*node_box).val);
            head = (*node_box).next.take();
        }
        
        for (i1, i2) in values.iter().zip(values.iter().rev()) {
            if i1 != i2 {
                return false;
            }
        }

        true
    }
}