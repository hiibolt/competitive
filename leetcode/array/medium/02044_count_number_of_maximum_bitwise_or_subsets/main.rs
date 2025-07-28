/**
 * 2044. "Count Number of Maximum Bitwise OR Subsets"
 * 
 * Difficulty: Medium
 * Tags: Array, Backtracking, Bit Manipulation, Enumebration, Weekly Context 263
 */
impl Solution {
    pub fn count_max_or_subsets (
        nums: Vec<i32>
    ) -> i32 {
        let max_bitwise_or = nums.iter()
            .fold(0i32, |acc, num| {
                acc | num
            });

        // Count the number of subsets that match
        //  this value
        let mut total = 0i32;
        for mask in 0..(1 << nums.len()) {
            let mut or = 0i32;
            for ind in 0..nums.len() {
                if ( mask >> ind ) & 1 == 1 {
                    or |= nums[ind];
                }
            }

            if or == max_bitwise_or {
                total += 1;
            }
        }

        total
    }
}