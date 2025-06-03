/**
 * 96. "Unique Binary Search Trees"
 * 
 * Difficulty: Medium
 * Tags: Math, Dynamic Programming, Combanitorics
 * Runtime: Beats 100%
 */
use std::collections::HashMap;
impl Solution {
    pub fn calc_coeff ( n: i128, mut k: i128 ) -> i128 {
        let mut ret: i128 = 1;

        if ( k > n - k ) {
            k = n - k;
        }

        for i in 0..k {
            ret *= (n - i);
            ret /= (i + 1);
        }

        ret
    }
    pub fn num_trees(
        n: i32
    ) -> i32 {
        let n = n as i128;

        ( Self::calc_coeff( ( 2 * n ), n ) / ( n + 1 ) ) as i32
    }
}