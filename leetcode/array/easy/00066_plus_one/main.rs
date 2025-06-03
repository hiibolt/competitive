/**
 * 66. "Plus One"
 * 
 * Difficulty: Easy
 * Tags: Array, Math
 * Runtime: Beats 100%
 */
impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut ind = digits.len() - 1;
        while let Some(9) = digits.get(ind) {
            digits[ind] = 0;
            if ind > 0 {
                ind -= 1;
            } else {
                digits.insert(0, 0);
                break;
            }
        }
        digits[ind] += 1;
        digits
    }
}