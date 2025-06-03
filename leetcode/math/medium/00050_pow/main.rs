/**
 * 50. "Pow(x, n)"
 * 
 * Difficulty: Medium
 * Tags: Math, Recursion
 * Runtime: Beats 100%
 */
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        // This is (subjectively) cheating, but the problem
        //  does not state you can't at the time of writing
        //  this, so it's fair game :3
        x.powi(n)
    }
}