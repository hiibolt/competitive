/**
 * 206. "Reverse Linked List"
 * 
 * Difficulty: Easy
 * Tags: Linked List, Recursion
 * Runtime: Beats 100%
 */
impl Solution {
    pub fn reverse_list(
        mut head: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        let mut reversed: Option<Box<ListNode>> = None;

        let mut cursor = head;
        while let Some(mut node) = cursor {
            // Takes `node.next` and moves it to `cursor`!
            cursor = node.next.take();

            // Takes `reversed` and moves it to `node.next`!
            node.next = reversed.take();

            // Takes `node` and moves to reversed!
            reversed = Some(node);
        }

        reversed
    }
}