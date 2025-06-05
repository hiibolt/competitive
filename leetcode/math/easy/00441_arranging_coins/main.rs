/**
 * 441. "Arranging Coins"
 * 
 * Difficulty: Easy
 * Tags: Math, Binary Search
 * Runtime: Beats 100%
 */
impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let mut last_row:   i64 = 1;
        let mut built_rows: i64 = 0;
        let n = n as i64;
        
        loop {
            if n > built_rows && n <= built_rows + last_row {
                return ((last_row - 1) + ((n - built_rows) % last_row == 0) as i64) as i32;
            }

            built_rows += last_row;
            last_row += 1i64; 
        }
    }
}