/**
 * 13. "Roman to Integer"
 * 
 * Difficulty: Easy
 * Tags: Hash Table, Math, String
 * Runtime: Beats 100%
 */
impl Solution {
    fn value(c: char) -> i32 {
        match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => panic!("Invalid character!")
        }
    }
    pub fn roman_to_int(s: String) -> i32 {
        let mut ret: i32 = 0;
        let mut last_value = 0;
        for chr in s.chars().rev() {
            let as_value = Self::value(chr);
            if as_value >= last_value {
                last_value = as_value;
                ret += as_value;
            } else {
                ret -= as_value;
            }
        }
        ret
    }
}