/**
 * 72. "Edit Distance"
 * 
 * Difficulty: Medium
 * Tags: String, Dynamic Programming
 * Runtime: Beats 8%
 */
use std::collections::HashMap;
fn lev<'a>(
    memo: &mut HashMap<(&'a [u8], &'a [u8]), i32>,
    arr1: &'a [u8],
    arr2: &'a [u8]
) -> i32 {
    if let Some(memo_sol) = memo.get(&(arr1, arr2)) {
        return *memo_sol;
    }
    if arr1.len() == 0 || arr2.len() == 0 {
        return arr1.len().max(arr2.len()) as i32;
    }
    if arr1[0] == arr2[0] {
        return lev(memo, &arr1[1..], &arr2[1..]);
    }
    let lev1 = lev(memo, &arr1[1..], arr2);
    let lev2 = lev(memo, arr1, &arr2[1..]);
    let lev3 = lev(memo, &arr1[1..], &arr2[1..]);

    let ret = 1 + lev1.min(lev2).min(lev3);
    memo.insert((arr1, arr2), ret);
    ret
}
impl Solution {
    pub fn min_distance(
        word1: String,
        word2: String
    ) -> i32 {
        let mut memo = HashMap::new(); 
        lev(&mut memo, word1.as_bytes(), word2.as_bytes())
    }
}