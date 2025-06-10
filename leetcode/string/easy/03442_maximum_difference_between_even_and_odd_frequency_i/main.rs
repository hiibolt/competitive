/**
 * 3442. "Maximum Difference Between Even and Odd Frequency"
 * 
 * Difficulty: Easy
 * Tags: Hash Table, String, Counting
 * Runtime: Beats 100%
 */
impl Solution {
    pub fn max_difference(
        s: String
    ) -> i32 {
        let mut freqs = vec!(0; 26);

        for ch in s.chars() {
            let ind = (ch as usize) - 97usize;
            freqs[ind] += 1;
        }

        let mut max_odd = -1;
        let mut min_even = i32::MAX;

        for freq in freqs {
            if freq == 0 {
                continue;
            }
            if freq % 2 != 0 {
                max_odd = max_odd.max(freq);
            } else {
                min_even = min_even.min(freq);
            }
        }

        max_odd - min_even
    }
}