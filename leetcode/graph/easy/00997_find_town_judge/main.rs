/**
 * 997. "Find the Town Judge"
 * 
 * Difficulty: Easy
 * Tags: Array, Hash Table, Graph
 * Runtime: Beats 58%
 */

use std::collections::HashMap;
impl Solution {
    pub fn find_judge (
        n: i32,
        trust_vecs: Vec<Vec<i32>>
    ) -> i32 {
        if n == 1 {
            return 1;
        }

        let mut adj: HashMap<i32, i32> = (1..=n).map(|n| (n, 0)).collect();
        for trust_vec in trust_vecs {
            adj.remove(&trust_vec[0]);
            if let Some(trusted) = adj.get_mut(&trust_vec[1]) {
                *trusted += 1;
            }
        }

        for (person, trusted_by) in adj {
            if trusted_by == n - 1 {
                return person;
            }
        }

        -1
    }
}