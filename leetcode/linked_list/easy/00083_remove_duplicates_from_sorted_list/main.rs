/**
 * 83. "Remove Duplicates from Sorted List"
 * 
 * Difficulty: Easy
 * Tags: Linked List
 * Runtime: Beats 9%
 */

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cursor = &mut head;
        while let Some(node) = cursor {
            while let Some(next_node) = &mut node.next {
                println!("{} vs {:?}", node.val, next_node.val);
                if node.val == next_node.val {
                    node.next = next_node.next.take();

                    println!("skipping {}", ":3");
                } else {
                    break;
                }
            }

            cursor = &mut node.next;
        }

        let mut cursor = &head;
        println!("Final value: {:?}", head);
        while let Some(node) = cursor {
            println!("{}", node.val);

            cursor = &node.next;
        }

        head
    }
}