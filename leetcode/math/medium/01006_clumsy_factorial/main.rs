/**
 * 1006. "Clumsy Factorial"
 * 
 * Difficulty: Medium
 * Tags: Math, Stack, Simulation
 * Runtime: Beats 100%
 */
impl Solution {
    pub fn clumsy( mut n: i32 ) -> i32 {
        let mut ret = 0i32;
        let mut sign = 1;

        while n >= 1 {
            let mult = ( (n  * {
                n -= 1;
                if n > 0 { n } else { 1 }
            }) as f32 / {
                n -= 2;
                if n > 0 { n + 1 } else { 1 }
            } as f32).floor() as i32;
            ret += sign * mult;

            let add = {
                n -= 1;
                if n >= 0 { n + 1 } else { 0 }
            };
            ret += add;
            
            sign = -1;

        }

        ret
    }
}

/*
10 * 9 / 8 + 7 - 6 * 5 / 4 + 3 - 2 * 1
11 + 7 - 7 + 3 - 2

*/