/**
 * 12. "Integer to Roman"
 * 
 * Difficulty: Medium
 * Tags: Hash Table, Math, String
 * Runtime: Beats 100%
 */
impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let mut ret = String::new();
        while num > 0 {
        if num >= 1000 {
            num -= 1000;
            ret.push('M');
        } else if num >= 900 {
            num -= 900;
            ret += "CM";
        } else if num >= 500 {
            num -= 500;
            ret.push('D');
        } else if num >= 400 {
            num -= 400;
            ret += "CD";
        } else if num >= 100 {
            num -= 100;
            ret.push('C');
        } else if num >= 90 {
            num -= 90;
            ret += "XC";
        } else if num >= 50 {
            num -= 50;
            ret.push('L');
        } else if num >= 40 {
            num -= 40;
            ret += "XL";
        } else if num >= 10 {
            num -= 10;
            ret.push('X');
        } else if num >= 9 {
            num -= 9;
            ret += "IX";
        } else if num >= 5 {
            num -= 5;
            ret.push('V');
        } else if num >= 4 {
            num -= 4;
            ret += "IV";
        } else {
            num -= 1;
            ret.push('I');
        }
        }
        ret
    }
}