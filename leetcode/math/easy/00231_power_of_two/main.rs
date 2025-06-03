/**
 * 231. "Power of Two"
 * 
 * Difficulty: Easy
 * Tags: Math, Bit Manipulation, Recursion
 * Runtime: Beats 100%
 */
impl Solution {
    pub fn is_power_of_two(mut n: i32) -> bool {
        if n <= 0 {
            return false;
        }

        while n != 1 {
            if n % 2 != 0 {
                return false;
            }

            n /= 2;
        }

        true
    }
}