/**
 * 3439. "Reschudule Meetings for Maximum Free Time IX
 *
 * Difficulty: Medium
 * Tags: Array, Greedy, Sliding Window
 * Runtime: Beats 100%
 */
impl Solution {
    pub fn max_free_time(
        event_time: i32,
        k: i32,
        starts: Vec<i32>,
        ends: Vec<i32>
    ) -> i32 {
        let mut gaps: Vec<i32> = Vec::new();
        
        // Calculate all gaps, including start/end
        gaps.push(starts[0]);
        for (start, end) in starts
            .iter()
            .skip(1)
            .zip(ends.iter())
        {
            gaps.push(start - end);
        }
        gaps.push(event_time - ends[ends.len() - 1]);

        // Find the most effective pair
        let k = k as usize;
        let mut total = 0i32;
        for i in 0..=k {
            total += gaps[i];
        }
        let mut max = total;
        for i in (k + 1)..gaps.len() {
            total -= gaps[i - k - 1];
            total += gaps[i];
            max = max.max(total);
        }

        max
    }
}
