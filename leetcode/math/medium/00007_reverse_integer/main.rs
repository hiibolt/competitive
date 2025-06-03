/**
 * 7. "Reverse Integer"
 * 
 * Difficulty: Medium
 * Tags: Math
 * Runtime: Beats 100%
 */
impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let sign = x.signum();
        x = x.abs();
        
        let mut ret: i32 = 0;
        for (ind, chr) in format!("{}", x).chars().enumerate() {
            if let Some(mult) = 10i32.checked_pow(ind as u32)
                .and_then(|res| (chr.to_digit(10u32)? as i32).checked_mul(res))
            {
                if let Some(add) = ret.checked_add(mult) {
                    ret = add;
                } else {
                    return 0;
                }
            } else {
                return 0;
            }
        }
        ret * sign
    }
}