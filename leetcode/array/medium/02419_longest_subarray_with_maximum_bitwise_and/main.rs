/**
 * 2419. "Longest Subarray With Maximum Bitwise AND"
 * 
 * Difficulty: Medium
 * Tags: Array, Bit Manipulation, Brain Teaser, Weekly Contest 312
 */
impl Solution {
    pub fn longest_subarray(
        nums: Vec<i32>
    ) -> i32 {
        let max_num = *nums.iter().max().expect("Problem lied :<");
        
        let mut max_elements_overall = 0i32;
        let mut max_elements = 0i32;
        for num in nums {
            if num & max_num != max_num {
                max_elements_overall = max_elements_overall.max(max_elements);
                max_elements = 0;
            } else {
                max_elements += 1;
            }
        }

        max_elements_overall.max(max_elements)
    }
}