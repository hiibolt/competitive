/**
 * 1717. "Maximum Score From Removing Substrings"
 * 
 * Difficulty: Medium
 * Tags: String, Stack, Greedy, Biweekly Contest 43
 */
impl Solution {
    fn handle_val (
        st: &mut Vec<char>,
        made_change: &mut bool,
        val: i32,
        left_char: char,
        right_char: char
    ) -> i32 {
        let mut score = 0i32;

        let mut ind = st.len() - 2;
        while ind >= 0 && st.len() > 1 {
            if ind < st.len() - 1 && 
                st[ind] == left_char && st[ind + 1] == right_char
            {
                st.remove(ind);
                st.remove(ind);
                score += val;
                *made_change = true;
            }
            ind = if ind != 0 { ind.saturating_sub(1) } else { break; };
        }

        score
    }
    pub fn maximum_gain ( 
        s: String,
        x: i32,
        y: i32
    ) -> i32 {
        let mut st: Vec<char> = s.chars().collect();
        
        let mut made_change = true;
        let mut score = 0i32;
        while made_change {
            made_change = false;
            if x > y {
                score += Self::handle_val(
                    &mut st, &mut made_change,
                    x, 'a', 'b');
                score += Self::handle_val(
                    &mut st, &mut made_change,
                    y, 'b', 'a');
            } else {
                score += Self::handle_val(
                    &mut st, &mut made_change,
                    y, 'b', 'a');
                score += Self::handle_val(
                    &mut st, &mut made_change,
                    x, 'a', 'b');
            }
        }
        score
    }
}