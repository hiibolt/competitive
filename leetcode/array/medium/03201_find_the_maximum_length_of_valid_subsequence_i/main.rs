/**
 * 3201. "Find the Maximum Length of Valid Subsequence I"
 * 
 * Difficulty: Medium
 * Tags: Array, Dynamic Programming
 * Runtime: Beats 100%
 */
impl Solution {
    pub fn maximum_length(
        nums: Vec<i32>
    ) -> i32 {
        let mut types: [i32; 3] = [ ( ( nums[0] & 1 ) - 1).abs(), nums[0] & 1, 1 ];
        let mut last = nums[0] & 1;
        for num in nums.into_iter().map(|num| num & 1).skip(1) {
            types[(num & 1) as usize] += 1;

            if num != last {
                types[2] += 1;
                last = (last - 1).abs();
            }
        }

        types.into_iter().max().unwrap_or(2).max(2)
    }
}