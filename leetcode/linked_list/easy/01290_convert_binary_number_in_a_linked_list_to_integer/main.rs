/**
 * 1290. "Convert Binary Number in a Linked List to Integer"
 *
 * Difficulty: Easy
 * Tags: Linked List, Math
 * Runtime: Beats 100%
 */
impl Solution {
    pub fn get_decimal_value(
        head: Option<Box<ListNode>>
    ) -> i32 {
        let mut num = 0i32;
        let mut bit_num = 0i32;

        let mut cursor = &head;
        while let Some(ref node) = cursor {
            num = num << 1;
            num = num | node.val;

            bit_num += 1;
            cursor = &((*node).next);
        }

        num
    }
}
