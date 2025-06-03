/**
 * 83. "Remove Duplicates from Sorted List"
 * 
 * Difficulty: Easy
 * Tags: Linked List
 * Runtime: Beats 100%
 */

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cursor = &mut head;
        while let Some(node) = cursor {
            while let Some(next_node) = &mut node.next {
                if node.val == next_node.val {
                    node.next = next_node.next.take();
                } else {
                    break;
                }
            }

            cursor = &mut node.next;
        }

        let mut cursor = &head;
        while let Some(node) = cursor {
            cursor = &node.next;
        }

        head
    }
}