/**
 * 62. "Unique Paths"
 * 
 * Difficulty: Medium
 * Tags: Math, Dynamic Programming, Combanitorics
 * Runtime: Beats 100%
 */
fn fact ( n: i32 ) -> i32 {
    if n <= 1 {
        1
    } else {
        n * fact( n - 1 )
    }
}
impl Solution {
    pub fn unique_paths(
        m: i32,
        n: i32
    ) -> i32 {
        if ( m == 1 || n == 1 ) { return 1 };
        let m = m as i128 - 1;
        let n = n as i128 - 1;

        let mut ret: i128 = 1;
        let mut nums: Vec<i128> = ((n + 1)..=(m + n)).collect();
        let mut ms: Vec<i128> = (2..=m).collect();
        while nums.len() + ms.len() > 0 {
            //println!("new cycle");
            let mut i = 0;
            while i < nums.len() {
                if let Some(new_ret) = ret.checked_mul(nums[i]) {
                    //println!("multiplying {ret}*{}", nums[i]);
                    ret = new_ret;
                    nums.remove(i);
                    continue;
                }

                i += 1;
            }
            for i in (0..ms.len()).rev() {
                if ret % ms[i] != 0 {
                    continue;
                }

                //println!("dividing {ret}/{}", ms[i]);
                ret /= ms[i];
                ms.remove(i);
            }
        }

        println!("m: {m} - n: {n} - num: {ret}");
        
        ret as i32
    }
}