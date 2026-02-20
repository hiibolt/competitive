use std::collections::BinaryHeap;
impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let mut highest_pos_heap = BinaryHeap::new();
        let mut highest_neg_heap = BinaryHeap::new();
        let mut lowest_pos_heap = BinaryHeap::new();
        let mut lowest_neg_heap = BinaryHeap::new();
        for num in nums {
            if num >= 0 {
                highest_pos_heap.push(num);
                lowest_pos_heap.push(-num);
            } else {
                highest_neg_heap.push(-num);
                lowest_neg_heap.push(num);
            }
        }

        if highest_pos_heap.len() == 1 {
            let first = highest_pos_heap.pop().unwrap();

            let highest_neg = highest_neg_heap.pop().unwrap_or(0) * highest_neg_heap.pop().unwrap_or(0);

            first * highest_neg
        } else if highest_pos_heap.len() == 0 {
            let first = lowest_neg_heap.pop().unwrap();
            
            let lowest_neg = lowest_neg_heap.pop().unwrap_or(0) * lowest_neg_heap.pop().unwrap_or(0);

            first * lowest_neg
        } else {
            let first = highest_pos_heap.pop().unwrap();

            let highest_pos = highest_pos_heap.pop().unwrap_or(0) * highest_pos_heap.pop().unwrap_or(0);
            let highest_neg = highest_neg_heap.pop().unwrap_or(0) * highest_neg_heap.pop().unwrap_or(0);
            
            first * highest_pos.max(highest_neg)
        }
    }
}