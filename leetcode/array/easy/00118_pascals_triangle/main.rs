/**
 * 118. "Pascal's Triangle"
 * 
 * Difficulty: Easy
 * Tags: Array, Dynamic Programming
 * Runtime: Beats 100%
 */
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let num_rows = num_rows as usize;
        let mut rows = vec!( vec!(1), vec!(1, 1) );
        if num_rows == 1 { return vec!(vec!(1)); } else if num_rows == 2 { return rows; }

        for row_ind in 2..num_rows {
            let mut row = vec!(1);
            for ind in 0..row_ind - 1 {
                let last_row = &rows[row_ind - 1];
                row.push(last_row[ind] + last_row[ind + 1]);
            }
            row.push(1);

            rows.push(row);
        }

        rows
    }
}