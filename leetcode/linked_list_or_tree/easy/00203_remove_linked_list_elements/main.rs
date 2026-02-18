/**
 * 203. "Remove Linked List Elements"
 * 
 * Difficulty: Easy
 * Tags: Linked List, Recursion
 * Runtime: Beats 100%
 */
impl Solution {
    pub fn remove_elements(
        mut head: Option<Box<ListNode>>,
        val: i32
    ) -> Option<Box<ListNode>> {
        while let Some(head_node) = &mut head {
            if head_node.val != val {
                break;
            }
            head = (*head_node).next.take();
        }

        let mut cursor = &mut head;
        while let Some(node_box) = cursor {
            let next_val = if let Some(next_node_box) = &(*node_box).next {
                next_node_box.val.clone()
            } else { break; };

            if next_val == val {
                let mut next_node_cursor = (*node_box).next.take();
                while let Some(next_node) = &mut next_node_cursor {
                    if next_node.val != val {
                        break;
                    }
                    next_node_cursor = (*next_node).next.take();
                }

                (*node_box).next = next_node_cursor;
            }

            cursor = &mut node_box.next;
        }

        head
    }
}