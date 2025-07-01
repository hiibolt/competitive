/**
 * 189. "Rotate Array"
 * 
 * Difficulty: Medium
 * Tags: Array, Math, Two Pointers
 * Runtime: Beats 5% (not the best solution, but it works with no extra space)
 */
impl Solution {
    pub fn rotate(
        nums: &mut Vec<i32>,
        k: i32
    ) {
        for _ in 0..(k as usize % nums.len()) {
            for ind in 1..nums.len() {
                nums.swap(0, ind);
            }
        }
    }
}