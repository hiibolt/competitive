/**
 * 2894. "Divisible and Non-Divisible Sums Difference"
 * 
 * Difficulty: Easy
 * Tags: Math
 * Runtime: Beats 100%
 */

impl Solution {
    pub fn difference_of_sums(
        n: i32,
        m: i32
    ) -> i32 {
        let are = (n * (n + 1)) / 2;
        let smallest_divisible = n - n % m;
        let not = (smallest_divisible * (smallest_divisible + m )) / (2 * m);

        return are - not * 2;
    }
}