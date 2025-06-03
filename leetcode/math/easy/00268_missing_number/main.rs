/**
 * 268. "Missing Number"
 * 
 * Difficulty: Easy
 * Tags: Array, Hash Table, Math, Binary Search, Bit Manipulation, Sorting
 * Runtime: Beats 100%
 */
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        (0..nums.len()).fold(0i32, |acc, i| {
            acc + (i + 1) as i32 - nums[i]
        })
    }
}