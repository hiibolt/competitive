/**
 * 2200. "Find All K-Distant Indices in an Array"
 * 
 * Difficulty: Easy
 * Tags: Array, Two Pointers
 * Runtime: Beats 100%
 */
impl Solution {
    pub fn find_k_distant_indices(
        nums: Vec<i32>,
        key: i32,
        k:   i32
    ) -> Vec<i32> {
        let mut ret = Vec::new();
        let len = nums.len() as i32;
        
        let mut lower_bound = 0;
        for (ind, val) in nums.into_iter().enumerate() {
            if val != key {
                continue;
            }

            let ind = ind as i32;
            let left = (ind - k).max(lower_bound);
            let right = (ind + k + 1).min(len);
            for i in left..right {
                ret.push(i);
            }
            lower_bound = ind + k + 1;
        }

        ret
    }
}