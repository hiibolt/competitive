/**
 * 2099. "Find Subsequence of Length K With the Largest Sum"
 * 
 * Difficulty: Easy
 * Tags: Array, Hash Table, Sorting, Heap (Priority Queue)
 * Runtime: Beats 100% (i'm also very proud of this solution ^^)
 */
use std::collections::BinaryHeap;
use std::cmp::{PartialEq, Eq, Ord, Ordering, Reverse};

struct Item {
    val: i32,
    ind: usize
}
impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}
impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}
impl Eq for Item {}

impl Solution {
    pub fn max_subsequence(
        nums: Vec<i32>,
        k: i32
    ) -> Vec<i32> {
        let mut items: Vec<Reverse<Item>> = nums.into_iter().enumerate()
            .map(|(ind, val)| Reverse(Item { val, ind }))
            .collect::<BinaryHeap<Reverse<Item>>>()
            .into_sorted_vec();
        items.drain(k as usize..);
        items.sort_by_key(|Reverse(i)| i.ind);

        items.into_iter()
            .map(|Reverse(i)| i.val)
            .collect()
    }
}