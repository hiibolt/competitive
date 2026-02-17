impl Solution {
    pub fn find_disappeared_numbers(
        mut nums: Vec<i32>
    ) -> Vec<i32> {
        let range_max = nums.len();
        nums.sort();
        nums.dedup();

        let mut missing_added = 0usize;
        for i in 1..=range_max {
            if nums.get(missing_added) == Some(&(i as i32)) {
                nums.remove(missing_added);
            } else {
                nums.insert(missing_added, i as i32);
                missing_added += 1;
            }
        }

        nums
    }
}