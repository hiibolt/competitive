/**
 * 9. "Palindrome Number"
 * 
 * Difficulty: Easy
 * Tags: Math
 * Runtime: Beats 100%
 */
impl Solution {
    pub fn is_palindrome(
        x: i32
    ) -> bool {
        let x_st = x.to_string();

        for (c1, c2) in x_st.chars().zip(x_st.chars().rev()) {
            if c1 != c2 {
                return false;
            }
        }

        true
    }
}