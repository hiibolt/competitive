/**
 * 1695. "Maximum Erasure Value"
 * 
 * Difficulty: Medium
 * Tags: Array, Hash Table, Sliding Window, Weekly Contest 220
 * Runtime: Beats 17%
 */
use std::collections::HashMap;
impl Solution {
    pub fn maximum_unique_subarray( nums: Vec<i32> ) -> i32 {
        let mut seen = HashMap::from([(nums[0], 0)]);

        let mut max_total = nums[0];
        let mut total = nums[0];
        let mut left_ind = 0;
        for (right_ind, &num) in nums.iter().enumerate().skip(1) {
            // If we've seen this before, erase all prior elements
            if seen.contains_key(&num) {
                let delete_until = seen[&num];

                for ind in left_ind..=delete_until {
                    total -= nums[ind];
                    seen.remove(&nums[ind]);
                }

                left_ind = delete_until + 1;
            }

            seen.insert(num, right_ind);
            total += num;
            max_total = max_total.max(total);
        }

        max_total
    }
}