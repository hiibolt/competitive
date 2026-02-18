/**
 * 21. "Merge Two Sorted Lists"
 * 
 * Difficulty: Easy
 * Tags: Linked List, Recursion
 * Runtime: Beats 5%
 */
impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut list1_head = list1.clone();
        let mut list2_head = list2.clone();
        let mut ret: Vec<i32> = Vec::new();
        while list1_head.is_some() || list2_head.is_some() {
            if let Some(node1) = list1_head.as_ref() {
                if let Some(node2) = list2_head.as_ref() {
                    if &node1.val > &node2.val {
                        ret.push(node1.val);
                        list1_head = node1.next.clone();
                    } else {
                        ret.push(node2.val);
                        list2_head = node2.next.clone();
                    }
                } else {
                    ret.push(node1.val);
                    list1_head = node1.next.clone();
                }
            } else {
                ret.push(list2_head.clone().unwrap().val);
                list2_head = list2_head.unwrap().next.clone();
            }
        }
        ret.sort();
        let mut ret_head: Option<Box<ListNode>> = None;
        for &element in ret.iter().rev() {
            ret_head = Some(Box::new(ListNode { val: element, next: ret_head } ));
        }
        ret_head
    }
}