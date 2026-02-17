impl Solution {
    pub fn find_max_consecutive_ones(
        nums: Vec<i32>
    ) -> i32 {
        let mut max = 0i32;
        let mut streak = 0i32;
        for num in nums {
            if num != 1 {
                streak = 0;
                continue;
            }

            streak += 1;
            max = max.max(streak);
        }

        max
    }
}