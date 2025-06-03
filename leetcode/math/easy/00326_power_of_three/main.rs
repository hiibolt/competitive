/**
 * 326. "Power of Three"
 * 
 * Difficulty: Easy
 * Tags: Math, Recursion
 * Runtime: Beats 100%
 */
impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        
        let log_3_n = n.ilog(3i32);

        (n - 3i32.pow(log_3_n)) == 0
    }
}
