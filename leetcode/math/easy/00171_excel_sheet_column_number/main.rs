/**
 * 171. "Excel Sheet Column Number"
 * 
 * Difficulty: Easy
 * Tags: Math, String
 * Runtime: Beats 100%
 */
impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        column_title
            .chars()
            .enumerate()
            .fold(0i32, |acc: i32, (ind, ch) | {
                acc + (26i32.pow(column_title.len() as u32 - ind as u32 - 1u32) * ((ch as i32) - 64i32))
            })
    }
}