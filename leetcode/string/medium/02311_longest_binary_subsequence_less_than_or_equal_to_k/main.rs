/**
 * 2311. "Longest Binary Subsequence Less Than or Equal to K"
 * 
 * Difficulty: Medium
 * Tags: String, Dynamic Programming, Greedy, Memoization
 * Runtime: Beats ?% (achieved 4ms, 84% got 0, 11 got 11ms)
 */
fn binary_string_greater_than_k (
    st: &String,
    k: i32
) -> bool {
    let mut total = 0i32;
    for (ind, val) in st.chars().enumerate() {
        if val == '0' { continue; }
        if let Some(val) = 2i32.checked_pow((st.len() - ind - 1) as u32) {
            if let Some(val) = total.checked_add(val) {
                total = val;

                if total <= k {
                    continue;
                }
            }
        }

        return true;
    }

    false
}
impl Solution {
    pub fn longest_subsequence(
        mut s: String, k: i32
    ) -> i32 {
        'outer: while binary_string_greater_than_k(&s, k) {
            for (ind, val) in s.chars().enumerate() {
                if val == '1' {
                    s.remove(ind);
                    continue 'outer;
                }
            }
        }

        s.len() as i32
    }
}