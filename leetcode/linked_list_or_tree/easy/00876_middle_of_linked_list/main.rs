/**
 * 876. "Middle of the Linked List"
 * 
 * Difficulty: Easy
 * Tags: Linked List, Two Pointers
 * Runtime: Beats 100%
 */
impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cursor: Box<ListNode> = head.clone().unwrap();
        let mut length: usize = 1;
        while cursor.next.is_some() {
            cursor = cursor.next.unwrap();
            length += 1;
        }

        cursor = head.unwrap();
        let mut count: usize = 1;
        while count <= length / 2 {
            cursor = cursor.next.unwrap();
            count += 1;
        }
        Some(cursor)
    }
}