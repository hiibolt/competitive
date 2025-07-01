/**
 * 3330. "Find the Original Typed String I"
 * 
 * Difficulty: Easy
 * Tags: String
 * Runtime: Beats 100% (cool math solution making use of block syntax :3)
 */
impl Solution {
    pub fn possible_string_count(
        word: String
    ) -> i32 {
        (word.len() - {
            let mut chars: Vec<char> = word.chars().collect();
            chars.dedup();
            chars.len()
        } + 1) as i32
    }
}