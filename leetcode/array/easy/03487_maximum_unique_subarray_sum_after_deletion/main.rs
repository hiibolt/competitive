/**
 * 3487. "Maximum Unique Subarray Sum After Deletion"
 * 
 * Difficulty: Easy
 * Tags: Array, Hash Table, Greedy, Weekly Contest 441
 * Runtime: Beats 100%
 */
use std::collections::HashSet;
impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let max = *nums.iter().max().unwrap_or(&0);
        if max <= 0 {
            return max;
        }
        nums.into_iter()
            .fold(HashSet::new(), |mut acc, num| {
                if num > 0 {
                    acc.insert(num);
                }
                acc
            })
            .into_iter()
            .sum::<i32>()
    }
}