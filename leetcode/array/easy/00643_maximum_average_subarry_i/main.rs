impl Solution {
    pub fn find_max_average(
        nums: Vec<i32>,
        k: i32
    ) -> f64 {
        let k = k as usize;
        let mut total = 0i32;
        for i in 0..k {
            total += nums[i];
        }

        let mut max_total = total;
        for i in k..nums.len() {
            total -= nums[i - k];
            total += nums[i];
            max_total = max_total.max(total);   
        }
        max_total as f64 / k as f64
    }
}