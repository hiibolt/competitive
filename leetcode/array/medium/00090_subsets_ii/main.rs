/**
 * 90. "Subsets II"
 * 
 * Difficulty: Medium
 * Tags: Array, Backtracking, Bit Manipulation
 * Runtime: Beats 100%
 */
use std::collections::{HashSet, BTreeMap};
impl Solution {
    pub fn subsets_with_dup(
        mut nums: Vec<i32>
    ) -> Vec<Vec<i32>> {
        let mut ret = HashSet::new();

        for mask in 0..(1 << nums.len()) {
            let mut subset = BTreeMap::new();

            for ind in 0..nums.len() {
                if (mask >> ind) & 1 == 1 {
                    *(subset.entry(nums[ind]).or_insert(0)) += 1;
                }
            }

            ret.insert(subset);
        }

        ret.into_iter()
            .map(|hm| {
                hm.into_iter()
                    .flat_map(|(num, count)| vec!(num; count))
                    .collect::<Vec<i32>>()
            })
            .collect()
    }
}