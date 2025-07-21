/**
 * 1957. "Delete Characters to Make Fancy String"
 * 
 * Difficulty: Easy
 * Tags: String
 * Runtime: Beats 85.34%
 */
impl Solution {
    pub fn make_fancy_string ( s: String ) -> String {
        s.chars().fold(
            (String::new(), ('~', 1)),
            |(mut st, (mut last, mut total)), ch| {
                if ch == last {
                    if total + 1 < 3 {
                        st.push(ch);
                    }
                    (st, (last, total + 1))
                } else {
                    st.push(ch);
                    (st, (ch, 1))
                }
            })
            .0
    }
}