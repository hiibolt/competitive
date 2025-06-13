/**
 * 2616. "Minimum Difference in Sums After Removal of Elements"
 * 
 * Difficulty: Medium
 * Tags: Array, Binary Search, Greedy
 * Runtime: Beats 10%
 */
fn can_form_pairs (
    nums: &Vec<i32>,
    target: i32,
    mut p: i32
) -> bool {
    let mut ind = 0;

    while ind < nums.len() - 1 && p > 0 {
        if nums[ind + 1] - nums[ind] <= target {
            ind += 2;
            p -= 1;
        } else {
            ind += 1;
        }
    }

    p == 0
}
impl Solution {
    pub fn minimize_max(
        mut nums: Vec<i32>,
        p: i32
    ) -> i32 {
        nums.sort();
        
        // Aim to find the best difference between all 
        //  possible differences
        let mut left_cursor = 0;
        let mut right_cursor = nums[nums.len() - 1] - nums[0];
        while left_cursor < right_cursor {
            let middle = (left_cursor + right_cursor) / 2;
            
            if can_form_pairs(&nums, middle, p) {
                right_cursor = middle;
            } else {
                left_cursor = middle + 1;
            }
        }

        right_cursor
    }
}