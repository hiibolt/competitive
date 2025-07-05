/**
 * 1394. "Find Lucky Integer in an Array"
 * 
 * Difficulty: Easy
 * Tags: Array, Hash Table, Counting
 * Runtime: Beats 100%
 */
impl Solution {
    pub fn find_lucky(
        arr: Vec<i32>
    ) -> i32 {
        let mut freqs = vec!(0usize; 500);
        for num in arr {
            freqs[num as usize - 1] += 1;
        }

        for (ind, num) in freqs.into_iter().enumerate().rev() {
            if ind + 1 == num {
                return num as i32
            }
        }

        -1i32
    }
}