/**
 * 3304. "Find the Kth Character in String Game I"
 * 
 * Difficulty: Easy
 * Tags: Math, Bit Manipulation, Recursion, Simulation, (I think String too)
 * Runtime: Beats 100%
 */
impl Solution {
    pub fn kth_character(
        k: i32
    ) -> char {
        let mut st = String::from("a");
        while st.len() < k as usize {
            st += &st.chars().map(|ch| {
                if ch == 'z' {
                    'a'
                } else {
                    (ch as u8 + 1) as char
                }
            }).collect::<String>();
        }
        st.chars().nth(k as usize - 1).unwrap()
    }
}