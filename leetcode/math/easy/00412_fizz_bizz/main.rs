/**
 * 380. "Fizz Buzz"
 * 
 * Difficulty: Easy
 * Tags: Math, String, Simulation
 * Runtime: Beats 100%
 */
impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..=n).map(|x| {
            let mut ret = String::new();
            if x % 3 == 0 {
                ret += "Fizz";
            }
            if x % 5 == 0{
                ret += "Buzz";
            }
            if ret.len() == 0 {
                ret += &x.to_string();
            }
            ret
        }).collect()
    }
}