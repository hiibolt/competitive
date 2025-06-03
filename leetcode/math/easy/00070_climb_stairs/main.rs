/**
 * 67. "Climb Stairs"
 * 
 * Difficulty: Easy
 * Tags: Math, Dynamic Programming, Memoization
 * Runtime: Beats 100%
 */
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n < 2 {
            return n;
        }
        let mut ret: i32 = 1;
        let mut one_back: i32 = 1;
        let mut two_back: i32 = 0;
        for _ in 2..=n {
            two_back = one_back;
            one_back = ret;
            ret = one_back + two_back;
        }
        ret
    }
}