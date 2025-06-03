/**
 * 231. "Power of Two"
 * 
 * Difficulty: Easy
 * Tags: Math, Bit Manipulation, Recursion
 * Runtime: Beats 100%
 */
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && n.count_ones() == 1
    }
}