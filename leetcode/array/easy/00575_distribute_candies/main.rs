use std::collections::HashSet;
impl Solution {
    pub fn distribute_candies(
        candy_types: Vec<i32>
    ) -> i32 {
        let mut seen = HashSet::new();
        let mut ret = 0;
        let max = (candy_types.len() / 2) as i32;
        for candy in candy_types {
            if seen.insert(candy) {
                ret += 1;
            }
            if ret == max {
                return max;
            }
        }

        ret
    }
}