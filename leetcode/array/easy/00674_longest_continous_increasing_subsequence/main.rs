impl Solution {
    pub fn find_length_of_lcis(
        nums: Vec<i32>
    ) -> i32 {
        let mut last = nums[0];
        let mut count = 1;
        let mut max_count = 1;
        for num in nums.into_iter().skip(1) {
            if num > last {
                count += 1;
                max_count = max_count.max(count);
            } else {
                count = 1;
            }
            last = num;
        }

        max_count
    }
}