/**
 * 15. "3Sum"
 * 
 * Difficulty: Medium
 * Tags: Array, Two Pointers, Sorting
 * Runtime: Beats 23%
 */

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn three_sum(
        mut nums: Vec<i32>
    ) -> Vec<Vec<i32>> {
        let mut nums: HashMap<i32, usize> = nums.into_iter()
            .fold(HashMap::new(), |mut hm, val| {
                *(hm.entry(val).or_insert(0)) += 1;
                hm
            });
        let mut ret = HashSet::new();

        for key in nums.clone().keys() {
            if nums[key] == 1 {
                nums.remove(key);
            } else {
                *nums.get_mut(key).unwrap() -= 1;
            }

            for mut new_vec in Self::two_sum(&nums, -key) {
                new_vec.insert(0, *key);
                new_vec.sort();
                ret.insert(new_vec);
            }

            *(nums.entry(*key).or_insert(0)) += 1;
        }

        ret.into_iter().collect::<Vec<Vec<i32>>>()
    }
    fn two_sum(
        nums: &HashMap<i32, usize>,
        target: i32
    ) -> Vec<Vec<i32>> {
        let mut ret = vec!();

        for (key, value) in nums.iter() {
            if target == 2 * key {
                if *value >= 2 {
                    ret.push(vec!(*key, *key));
                }
                continue;
            }
            if nums.contains_key(&(target - key)) {
                ret.push(vec!(*key, target - key));
            }
        }

        ret
    }
}