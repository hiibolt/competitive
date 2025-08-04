/**
 * 904. "Fruit Into Baskets"
 * 
 * Difficulty: Medium
 * Tags: Array, Hash Table, Sliding Window, Weekly Contest 102
 */
impl Solution {
    pub fn total_fruit(
        fruits: Vec<i32>
    ) -> i32 {
        if !fruits.iter().any(|&e| e != fruits[0]) { 
            return fruits.len() as i32;
        }
        let mut ret = 0i32;

        let mut left_ind = 0usize;
        'outer: while left_ind < fruits.len() {
            let mut first_diff_ind = left_ind + 1;

            // Get our two characters
            let first: i32 = fruits[left_ind];
            let second: i32 = loop {
                if first_diff_ind >= fruits.len() {
                    break 'outer;
                }
                if fruits[first_diff_ind] != first {
                    break fruits[first_diff_ind];
                }
                first_diff_ind += 1;
            };

            // Find the first character that doesn't match either
            let mut right_ind = left_ind;
            while right_ind < fruits.len() && 
                (fruits[right_ind] == first || fruits[right_ind] == second)
            {
                right_ind += 1;
            }
            ret = ret.max((right_ind - left_ind) as i32);

            left_ind = first_diff_ind;
        }

        ret
    }
}