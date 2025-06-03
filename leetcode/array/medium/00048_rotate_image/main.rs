/**
 * 48. "Rotate Image"
 * 
 * Difficulty: Medium
 * Tags: Array, Math, Matrix
 */
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        *matrix = (*matrix)
            .iter()
            .enumerate()
            .map(|(ind, _)| matrix.iter().rev().map(|row| row[ind]).collect() )
            .collect::<Vec<Vec<i32>>>();
    }
}