/**
 * 202. "Happy Number"
 * 
 * Difficulty: Easy
 * Tags: Hash Table, Math, Two Pointers
 * Runtime: Beats 100%
 */
impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        let mut num_cycles: i32 = 0;
        while n != 1 && num_cycles < 100 {
            n = n.to_string().chars().map(|ch| ch.to_digit(10).unwrap().pow(2) as i32).sum::<i32>();
            num_cycles += 1;
        }
        n == 1
    }
}