impl Solution {
    pub fn find_error_nums(
        mut nums: Vec<i32>
    ) -> Vec<i32> {
        nums.sort();
        let old_sum = nums.iter().map(|&n| n).sum::<i32>();
        nums.dedup();
        let new_sum = nums.iter().map(|&n| n).sum::<i32>();
        let expected_sum = (1..=(nums.len() + 1) as i32).sum::<i32>();

        vec!(old_sum - new_sum, expected_sum - new_sum)
    }
}