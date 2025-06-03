/**
 * 342. "Power of Four"
 * 
 * Difficulty: Easy
 * Tags: Math, Recursion
 * Runtime: Beats 100%
 */
impl Solution {
    pub fn is_power_of_four(
        n: i32
    ) -> bool {
        if n <= 0 {
            return false;
        }

        let log_4_n = n.ilog(4);

        (n - 4i32.pow(log_4_n)) == 0
    }
}