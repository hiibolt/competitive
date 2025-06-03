/**
 * 263. "Ugly Number"
 * 
 * Difficulty: Easy
 * Tags: Math
 * Runtime: Beats 100%
 */
impl Solution {
    pub fn is_ugly(mut n: i32) -> bool {
        if n <= 0 {
            return false;
        }

        while n != 1 {
            if n % 2 == 0 {
                n /= 2;
                continue;
            } else if n % 3 == 0 {
                n /= 3;
                continue;
            } else if n % 5 == 0 {
                n /= 5;
                continue
            }

            return false;
        }

        true
    }
}