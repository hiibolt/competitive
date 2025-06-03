/**
 * 292. "Nim Game"
 * 
 * Difficulty: Easy
 * Tags: Math, Brain Teaser, Game Theory
 * Runtime: Beats 100%
 */
impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        (n % 4) != 0
    }
}