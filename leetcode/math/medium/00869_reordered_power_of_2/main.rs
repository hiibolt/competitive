/**
 * 869. "Reordered Power of 2"
 * 
 * Difficulty: Medium
 * Tags: Hash Table, Math, Sorting, Counting, Enumeration, Weekly Contest 93
 * Runtime: Beats 100%
 */
use std::collections::{BTreeMap, HashSet};
fn to_digits ( num: i32 ) -> BTreeMap<char, i32> {
    num.to_string()
        .chars()
        .fold(BTreeMap::new(), |mut acc, digit| {
            (*acc.entry(digit).or_insert(0)) += 1;
            acc
        })
}
impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let mut num_sets = HashSet::new();
        for i in 0..(i32::MAX.ilog2()) {
            let num = 2i32.pow(i);
            num_sets.insert(to_digits(num));
        }
        num_sets.contains(&to_digits(n))
    }
}