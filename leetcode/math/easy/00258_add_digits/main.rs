/**
 * 68. "Add Digits"
 * 
 * Difficulty: Easy
 * Tags: 
 */
impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        if num > 9 { 
            match num % 9 {
                0 => 9,
                other => other
            }
        } else { num }
    }
}