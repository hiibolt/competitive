impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        format!("{num:b}")
            .chars()
            .rev()
            .enumerate()
            .fold(0i32, |acc, (ind, val)| {
                if val == '0' {
                    acc + 2i32.pow(ind as u32)
                } else {
                    acc
                }
            })
    }
}