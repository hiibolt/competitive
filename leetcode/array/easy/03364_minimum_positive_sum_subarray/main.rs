/**
 * 3364. "Minimum Positive Sum Subarray"
 * 
 * Difficulty: Easy
 * Tags: Array, Sliding Window, Prefix Sum, Weekly Contest 425
 * Runtime: Beats 100%
 */
impl Solution {
    pub fn minimum_sum_subarray(
        nums: Vec<i32>, l: i32, r: i32
    ) -> i32 {
        let mut prefix_sum = vec![0; nums.len() + 1];
        for (ind, num) in nums.into_iter().enumerate() {
            prefix_sum[ind + 1] = if ind > 0 {
                num + prefix_sum[ind]
            } else { num };
        }

        let mut minimum: Option<i32> = None;
        for left_ind in 0..prefix_sum.len() - 1 {
            for right_ind in left_ind..prefix_sum.len() {
                let total = prefix_sum[right_ind] - prefix_sum[left_ind];
                let width = (right_ind - left_ind) as i32;
                if total <= 0 || width < l || width > r {
                    continue;
                }

                minimum = minimum.map_or(
                    Some(total),
                    |current_min| {
                        Some(current_min.min(total))
                    }
                );
            }
        }

        minimum.unwrap_or(-1i32)
    }
}