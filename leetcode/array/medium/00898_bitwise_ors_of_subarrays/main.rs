/**
 * 898. "Bitwise ORs of Subarrays"
 * 
 * Difficulty: Medium
 * Tags: Array, DP, Bit Manipulation, Weekly Contest 100
 * Runtime: Beats 100%
 */
use std::collections::HashSet;
impl Solution {
    pub fn subarray_bitwise_o_rs(
        mut arr: Vec<i32>
    ) -> i32 {
        arr.dedup();

        let mut ret: HashSet<i32> = HashSet::new();
        let mut current_ors = HashSet::new();
        for num in arr {
            current_ors = current_ors.into_iter()
                .map(|old_or| old_or | num)
                .chain(std::iter::once(num))
                .collect();
            ret.extend(&current_ors);
        }
        
        ret.len() as i32
    }
}